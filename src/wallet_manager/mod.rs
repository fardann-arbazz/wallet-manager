use std::{
    error::Error,
    fmt::Display,
    io::{self, Write},
    thread,
    time::Duration,
};

#[derive(Debug)]
enum WalletError {
    InvalidInput(String),
    InsufficientFunds,
    ParseError,
}

impl Display for WalletError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WalletError::InvalidInput(msg) => write!(f, "Input Tidak Valid: {}", msg),
            WalletError::InsufficientFunds => write!(f, "Saldo Tidak Cukup"),
            WalletError::ParseError => write!(f, "Error Parsing"),
        }
    }
}

impl Error for WalletError {}

#[derive(Debug, Clone, PartialEq)]
enum TypeTransaction {
    Income,
    Expense,
}

impl Display for TypeTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeTransaction::Income => write!(f, "Pemasukan"),
            TypeTransaction::Expense => write!(f, "Pengeluaran"),
        }
    }
}

#[derive(Debug, Clone)]
struct WalletItems {
    description: String,
    amount: i64,
    transaction_type: TypeTransaction,
}

impl WalletItems {
    #[allow(dead_code)]
    fn new(description: String, amount: i64, transaction_type: TypeTransaction) -> Self {
        Self {
            description,
            amount,
            transaction_type,
        }
    }
}

struct WalletList {
    items: Vec<WalletItems>,
}

impl WalletList {
    fn new() -> Self {
        Self { items: Vec::new() }
    }

    fn get_checked(&mut self, index: usize) -> Option<&mut WalletItems> {
        if index == 0 || index > self.items.len() {
            println!("\n   [ERROR] Nomor transaksi tidak valid!");
            None
        } else {
            Some(&mut self.items[index - 1])
        }
    }

    fn calculate_balance(&self) -> i64 {
        self.items
            .iter()
            .fold(0, |acc, items| match items.transaction_type {
                TypeTransaction::Income => acc + items.amount,
                TypeTransaction::Expense => acc - items.amount,
            })
    }

    fn show(&self) {
        println!();
        println!("   RIWAYAT TRANSAKSI");
        println!("   {}", "─".repeat(70));

        if self.items.is_empty() {
            println!("   Tidak ada transaksi ditemukan.");
            println!("   Tambahkan transaksi pertama Anda untuk memulai.");
        } else {
            println!(
                "   {:>3} | {:>12} | {:>15} | Deskripsi",
                "No", "Jenis", "Jumlah"
            );
            println!("   {}", "─".repeat(70));

            for (i, items) in self.items.iter().enumerate() {
                let type_symbol = match items.transaction_type {
                    TypeTransaction::Expense => "-",
                    TypeTransaction::Income => "+",
                };

                let amount_str =
                    format!("{}Rp{}", type_symbol, Self::format_currency(items.amount));
                let desc_truncated = if items.description.len() > 35 {
                    format!("{}...", &items.description[..32])
                } else {
                    items.description.clone()
                };

                println!(
                    "   {:>3} | {:>12} | {:>15} | {}",
                    i + 1,
                    items.transaction_type,
                    amount_str,
                    desc_truncated
                );
            }

            let balance = self.calculate_balance();
            println!("   {}", "─".repeat(70));

            let balance_status = if balance > 0 {
                "SURPLUS"
            } else if balance == 0 {
                "SEIMBANG"
            } else {
                "DEFISIT"
            };

            println!(
                "   TOTAL SALDO: Rp{} [{}]",
                Self::format_currency(balance.abs()),
                balance_status
            );
        }
        println!();
    }

    fn add(&mut self, items: WalletItems) -> Result<(), WalletError> {
        if let TypeTransaction::Expense = items.transaction_type {
            let current_balance = self.calculate_balance();
            if current_balance < items.amount {
                return Err(WalletError::InsufficientFunds);
            }
        }

        self.items.push(items);
        Ok(())
    }

