ModOS
​English | Polski
​English
​ModOS is a minimalist, custom no_std operating system kernel written in Rust and Assembly, designed for x86_64 architecture and booted via GRUB2 using Multiboot2.
​Project Structure:
modos/
├── .cargo/
│   └── config.toml     # Cargo target configuration (x86_64-unknown-none)
├── src/
│   ├── boot.s          # Multiboot2 assembly entry point & stack setup
│   └── main.rs         # Rust kernel entry point with VGA text-mode terminal
├── Cargo.toml          # Rust package configuration
└── linker.ld           # Custom linker script (loads kernel at 1M)
​Requirements & Building:
• ​Rust Toolchain (nightly recommended for build-std)
• ​LLVM tools / rust-std components (x86_64-unknown-none)
• ​GRUB / xorriso (for building bootable ISO images)
​Build the kernel using Cargo:
cargo build --release
​Polski
​ModOS to minimalistyczny, autorski jądro systemu operacyjnego typu no_std, napisany w języku Rust oraz Assemblerze. Projekt jest przeznaczony dla architektury x86_64 i uruchamiany za pomocą programu rozruchowego GRUB2 (standard Multiboot2).
​Struktura projektu:
modos/
├── .cargo/
│   └── config.toml     # Konfiguracja docelowa Cargo (x86_64-unknown-none)
├── src/
│   ├── boot.s          # Punkt wejścia w Assemblerze (Multiboot2) i konfiguracja stosu
│   └── main.rs         # Główny kod kernela w Ruście z terminalem VGA
├── Cargo.toml          # Konfiguracja paczki w Ruście
└── linker.ld           # Własny skrypt linkera (ładowanie jądra pod adres 1M)
​Wymagania i budowanie:
• ​Narzędzia Rusta (zalecana wersja nightly pod build-std)
• ​Komponenty x86_64-unknown-none
• ​GRUB / xorriso (do budowania bootowalnych obrazów ISO)