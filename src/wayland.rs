//! Wayland layer-shell: barra con 3 zone di testo (left, center, right). Nessun input.

use std::os::unix::io::AsFd;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use calloop::generic::Generic;
use calloop::timer::{TimeoutAction, Timer};
use calloop::{EventLoop, Interest, Mode, PostAction};
use rustix::io::dup;
use tempfile::tempfile;
use wayland_client::{
    delegate_noop,
    protocol::{wl_buffer, wl_compositor, wl_output, wl_registry, wl_shm, wl_shm_pool, wl_surface},
    Connection, Dispatch, EventQueue, QueueHandle,
};
use wayland_client::globals::{registry_queue_init, GlobalListContents};
use wayland_protocols_wlr::layer_shell::v1::client::{
    zwlr_layer_shell_v1, zwlr_layer_surface_v1,
};

use crate::config;
use crate::draw;

/// Le 3 stringhe da disegnare (sinistra, centro, destra).
fn bar_text() -> (String, String, String) {
    (
        crate::modules::layout::left(),
        crate::modules::layout::center(),
        crate::modules::layout::right(),
    )
}

/// Stato Wayland.
pub struct AppState {
    pub width: u32,
    pub height: u32,
    pub configured: bool,
    pub layer_surface: Option<zwlr_layer_surface_v1::ZwlrLayerSurfaceV1>,
    pub surface: Option<wl_surface::WlSurface>,
    pub redraw_pending: Arc<AtomicBool>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            configured: false,
            layer_surface: None,
            surface: None,
            redraw_pending: Arc::new(AtomicBool::new(false)),
        }
    }
}

