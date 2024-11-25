# Auto GitPull
**Auto GitPull** adalah sebuah tool sederhana yang dirancang untuk memperbarui repositori Git secara otomatis dari daftar folder yang didefinisikan dalam file `sessions.txt`. Tool ini mendukung berbagai platform seperti Windows, Linux, Termux, dan macOS dengan mendeteksi lingkungan perangkat secara otomatis.

---

## Versi
**Versi Saat Ini**: `v1.0.0`

---

## Fitur Utama
- **Multi-Platform Support**:  
  Script secara otomatis mendeteksi sistem operasi dan menyesuaikan perintah berdasarkan platform (Windows, Linux, Termux, atau macOS).

- **Otomasi Git Pull**:  
  Melakukan `git pull` untuk setiap repositori dalam daftar folder, memastikan kode selalu terbarui.

- **Notifikasi Status**:  
  Memberikan notifikasi apakah pembaruan berhasil atau gagal untuk setiap repositori.

---

## Bahasa Pemrograman
Tool ini ditulis menggunakan bahasa **Rust**, yang dikenal untuk performanya yang cepat dan efisien.

---

## Cara Install dan Jalankan

Ikuti langkah-langkah berikut untuk menggunakan **Auto GitPull**:

### 1. Clone Repositori
Unduh kode sumber dengan perintah:
```bash
git clone https://github.com/username/auto-gitpull.git
```

### 2. Masuk ke Direktori Proyek
Pindah ke direktori proyek:
```bash
cd auto-gitpull
```

### 3. Compile Proyek
Kompilasi script menggunakan Cargo:
```bash
cargo build --release
```

### 4. Jalankan Script
Jalankan script dengan perintah:
```bash
./target/release/auto-gitpull
```

---

## Konfigurasi
### File `sessions.txt`
Tool ini menggunakan file `sessions.txt` untuk mendefinisikan daftar folder yang akan diperbarui. Setiap baris dalam file ini adalah path absolut ke folder repositori Git.

Contoh isi `sessions.txt`:
```
/data/data/com.termux/files/home/blockmesh-autobot
C:\Users\User\Documents\MyRepo
/var/www/html/website
```

Pastikan folder yang didefinisikan sudah berisi repositori Git.

---

## Catatan
- Script ini mendukung sistem operasi Windows, Linux, Termux, dan macOS secara otomatis.
- Pastikan perangkat Anda telah memiliki **Git** dan **Rust** terinstal sebelum menjalankan script.
- Error pada proses `git pull` akan dicetak di konsol untuk membantu debugging.

---

## Kontributor
Script ini dikembangkan oleh **livexord**.  
- Telegram: [@livexordsscript](https://t.me/livexordsscript)

--- 