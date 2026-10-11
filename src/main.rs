// src/main.rs
use chrono::{Datelike, Local};
use clap::Parser;
use colored::Colorize;
use std::{panic, thread};
use sysinfo::System;
mod utils;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short = 'a', long = "ascii")]
    name: Option<String>,
}

struct PackageManager {
    name: &'static str,
    db_path: &'static str,
}

impl PackageManager {
    fn count_packages(&self) -> Option<usize> {
        let metadata = std::fs::metadata(self.db_path).ok()?;
        if !metadata.is_dir() {
            return None;
        }

        let dir = std::fs::read_dir(self.db_path).ok()?;
        let mut count = 0;

        for entry in dir.flatten() {
            let name = entry.file_name().to_string_lossy().into_owned();

            if name.starts_with('.') || name == "ALPM_DB_VERSION" {
                continue;
            }

            if self.name == "flatpak" {
                if let Ok(mut app_dir) = std::fs::read_dir(entry.path())
                    && app_dir.any(|e| {
                        e.map(|b| b.file_name().to_string_lossy() == "current")
                            .unwrap_or(false)
                    })
                {
                    count += 1;
                }
                continue;
            }

            count += 1;
        }

        if count > 0 { Some(count) } else { None }
    }
}

fn main() {
    panic::set_hook(Box::new(|_| {}));
    let args = Args::parse();
    let target_name = match args.name {
        Some(name) => name,
        None => {
            let today = Local::now();
            if today.month() == 10 && today.day() == 31 {
                "pumpkin".to_string()
            } else {
                "saturn".to_string()
            }
        }
    };
    let ascii_art = match utils::get_ascii_art(&target_name) {
        Some(art) => art,
        None => {
            println!("Warning: art '{}' does not exist. Use saturn", target_name);
            utils::SATURN
        }
    };

    let mut sys = System::new_all();
    sys.refresh_all();
    thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_all();

    let username = std::env::var("USER")
        .or_else(|_| std::env::var("USERNAME"))
        .unwrap_or_else(|_| "Unknown".to_string());
    let os_name = System::name().unwrap_or_else(|| "Unknown".to_string());
    let kernel_ver = System::kernel_version().unwrap_or_else(|| "Unknown".to_string());

    let uptime_secs = System::uptime();
    let hours = uptime_secs / 3600;
    let minutes = (uptime_secs % 3600) / 60;
    let uptime_str = if hours > 0 {
        format!("{} hours, {} minutes", hours, minutes)
    } else {
        format!("{} minutes", minutes)
    };

    let total_ram = sys.total_memory() / 1024 / 1024;
    let used_ram = sys.used_memory() / 1024 / 1024;
    let cpu_brand = sys
        .cpus()
        .first()
        .map(|cpu| cpu.brand().trim())
        .unwrap_or("Unknown");
    let cpu_info_str = cpu_brand.to_string();

    let gpu_info_str = utils::get_gpu().unwrap_or_else(|| "Unknown GPU".to_string());

    let local_ip_str = std::net::UdpSocket::bind("0.0.0.0:0")
        .and_then(|socket| socket.connect("8.8.8.8:80").map(|_| socket))
        .and_then(|socket| socket.local_addr())
        .map(|addr| addr.ip().to_string())
        .unwrap_or_else(|_| "Unknown".to_string());

    let pct = (used_ram as f32 / total_ram as f32) * 10.0;
    let index = (pct as usize).min(9);
    let mut bar_chars = vec!['━'; 10];
    bar_chars[index] = '■';
    let bar_str: String = bar_chars.into_iter().collect();
    let memory_info_str = format!(
        "{} {} MiB / {} MiB",
        bar_str.truecolor(120, 120, 120),
        used_ram,
        total_ram
    );

    let shell_str = utils::get_shell();

    let managers = vec![
        PackageManager {
            name: "pacman",
            db_path: "/var/lib/pacman/local",
        },
        PackageManager {
            name: "dpkg",
            db_path: "/var/lib/dpkg/info",
        },
        PackageManager {
            name: "rpm",
            db_path: "/var/lib/rpm",
        },
        PackageManager {
            name: "flatpak",
            db_path: "/var/lib/flatpak/app",
        },
        PackageManager {
            name: "port",
            db_path: "/opt/local/var/macports/software",
        },
        PackageManager {
            name: "pkg",
            db_path: "/var/db/pkg",
        },
        PackageManager {
            name: "xbps",
            db_path: "/var/db/xbps",
        },
        PackageManager {
            name: "nix",
            db_path: "/nix/var/nix/profiles/system/sw",
        },
    ];

    let mut found_managers = Vec::new();
    let mut total_pkgs = 0;

    for pm in managers {
        if let Some(count) = pm.count_packages() {
            found_managers.push(format!("{} ({})", count, pm.name));
            total_pkgs += count;
        }
    }

    let packages_str = if total_pkgs > 0 {
        found_managers.join(", ")
    } else {
        "0".to_string()
    };

    let wm_de = utils::get_wm_de();
    let dots = format!(
        "{} {} {} {} {} {}",
        "●".red(),
        "●".green(),
        "●".yellow(),
        "●".blue(),
        "●".magenta(),
        "●".cyan()
    );

    let lines: Vec<String> = vec![
        format!("{} {}", "User:    ".bold().blue(), username),
        format!("{} {} {}", "OS:   ".bold().blue(), os_name, kernel_ver),
        format!("{} {}", "Kernel:  ".bold().blue(), kernel_ver),
        format!("{} {}", "Shell:   ".bold().blue(), shell_str),
        format!("{} {}", "WM/DE:   ".bold().blue(), wm_de),
        format!("{} {}", "Uptime:  ".bold().blue(), uptime_str),
        format!("{} {}", "CPU:     ".bold().blue(), cpu_info_str),
        format!("{} {}", "GPU:     ".bold().blue(), gpu_info_str),
        format!("{} {}", "Memory:  ".bold().blue(), memory_info_str),
        format!("{} {}", "Packages:".bold().blue(), packages_str),
        format!("{} {}", "Local IP:".bold().blue(), local_ip_str),
        dots,
    ];

    let cleaned_ascii = ascii_art.trim_matches('\n');
    let ascii_lines: Vec<String> = cleaned_ascii
        .lines()
        .map(|line| match target_name.as_str() {
            "pumpkin" | "jackolantern" | "halloween" => line.truecolor(255, 117, 24).to_string(),
            "saturn" => {
                if line.contains("o888") || line.contains("Pd88") {
                    line.truecolor(224, 194, 142).to_string()
                } else {
                    line.truecolor(140, 125, 107).to_string()
                }
            }
            _ => line.to_string(),
        })
        .collect();

    let mut final_lines = vec!["".to_string(), "".to_string()];
    final_lines.extend(lines);

    let max_logo_width = ascii_lines.iter().map(|s| s.len()).max().unwrap_or(0);

    println!();

    let total_rows = std::cmp::max(ascii_lines.len(), final_lines.len());

    for i in 0..total_rows {
        let logo_part = if i < ascii_lines.len() {
            ascii_lines[i].cyan().to_string()
        } else {
            " ".repeat(max_logo_width)
        };

        let info_part = if i < final_lines.len() {
            &final_lines[i]
        } else {
            ""
        };
        println!("{}   {}", logo_part, info_part);
    }
    println!();
}
