use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::process::Command;

fn main() -> io::Result<()> {
    // Membuka file sessions.txt
    let file = File::open("sessions.txt")?;
    let reader = io::BufReader::new(file);

    // Deteksi sistem operasi
    let os = env::consts::OS; // Mengambil nama OS (linux, windows, macos)

    // Membaca setiap baris (folder) dalam sessions.txt
    for line in reader.lines() {
        let folder = line?; // Ambil folder
        println!("Changing directory and pulling updates for: {}", folder);

        // Menentukan perintah berdasarkan OS
        let status = if os == "windows" {
            // Jika di Windows
            Command::new("cmd")
                .arg("/C")
                .arg(format!("cd {} && git pull", folder))
                .status()?
        } else {
            // Jika di Linux, Termux, atau macOS
            Command::new("sh")
                .arg("-c")
                .arg(format!("cd {} && git pull", folder))
                .status()?
        };

        if status.success() {
            println!("Successfully pulled updates for: {}", folder);
        } else {
            eprintln!("Failed to pull updates for: {}", folder);
        }
    }

    Ok(())
}