// Dispatch per WlRegistry (richiesto da registry_queue_init)
impl Dispatch<wl_registry::WlRegistry, GlobalListContents> for AppState {
    fn event(
        _state: &mut Self,
        _proxy: &wl_registry::WlRegistry,
        _event: wl_registry::Event,
        _data: &GlobalListContents,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<wl_compositor::WlCompositor, ()> for AppState {
    fn event(_: &mut Self, _: &wl_compositor::WlCompositor, _: wl_compositor::Event, _: &(), _: &Connection, _: &QueueHandle<Self>) {}
}

impl Dispatch<wl_surface::WlSurface, ()> for AppState {
    fn event(_: &mut Self, _: &wl_surface::WlSurface, _: wl_surface::Event, _: &(), _: &Connection, _: &QueueHandle<Self>) {}
}

impl Dispatch<wl_shm::WlShm, ()> for AppState {
    fn event(_: &mut Self, _: &wl_shm::WlShm, _: wl_shm::Event, _: &(), _: &Connection, _: &QueueHandle<Self>) {}
}

delegate_noop!(AppState: ignore wl_shm_pool::WlShmPool);
delegate_noop!(AppState: ignore wl_buffer::WlBuffer);

impl Dispatch<zwlr_layer_shell_v1::ZwlrLayerShellV1, ()> for AppState {
    fn event(_: &mut Self, _: &zwlr_layer_shell_v1::ZwlrLayerShellV1, _: zwlr_layer_shell_v1::Event, _: &(), _: &Connection, _: &QueueHandle<Self>) {}
}

// Dispatch per layer surface: evento configure
impl Dispatch<zwlr_layer_surface_v1::ZwlrLayerSurfaceV1, ()> for AppState {
    fn event(
        state: &mut Self,
        proxy: &zwlr_layer_surface_v1::ZwlrLayerSurfaceV1,
        event: zwlr_layer_surface_v1::Event,
        _data: &(),
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
        match event {
            zwlr_layer_surface_v1::Event::Configure { serial, width, height } => {
                state.width = width;
                state.height = height;
                state.configured = true;
                proxy.ack_configure(serial);
                state.redraw_pending.store(true, Ordering::SeqCst);
            }
            zwlr_layer_surface_v1::Event::Closed => {}
            _ => {}
        }
    }
}

/// Crea la surface layer e la configura; avvia il loop.
pub fn run() -> Result<(), String> {
    let conn = Connection::connect_to_env().map_err(|e| e.to_string())?;
    let (globals, mut queue) =
        registry_queue_init::<AppState>(&conn).map_err(|e| format!("registry init: {}", e))?;

    let state = Arc::new(Mutex::new(AppState::default()));

    // Bind globals (tipo esplicito per layer_shell)
    let compositor: wl_compositor::WlCompositor = globals
        .bind(&queue.handle(), 4..=5, ())
        .map_err(|e| format!("compositor: {}", e))?;
    let shm: wl_shm::WlShm = globals
        .bind(&queue.handle(), 1..=1, ())
        .map_err(|e| format!("shm: {}", e))?;

    let layer_shell: zwlr_layer_shell_v1::ZwlrLayerShellV1 = globals
        .bind(&queue.handle(), 1..=5, ())
        .map_err(|e| format!("layer_shell (compositor wlr?): {}", e))?;

    let surface = compositor.create_surface(&queue.handle(), ());
    let layer_surface = layer_shell.get_layer_surface(
        &surface,
        None::<&wl_output::WlOutput>,
        zwlr_layer_shell_v1::Layer::Top,
        config::LAYER_NAMESPACE.to_string(),
        &queue.handle(),
        (),
    );

    layer_surface.set_size(0, config::BAR_HEIGHT); // 0 = full width
    layer_surface.set_anchor(
        zwlr_layer_surface_v1::Anchor::Top
            | zwlr_layer_surface_v1::Anchor::Left
            | zwlr_layer_surface_v1::Anchor::Right,
    );
    layer_surface.set_exclusive_zone(config::BAR_HEIGHT as i32);
    surface.commit();

    {
        let mut s = state.lock().unwrap();
        s.layer_surface = Some(layer_surface);
        s.surface = Some(surface.clone());
    }

    // Roundtrip: invia commit e attendi configure
    queue.roundtrip(&mut *state.lock().unwrap()).map_err(|e| e.to_string())?;

    // Loop fino al primo configure (il compositor può mandarlo in un evento successivo)
    for _ in 0..100 {
        if state.lock().unwrap().configured {
            break;
        }
        let _ = conn.flush();
        queue.blocking_dispatch(&mut *state.lock().unwrap()).map_err(|e| e.to_string())?;
    }

    let (width, height) = {
        let s = state.lock().unwrap();
        (s.width, s.height)
    };
    if width == 0 || height == 0 {
        return Err("compositor gave zero size".to_string());
    }

    let font = draw::load_font()?;
    let size = (width * height * 4) as usize;
    let mut pixel_buf = vec![0u8; size];
    let (left, center, right) = bar_text();
    draw::draw_bar(&mut pixel_buf, width, height, &font, &left, &center, &right);

    let mut shm_file = tempfile().map_err(|e| format!("tempfile: {}", e))?;
    shm_file.set_len(size as u64).map_err(|e| e.to_string())?;
    draw::write_buf_to_file(&mut shm_file, &pixel_buf)?;
    let pool = shm.create_pool(shm_file.as_fd(), size as i32, &queue.handle(), ());
    let buffer = pool.create_buffer(
        0,
        width as i32,
        height as i32,
        (width * 4) as i32,
        wl_shm::Format::Xrgb8888,
        &queue.handle(),
        (),
    );

    let surface = state.lock().unwrap().surface.clone().unwrap();
    surface.attach(Some(&buffer), 0, 0);
    surface.commit();
    conn.flush().map_err(|e| e.to_string())?;

    let timer_interval = Duration::from_millis(500); // aggiornamento barra ogni 0.5 s

    struct LoopData {
        conn: Connection,
        queue: EventQueue<AppState>,
        state: Arc<Mutex<AppState>>,
        surface: wl_surface::WlSurface,
        font: fontdue::Font,
        shm_file: std::fs::File,
        _pool: wl_shm_pool::WlShmPool,
        buffer: wl_buffer::WlBuffer,
        pixel_buf: Vec<u8>,
        width: u32,
        height: u32,
    }

    let mut loop_data = LoopData {
        conn,
        queue,
        state,
        surface,
        font,
        shm_file: shm_file,
        _pool: pool,
        buffer,
        pixel_buf,
        width,
        height,
    };

    let mut event_loop =
        EventLoop::<LoopData>::try_new().map_err(|e| e.to_string())?;
    let handle = event_loop.handle();

    let wl_fd = dup(loop_data.queue.as_fd()).map_err(|e| format!("dup wayland fd: {}", e))?;
    handle
        .insert_source(
            Generic::new(wl_fd, Interest::READ, Mode::Level),
            |_, _, data: &mut LoopData| {
                if let Some(guard) = data.conn.prepare_read() {
                    let _ = guard.read();
                }
                let _ = data.conn.flush();
                let mut s = data.state.lock().unwrap();
                let _ = data.queue.dispatch_pending(&mut *s);
                Ok(PostAction::Continue)
            },
        )
        .map_err(|e| format!("wayland source: {}", e))?;

    handle
        .insert_source(
            Timer::from_duration(timer_interval),
            |_, _, data: &mut LoopData| {
                let (left, center, right) = bar_text();
                draw::draw_bar(
                    &mut data.pixel_buf,
                    data.width,
                    data.height,
                    &data.font,
                    &left,
                    &center,
                    &right,
                );
                let _ = draw::write_buf_to_file(&mut data.shm_file, &data.pixel_buf);
                data.surface.attach(Some(&data.buffer), 0, 0);
                data.surface.commit();
                TimeoutAction::ToDuration(timer_interval)
            },
        )
        .map_err(|e| format!("timer: {}", e))?;

    event_loop
        .run(Duration::from_millis(500), &mut loop_data, |_| {})
        .map_err(|e| e.to_string())?;

    Ok(())
}