    fn update(&mut self, index: usize, items: WalletItems) -> Result<(), WalletError> {
        if let Some(item) = self.get_checked(index) {
            item.description = items.description;
            item.amount = items.amount;
            item.transaction_type = items.transaction_type;
            Ok(())
        } else {
            Err(WalletError::InvalidInput(format!(
                "Indeks tidak valid: {}",
                index
            )))
        }
    }

    fn filter_by_type(&self, transaction_type: TypeTransaction) -> Vec<WalletItems> {
        self.items
            .iter()
            .filter(|item| item.transaction_type == transaction_type)
            .cloned()
            .collect()
    }

    fn total(&self) {
        let balance = self.calculate_balance();

        println!();
        println!("   INFORMASI SALDO");
        println!("   {}", "─".repeat(50));
        println!();

        let (status_text, symbol) = if balance > 0 {
            ("UNTUNG", "+")
        } else if balance == 0 {
            ("SEIMBANG", "")
        } else {
            ("RUGI", "-")
        };

        println!(
            "   Saldo Saat Ini: {}Rp{}",
            symbol,
            Self::format_currency(balance.abs())
        );
        println!("   Status: {}", status_text);
        println!();
    }

    fn format_currency(amount: i64) -> String {
        let mut result = amount.to_string();
        let mut chars: Vec<char> = result.chars().collect();

        if chars.len() > 3 {
            let mut i = chars.len() - 3;
            while i > 0 {
                chars.insert(i, '.');
                if i > 3 {
                    i -= 3;
                } else {
                    break;
                }
            }
            result = chars.into_iter().collect();
        }

        result
    }

    fn get_summary(&self) -> (i64, i64, i64) {
        let mut total_income = 0;
        let mut total_expense = 0;

        for transaction in &self.items {
            match transaction.transaction_type {
                TypeTransaction::Income => total_income += transaction.amount,
                TypeTransaction::Expense => total_expense += transaction.amount,
            }
        }

        (total_income, total_expense, total_income - total_expense)
    }

    fn remove_transaction(&mut self, index: usize) {
        if let Some(_) = self.get_checked(index) {
            self.items.remove(index - 1);
            println!("\n   [SUCCESS] Transaksi berhasil dihapus.");
        }
    }

    fn search_transaction(&mut self, description: &str) -> Vec<WalletItems> {
        self.items
            .iter()
            .filter(|item| {
                item.description
                    .to_lowercase()
                    .contains(&description.to_lowercase())
            })
            .cloned()
            .collect()
    }
}

struct WalletApp {
    wallet: WalletList,
}

impl WalletApp {
    fn new() -> Self {
        Self {
            wallet: WalletList::new(),
        }
    }

    fn clear_screen(&self) {
        print!("\x1B[2J\x1B[1;1H");
        io::stdout().flush().unwrap();
    }

    fn print_header(&self) {
        println!();
        println!("   SISTEM MANAJEMEN DOMPET");
        println!("   Alat Keuangan Profesional");
        println!();
    }

