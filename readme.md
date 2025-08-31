# Belajar Fungsi & Modul di Rust - Kalkulator Mini

Proyek ini adalah contoh sederhana untuk mempelajari **fungsi (`fn`)** dan **modul (`mod`)** dalam bahasa pemrograman [Rust](https://www.rust-lang.org/).  
Sebagai studi kasus, kita membuat **kalkulator mini** yang bisa melakukan operasi dasar matematika.

## Tujuan

- Memahami cara membuat dan memanggil fungsi di Rust
- Memahami struktur dan penggunaan modul (`mod`) dalam satu file
- Belajar menggunakan `pub` untuk membuat fungsi bisa diakses dari luar modul

---

## Materi yang Dipraktikkan

✅ Fungsi dengan parameter dan return  
✅ Modularisasi kode dengan `mod`  
✅ Mengakses fungsi dari modul menggunakan `modul::fungsi()`  
✅ Penanganan pembagian dengan nol  
✅ Tipe data dasar: `i32`, `f32`

---

## Struktur Proyek

src/
└── main.rs # Semua kode (fungsi & modul) ada di sini
Cargo.toml # File konfigurasi proyek Rust

## Output Program

20 + 4 = 24
20 - 4 = 16
20 * 4 = 80
20 / 4 = 5
15% dari 200 = 30
