// src/utils.rs
use std::env;
use std::process::Command;

pub fn get_wm_de() -> String {
    let raw_name = env::var("XDG_CURRENT_DESKTOP")
        .or_else(|_| env::var("DESKTOP_SESSION"))
        .unwrap_or_else(|_| "unknown".to_string());

    if raw_name == "unknown" {
        return "Unknown".to_string();
    }
    let mut name = raw_name.to_lowercase();
    if let Some(first_char) = name.get_mut(0..1) {
        first_char.make_ascii_uppercase();
    }
    name
}

pub fn get_shell() -> String {
    let raw_shell = env::var("SHELL").unwrap_or_else(|_| "unknown".to_string());
    let mut shell_name = raw_shell
        .split("/")
        .last()
        .unwrap_or("unknown")
        .to_lowercase();
    if shell_name == "unknown" {
        return "Unknown".to_string();
    }
    if let Some(first_char) = shell_name.get_mut(0..1) {
        first_char.make_ascii_uppercase();
    }
    shell_name
}

pub const SATURN: &str = r##"
                                             ___
                                          ,o88888
                                       ,o8888888'
                 ,:o:o:oooo.        ,8O88Pd8888"
             ,.::.::o:ooooOoOoO. ,oO8O8Pd888'"
           ,.:.::o:ooOoOoOO8O8OOo.8OOPd8O8O"
          , ..:.::o:ooOoOOOO8OOOOo.FdO8O8"
         , ..:.::o:ooOoOO8O888O8O,COCOO"
        , . ..:.::o:ooOoOOOO8OOOOCOCO"
         . ..:.::o:ooOoOoOO8O8OCCCC"o
            . ..:.::o:ooooOoCoCCC"o:o
            . ..:.::o:o:,cooooCo"oo:o:
         `   . . ..:.:cocoooo"'o:o:::'
         .`   . ..::ccccoc"'o:o:o:::'
        :.:.    ,c:cccc"':.:.:.:.:.'
      ..:.:"'`::::c:"'..:.:.:.:.:.'
    ...:.'.:.::::"'    . . . . .'
   .. . ....:."' `   .  . . ''
 . . . ...."'
 .. . ."'
"##;

pub const JACK_O_LANTERN: &str = r##"
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣾⡷⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⣷⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⣽⣦⣀⡀⠤⠤⣤⣀⡀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⣀⠴⠊⡉⠔⢛⠭⠉⡇⠙⢄⠈⠐⠮⡉⠒⠬⣑⠢⣀⠀⠀
⠀⠀⣠⠞⠁⡴⠋⠀⡰⠊⡀⠀⢸⠀⠀⢣⠀⠀⣦⢄⡀⠈⠣⡈⢧⡀
⠀⡔⠁⢀⠎⠀⠀⡰⢡⢮⣇⠀⢸⠀⠀⠀⠃⠀⣿⣦⡙⡄⠀⢷⡀⢃
⠸⠁⠀⡞⠀⠀⢀⣇⡇⣾⣿⣆⠀⠀⠀⠀⡸⠘⠛⠛⠛⡉ ⠀ ⡇⢸
⡆⠀⢸⠀⠀⠀⢸⠉⠉⠉⠉⠁⠀⠀⢻⣿⢹⠀⠀⠀⠀⣠ ⠀ ⠁⢸
⡇⠀⢸⠀⠀⠀⠘⠀⠀⠀⠀⠀⠀⡀⠀⠫⢾⠀⢀⣤⢞⠋⠀ ⢠⠃⠸
⢡⠀⠈⠀⠀⠀⠈⢳⡦⣄⣀⣤⣀⣧⣶⣤⣾⣿⣿⣣ ⢫⠀⠀⡏⢠⠆
⠈⢧⡀⠑⡀⠀⠀⠘⠝⠾⠿⣿⣿⡿⠿⡿⠛⠏⠉⠀⡜⢠⠞⣠⠏⠀
⠀⠀⠳⣄⠈⢤⠀⠀⠈⢢⡀⠈⠋⡇⠀⠀⠸⠀⠀⡔⡠⣋⡴⠋⠀⠀
⠀⠀⠀⠈⠙⠲⠯⠶⢤⣀⣑⣦⣀⡇⠤⠴⠥⠴⠚⠈⠉⠁⠀⠀⠀⠀
"##;

pub fn get_ascii_art(name: &str) -> Option<&'static str> {
    match name.to_lowercase().as_str() {
        "saturn" => Some(SATURN),
        "pumpkin" | "jackolantern" | "halloween" => Some(JACK_O_LANTERN),

        _ => None,
    }
}

pub fn get_gpu() -> Option<String> {
    let output = Command::new("lspci").arg("-mm").output().ok()?;
    let stdout = String::from_utf8_lossy(&output.stdout);

    for line in stdout.lines() {
        if line.contains("\"VGA compatible controller\"") || line.contains("\"3D controller\"") {
            let parts: Vec<&str> = line.split('"').collect();

            if parts.len() > 5 {
                let vendor = parts[3].trim();
                let device = parts[5].trim();
                return Some(format!("{} {}", vendor, device));
            }
        }
    }
    None
}
