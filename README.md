# Sistem Manajemen Dompet (Wallet Management System)

![Rust](https://img.shields.io/badge/Bahasa-Rust-orange?style=flat-square&logo=rust)
![License](https://img.shields.io/badge/Lisensi-MIT-blue?style=flat-square)
![Status](https://img.shields.io/badge/Status-Stable-brightgreen?style=flat-square)

Sebuah aplikasi manajemen keuangan pribadi berbasis CLI (Command Line Interface) yang dibangun dengan bahasa pemrograman Rust. Aplikasi ini membantu Anda mencatat pemasukan dan pengeluaran dengan fitur-fitur lengkap untuk manajemen keuangan sehari-hari.

## 🌟 Fitur Utama

- 📊 **Manajemen Transaksi**:
  - Tambah transaksi pemasukan/pengeluaran
  - Edit dan hapus transaksi
  - Cari transaksi berdasarkan deskripsi
- 📈 **Analisis Keuangan**:
  - Tampilkan saldo saat ini
  - Ringkasan total pemasukan dan pengeluaran
  - Filter transaksi berdasarkan jenis
- 💰 **Format Mata Uang**:
  - Tampilan jumlah uang dengan format Rupiah (Rp)
  - Pemisah ribuan otomatis
- 🛡️ **Validasi Data**:
  - Pencegahan saldo negatif
  - Validasi input pengguna
- 🎨 **Antarmuka User-Friendly**:
  - Tampilan tabel yang rapi
  - Animasi loading
  - Pesan error/sukses yang informatif

## 🚀 Cara Menggunakan

### Prasyarat

- Rust dan Cargo terinstal di sistem Anda
  - Instalasi Rust: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

### Instalasi & Menjalankan

1. Clone repositori ini:
   ```bash
   git clone https://github.com/fardann-arbazz/wallet-manager.git
   cd wallet-manager
   ```

2. Build dan jalankan program:
   ```bash
   cargo run --release
   ```

### Panduan Penggunaan

Setelah aplikasi berjalan, Anda akan melihat menu utama dengan pilihan:

1. **Lihat Riwayat Transaksi**: Menampilkan semua transaksi dalam format tabel
2. **Tambah Transaksi Baru**: Menambahkan transaksi baru (pemasukan/pengeluaran)
3. **Cek Saldo**: Menampilkan saldo saat ini dengan status (Surplus/Seimbang/Defisit)
4. **Ringkasan Keuangan**: Menampilkan analisis keuangan lengkap
5. **Filter Berdasarkan Jenis**: Menyaring transaksi berdasarkan pemasukan/pengeluaran
6. **Hapus Transaksi**: Menghapus transaksi tertentu
7. **Perbarui Transaksi**: Mengedit transaksi yang sudah ada
8. **Cari Transaksi**: Mencari transaksi berdasarkan kata kunci
9. **Keluar**: Keluar dari aplikasi

## 📝 Contoh Penggunaan

### Menambahkan Transaksi Pemasukan
```plaintext
   TAMBAH TRANSAKSI BARU
   ──────────────────────────────

   JENIS TRANSAKSI
   ────────────────────
   1. Pemasukan
   2. Pengeluaran

   Pilih jenis (1/2): 1
   Masukkan jumlah (Rp): 2500000
   Masukkan deskripsi: Gaji Bulan Juli

   [SUCCESS] Transaksi berhasil ditambahkan
   Jenis: Pemasukan
   Jumlah: Rp2.500.000
```

### Melihat Ringkasan Keuangan
```plaintext
   RINGKASAN KEUANGAN
   ──────────────────────────────────────────────────

   Total Pemasukan  : +Rp5.750.000
   Total Pengeluaran: -Rp3.200.000
   ──────────────────────────────────────────────────
   Saldo Akhir      : +Rp2.550.000
   Status Keuangan  : SURPLUS

   Rasio Tabungan: 44.3% - Kondisi keuangan bagus!
```

## 🤝 Berkontribusi

Kontribusi selalu diterima! Berikut cara berkontribusi:

1. Fork proyek ini
2. Buat branch fitur baru (`git checkout -b fitur-baru`)
3. Commit perubahan Anda (`git commit -am 'Menambahkan fitur baru'`)
4. Push ke branch (`git push origin fitur-baru`)
5. Buat Pull Request

## 📜 Lisensi

Proyek ini dilisensikan di bawah Lisensi MIT - lihat file [LICENSE](LICENSE) untuk detail lebih lanjut.

## ✨ Credits

Dikembangkan dengan ❤️ oleh Fardan Arbaz menggunakan Rust.

---

💡 **Tips**: Aplikasi ini cocok untuk:
- Pencatatan keuangan pribadi
- Belajar pemrograman Rust
- Contoh implementasi CLI yang interaktif

Jangan lupa untuk ⭐ repositori ini jika Anda merasa proyek ini bermanfaat!
