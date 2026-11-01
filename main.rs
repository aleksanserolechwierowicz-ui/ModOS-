#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::ptr::write_volatile;

// Adres pamięci tekstowej VGA dla trybu tekstowego
const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
// Wyczyszczenie ekranu i wypisanie powitania w terminalu ModOS
clear_screen();
print_str(0, "Witaj w ModOS v0.0.0 Alpha! Kernel wystartowal.");
print_str(1, "Gotowy do devowania. Wpisz komende...");

// Główna pętla kernela (tutaj w przyszłości podepniemy klawiaturę PS/2 lub USB)
loop {}
}

/// Funkcja czyszcząca ekran VGA
fn clear_screen() {
let mut i = 0;
while i < 80 * 25 * 2 {
unsafe {
write_volatile(VGA_BUFFER.offset(i), 0); // Znak spacji
write_volatile(VGA_BUFFER.offset(i + 1), 0x07); // Kolor: szary tekst na czarnym tle
}
i += 2;
}
}

/// Funkcja wypisująca tekst w danej linii terminala
fn print_str(row: usize, s: &str) {
let mut offset = row * 80 * 2;
for byte in s.bytes() {
unsafe {
write_volatile(VGA_BUFFER.offset(offset), byte);
write_volatile(VGA_BUFFER.offset(offset + 1), 0x0A); // Jasnozielony tekst (hakierski vibe!)
}
offset += 2;
}
}

/// Obsługa nagłych błędów (panic) - wypisuje informację na ekranie (jeśli się da)
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
print_str(23, "PANIC! Kernel zlapal blad.");
loop {}
}