    fn print_loading(&self, message: &str) {
        print!("   Memproses {}...", message);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(500));
        println!(" Selesai!");
    }

    fn pause(&self) {
        println!("   Tekan Enter untuk melanjutkan...");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
    }

    fn run(&mut self) {
        self.clear_screen();
        self.print_header();

        println!("   Selamat datang di Sistem Manajemen Dompet!");
        println!("   Kelola keuangan Anda dengan mudah dan tepat");
        println!();

        thread::sleep(Duration::from_millis(1000));

        loop {
            self.clear_screen();
            self.print_header();
            self.show_menu();

            match self.get_user_choice() {
                Ok(choice) => {
                    if !self.handle_choice(&choice) {
                        break;
                    }
                }
                Err(e) => {
                    println!("\n   [ERROR] {}", e);
                    self.pause();
                }
            }
        }
    }

    fn show_menu(&self) {
        let balance = self.wallet.calculate_balance();
        let balance_symbol = if balance >= 0 { "+" } else { "-" };

        println!("   MENU UTAMA");
        println!("   {}", "─".repeat(50));
        println!();
        println!(
            "   Saldo Saat Ini: {}Rp{}",
            balance_symbol,
            WalletList::format_currency(balance.abs())
        );
        println!();
        println!("   1. Lihat Riwayat Transaksi");
        println!("   2. Tambah Transaksi Baru");
        println!("   3. Cek Saldo");
        println!("   4. Ringkasan Keuangan");
        println!("   5. Filter Berdasarkan Jenis");
        println!("   6. Hapus Transaksi");
        println!("   7. Perbarui Transaksi");
        println!("   8. Cari Transaksi");
        println!("   9. Keluar");
        println!();
        print!("   Pilih opsi (1-9): ");
        io::stdout().flush().unwrap();
    }

    fn get_user_choice(&self) -> Result<String, WalletError> {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|_| WalletError::InvalidInput("Input tidak valid".to_string()))?;

        Ok(input.trim().to_string())
    }

    fn handle_choice(&mut self, choice: &str) -> bool {
        match choice {
            "1" => {
                self.print_loading("riwayat transaksi");
                self.wallet.show();
                self.pause();
            }
            "2" => {
                self.print_loading("formulir transaksi");
                if let Err(e) = self.handle_add_transaction() {
                    println!("\n   [ERROR] {}", e);
                }
                self.pause();
            }
            "3" => {
                self.print_loading("perhitungan saldo");
                self.wallet.total();
                self.pause();
            }
            "4" => {
                self.print_loading("ringkasan keuangan");
                self.show_summary();
                self.pause();
            }
            "5" => {
                self.print_loading("filter transaksi");
                if let Err(e) = self.handle_show_filter_type() {
                    println!("   [ERROR] {}", e);
                }
                self.pause();
            }
            "6" => {
                self.print_loading("data transaksi");
                if let Err(e) = self.handle_delete_transaction() {
                    println!("   [ERROR] {}", e);
                }
                self.pause();
            }
            "7" => {
                self.print_loading("pembaruan transaksi");
                if let Err(e) = self.handle_update_transaction() {
                    println!("   [ERROR] {}", e);
                }
                self.pause();
            }
            "8" => {
                self.print_loading("pencarian transaksi");
                if let Err(e) = self.handle_search_transaction() {
                    println!("   [ERROR] {}", e);
                }
            }
            "9" => {
                println!();
                println!("   SELAMAT TINGGAL");
                println!("   {}", "─".repeat(30));
                println!();
                println!("   Terima kasih telah menggunakan Sistem Manajemen Dompet");
                println!("   Sampai jumpa lagi!");
                println!();
                thread::sleep(Duration::from_millis(1500));
                return false;
            }
            _ => {
                println!();
                println!("   [ERROR] Perintah tidak valid");
                println!("   Silakan pilih menu 1-9 saja");
                println!();
                self.pause();
            }
        }
        true
    }

    fn handle_add_transaction(&mut self) -> Result<(), WalletError> {
        println!();
        println!("   TAMBAH TRANSAKSI BARU");
        println!("   {}", "─".repeat(30));
        println!();

        let transaction_type = self.get_type_transaction()?;
        let amount = self.get_amount()?;
        let description = self.get_description()?;

        let transaction = WalletItems {
            transaction_type,
            amount,
            description,
        };

        let message = match transaction.transaction_type {
            TypeTransaction::Income => "Pemasukan",
            TypeTransaction::Expense => "Pengeluaran",
        };

        match self.wallet.add(transaction) {
            Ok(_) => {
                println!();
                println!("   [SUCCESS] Transaksi berhasil ditambahkan");
                println!("   Jenis: {}", message);
                println!("   Jumlah: Rp{}", WalletList::format_currency(amount));
                println!();
            }
            Err(e) => {
                println!();
                println!("   [ERROR] Gagal menambahkan transaksi");
                println!("   Alasan: {}", e);
                println!();
            }
        }

        Ok(())
    }

    fn handle_update_transaction(&mut self) -> Result<(), WalletError> {
        println!();
        println!("   PERBARUI TRANSAKSI");
        println!("   {}", "─".repeat(30));
        println!();

        self.wallet.show();

        let index = self.get_index_transaction()?;
        let description = self.get_description()?;
        let transaction_type = self.get_type_transaction()?;
        let amount = self.get_amount()?;

        let transaction = WalletItems {
            transaction_type,
            amount,
            description,
        };

        match self.wallet.update(index, transaction) {
            Ok(_) => {
                println!();
                println!("   [SUCCESS] Transaksi berhasil diperbarui");
                println!();
            }
            Err(e) => {
                println!();
                println!("   [ERROR] Gagal memperbarui transaksi");
                println!("   Alasan: {}", e);
                println!();
            }
        }

        Ok(())
    }

    fn handle_delete_transaction(&mut self) -> Result<(), WalletError> {
        println!();
        println!("   HAPUS TRANSAKSI");
        println!("   {}", "─".repeat(30));
        println!();

        self.wallet.show();

        let index = self.get_index_transaction()?;
        self.wallet.remove_transaction(index);

        Ok(())
    }

    fn handle_show_filter_type(&mut self) -> Result<(), WalletError> {
        println!();
        println!("   FILTER TRANSAKSI");
        println!("   {}", "─".repeat(30));
        println!();

        let filter_type = self.get_filter_type()?;
        let transactions = self.wallet.filter_by_type(filter_type.clone());

        if transactions.is_empty() {
            println!(
                "   Tidak ada transaksi ditemukan untuk jenis: {}",
                filter_type
            );
        } else {
            println!("   HASIL FILTER: {}", filter_type);
            println!("   {}", "─".repeat(50));
            println!();

            for (i, items) in transactions.iter().enumerate() {
                let symbol = match items.transaction_type {
                    TypeTransaction::Income => "+",
                    TypeTransaction::Expense => "-",
                };
                println!(
                    "   {}. {} | {}Rp{}",
                    i + 1,
                    items.description,
                    symbol,
                    WalletList::format_currency(items.amount)
                );
            }
            println!();
        }

        Ok(())
    }

    fn handle_search_transaction(&mut self) -> Result<(), WalletError> {
        println!();
        println!("   CARI TRANSAKSI");
        println!("   {}", "─".repeat(30));
        println!();

        let description = self.get_description()?;
        let items = self.wallet.search_transaction(&description);

        if items.is_empty() {
            println!("   Tidak ada transaksi ditemukan");
            println!("   Kata kunci: '{}'", description);
        } else {
            println!("   HASIL PENCARIAN");
            println!("   {}", "─".repeat(50));
            println!();

            for (i, item) in items.iter().enumerate() {
                let symbol = match item.transaction_type {
                    TypeTransaction::Income => "+",
                    TypeTransaction::Expense => "-",
                };
                println!(
                    "   {}. {} | {}Rp{}",
                    i + 1,
                    item.description,
                    symbol,
                    WalletList::format_currency(item.amount)
                );
            }
            println!();
        }

        self.pause();
        Ok(())
    }

    fn get_filter_type(&self) -> Result<TypeTransaction, WalletError> {
        println!("   JENIS TRANSAKSI");
        println!("   {}", "─".repeat(20));
        println!("   1. Pemasukan");
        println!("   2. Pengeluaran");
        println!();
        print!("   Pilih jenis (1/2): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|_| WalletError::InvalidInput("Input tidak valid".to_string()))?;

        let choice = input.trim().to_string();

        match choice.to_lowercase().as_str() {
            "1" | "pemasukan" => Ok(TypeTransaction::Income),
            "2" | "pengeluaran" => Ok(TypeTransaction::Expense),
            _ => Err(WalletError::InvalidInput("Pilihan tidak valid".to_string())),
        }
    }

    fn get_index_transaction(&self) -> Result<usize, WalletError> {
        print!("   Masukkan nomor transaksi: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|_| WalletError::InvalidInput("Input tidak valid".to_string()))?;

        let index = input
            .trim()
            .parse::<usize>()
            .map_err(|_| WalletError::InvalidInput("Nomor transaksi tidak valid".to_string()))?;

        Ok(index)
    }

    fn get_type_transaction(&self) -> Result<TypeTransaction, WalletError> {
        println!("   JENIS TRANSAKSI");
        println!("   {}", "─".repeat(20));
        println!("   1. Pemasukan");
        println!("   2. Pengeluaran");
        println!();
        print!("   Pilih jenis (1/2): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|_| WalletError::InvalidInput("Input tidak valid".to_string()))?;

        match input.trim().to_lowercase().as_str() {
            "1" | "pemasukan" => Ok(TypeTransaction::Income),
            "2" | "pengeluaran" => Ok(TypeTransaction::Expense),
            _ => Err(WalletError::InvalidInput(
                "Jenis transaksi tidak valid".to_string(),
            )),
        }
    }

    fn get_amount(&self) -> Result<i64, WalletError> {
        print!("   Masukkan jumlah (Rp): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|_| WalletError::InvalidInput("Input tidak valid".to_string()))?;

        let amount = input
            .trim()
            .replace(".", "")
            .replace(",", "")
            .replace("Rp", "")
            .trim()
            .parse()
            .map_err(|_| WalletError::ParseError)?;

        if amount <= 0 {
            return Err(WalletError::InvalidInput(
                "Jumlah harus lebih besar dari 0".to_string(),
            ));
        }

        Ok(amount)
    }

    fn get_description(&self) -> Result<String, WalletError> {
        print!("   Masukkan deskripsi: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|_| WalletError::InvalidInput("Input tidak valid".to_string()))?;

        let description = input.trim().to_string();

        if description.is_empty() {
            return Err(WalletError::InvalidInput(
                "Deskripsi tidak boleh kosong".to_string(),
            ));
        }

        Ok(description)
    }

    fn show_summary(&self) {
        let (total_income, total_expense, balance) = self.wallet.get_summary();

        println!();
        println!("   RINGKASAN KEUANGAN");
        println!("   {}", "─".repeat(50));
        println!();
        println!(
            "   Total Pemasukan  : +Rp{}",
            WalletList::format_currency(total_income)
        );
        println!(
            "   Total Pengeluaran: -Rp{}",
            WalletList::format_currency(total_expense)
        );
        println!("   {}", "─".repeat(50));

        let (status_text, symbol) = if balance > 0 {
            ("SURPLUS", "+")
        } else if balance == 0 {
            ("SEIMBANG", "")
        } else {
            ("DEFISIT", "-")
        };

        println!(
            "   Saldo Akhir      : {}Rp{}",
            symbol,
            WalletList::format_currency(balance.abs())
        );
        println!("   Status Keuangan  : {}", status_text);
        println!();

        // Saran keuangan
        if balance > 0 {
            let savings_rate = (balance as f64 / total_income as f64) * 100.0;
            println!(
                "   Rasio Tabungan: {:.1}% - Kondisi keuangan bagus!",
                savings_rate
            );
        } else if balance == 0 {
            println!("   Pengeluaran sama dengan pemasukan - pertahankan keseimbangan ini!");
        } else {
            println!("   Pertimbangkan untuk mengurangi pengeluaran atau menambah pemasukan.");
        }
        println!();
    }

    fn show_welcome_animation(&self) {
        let messages = vec![
            "Memuat sistem",
            "Menginisialisasi dompet",
            "Mempersiapkan antarmuka",
            "Siap digunakan",
        ];

        for message in &messages {
            print!("\r   {}...", message);
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(400));
        }
        println!("\r   Sistem Manajemen Dompet Siap!");
    }
}

pub fn wallet_app() {
    let mut app = WalletApp::new();

    // Tampilkan animasi selamat datang
    app.show_welcome_animation();
    thread::sleep(Duration::from_millis(800));

    app.run();
}
