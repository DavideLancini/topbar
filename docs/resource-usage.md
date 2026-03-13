# Utilizzo risorse – topbar vs waybar

Misura effettuata il 2026-03-04 (topbar in esecuzione da ~2 s, waybar da ~14 h).

## topbar (questo progetto)

| Metrica    | Valore   |
|-----------|----------|
| VmPeak    | 24 468 kB (~24 MB) |
| VmSize    | 24 028 kB (~23 MB) |
| VmRSS     | 22 036 kB (~21 MB) |
| VmHWM     | 22 320 kB (~22 MB) |
| Threads   | 1        |
| Binario   | release, Rust |

```
  PID    VSZ   RSS %MEM %CPU  COMMAND
topbar  24028 22036  0.1  ~4  topbar
```

## waybar (riferimento)

| Metrica    | Valore   |
|-----------|----------|
| VmPeak    | 1 764 544 kB (~1,7 GB) |
| VmSize    | 1 701 108 kB (~1,6 GB) |
| VmRSS     | 32 444 kB (~32 MB)     |
| VmHWM     | 60 940 kB (~59 MB)     |
| Threads   | 24       |
| Binario   | C++, GTK, ecc. |

```
  PID      VSZ   RSS %MEM %CPU  COMMAND
waybar 1701108 32444  0.2  5.0  waybar
```

## Confronto sintetico

|           | topbar | waybar |
|-----------|--------|--------|
| RSS       | ~21 MB | ~32 MB |
| VSZ       | ~23 MB | ~1,6 GB |
| Threads   | 1      | 24     |

RSS (memoria fisica effettiva): topbar usa circa **un terzo** della memoria di waybar.  
VSZ e numero di thread sono molto più bassi in topbar (configurazione minimale, senza GTK/config runtime).
