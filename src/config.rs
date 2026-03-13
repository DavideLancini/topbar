//! Config a compile-time. Modifica e ricompila.

/// Altezza della barra in pixel (compatto per font 18px).
pub const BAR_HEIGHT: u32 = 22;

/// Layer Wayland (Top = barra in alto).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[allow(dead_code)]
pub enum Layer {
    Background = 0,
    Bottom = 1,
    Top = 2,
    Overlay = 3,
}

pub const LAYER: Layer = Layer::Top;
pub const LAYER_NAMESPACE: &str = "topbar";
/// Nero 100% opaco (RGB per XRGB8888).
pub const BG_COLOR: u32 = 0x00_00_00;
pub const FG_COLOR: u32 = 0xCD_D6_F4;
/// Stesso size di Alacritty (font size 18).
pub const FONT_SIZE: f32 = 18.0;
/// Monospace di sistema (fc-match monospace): Noto Sans Mono.
pub const FONT_PATH: &str = "/usr/share/fonts/noto/NotoSansMono-Regular.ttf";

// --- Clock (usati quando abilitiamo il modulo clock)
#[allow(dead_code)]
pub const CLOCK_FORMAT: &str = "%H:%M %d/%m/%y";
#[allow(dead_code)]
pub const CLOCK_INTERVAL_SECS: u64 = 1;

// --- Backlight (path compilato)
#[allow(dead_code)]
pub const BACKLIGHT_PATH: &str = "/sys/class/backlight/amdgpu_bl1";
#[allow(dead_code)]
pub const BACKLIGHT_INTERVAL_SECS: u64 = 2;

// --- Battery (nome power_supply, es. BAT0)
#[allow(dead_code)]
pub const BATTERY_NAME: &str = "BAT0";
#[allow(dead_code)]
pub const BATTERY_INTERVAL_SECS: u64 = 5;

// --- CPU / memory / temp
#[allow(dead_code)]
pub const CPU_INTERVAL_SECS: u64 = 3;
#[allow(dead_code)]
pub const MEMORY_INTERVAL_SECS: u64 = 3;
#[allow(dead_code)]
pub const TEMP_INTERVAL_SECS: u64 = 1;
#[allow(dead_code)]
pub const CPU_FREQ_INTERVAL_SECS: u64 = 3;

// --- Custom timer (path script)
#[allow(dead_code)]
pub const TIMER_SCRIPT: &str = "/home/davidelancini/0Projects/myArch/configs/waybar/timer.sh";
#[allow(dead_code)]
pub const TIMER_INTERVAL_SECS: u64 = 1;

// --- Mega sync
#[allow(dead_code)]
pub const MEGA_INTERVAL_SECS: u64 = 30;

// --- Notifications
#[allow(dead_code)]
pub const NOTIFICATIONS_INTERVAL_SECS: u64 = 1;

// --- Network / pulseaudio / bluetooth (polling)
#[allow(dead_code)]
pub const NETWORK_INTERVAL_SECS: u64 = 5;
#[allow(dead_code)]
pub const PULSEAUDIO_INTERVAL_SECS: u64 = 2;
#[allow(dead_code)]
pub const BLUETOOTH_INTERVAL_SECS: u64 = 5;
