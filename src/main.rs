// --- Modul kalkulator ---
mod kalkulator {
    pub fn tambah(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn kurang(a: i32, b: i32) -> i32 {
        a - b
    }

    pub fn kali(a: i32, b: i32) -> i32 {
        a * b
    }

    pub fn bagi(a: i32, b: i32) -> f32 {
        if b == 0 {
            println!("⚠️ Error: Pembagian dengan nol!");
            0.0
        } else {
            a as f32 / b as f32
        }
    }

    pub fn persen(total: f32, persen: f32) -> f32 {
        (total * persen) / 100.0
    }
}

// --- Fungsi utama ---
fn main() {
    let a = 20;
    let b = 4;

    println!("{} + {} = {}", a, b, kalkulator::tambah(a, b));
    println!("{} - {} = {}", a, b, kalkulator::kurang(a, b));
    println!("{} * {} = {}", a, b, kalkulator::kali(a, b));
    println!("{} / {} = {}", a, b, kalkulator::bagi(a, b));
    
    let total = 200.0;
    let persen = 15.0;
    println!("{}% dari {} = {}", persen, total, kalkulator::persen(total, persen));
}
