# wrkfetch
A lightweight, custom system information fetch tool written in Rust. featuring a Saturn ASCII art and packages from package managers counting.

`wrkfetch` is a CLI tool built from scratch over 2 days as a learning project to dive deeper into Rust programmming, system APIs, and async programming in a systems context. It gathers essential system metrics and displays them neatly alongside a custom Saturn ASCII.

---

![wrkfetch](https://github.com/wxwreak/wrkfetch/blob/main/wrkfetch.jpg)

---

## Features
- **Fast & Lightweight:** Written in Rust using efficient system interorgation libraries (`sysinfo`, `wgpu`).

- **Package Managers Support**
| Package Managers |
| :--- |
| Pacman, DPKG, RPM, FlatPak, Port, PKG, Xbps, Nix |

- **Visual Memory Bar:** Includs a dynamic bar representing current RAM usage.
- **Network & Hardware Info:** Fetches active GPU name, local IP, active shell, WM/DE, uptime, kernel version.

## Displayed Info
- **User & Hostname**
- **OS & Kernel Version**
- **Shell & WM / DE**
- **Uptime**
- **CPU Brand**
- **GPU Name**
- **Memory Usage with visual progress bar**
- **Packages breakdown count and manager names**
- **Local IP**
- **Color palette dots**

## Reqs
- **Rust** (edition 2024)
- **Crates Used:**
    - `colored`
    - `sysinfo`
    - `wgpu`
    - `pollster`

## Install & Usage
### Installation via Makefile
If you want to quickly build and install `wrkfetch` to `~/.local/bin`:

```bash
make
make install
```
Make sure you have included `PATH` of `~/.local/bin`
Simple run:

```bash
wrkfetch
```

## Project Purpose
This tool was created as a personal hands-on project to learn Rust, explore system internals, process file directories for package counting, and handle async GPU queries using `wgpu`.

## License
This project is licensed undere the MIT License - see the [License](LICENSE).