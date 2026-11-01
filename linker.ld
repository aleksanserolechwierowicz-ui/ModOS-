ENTRY(start)

SECTIONS {
/* Załaduj kernel pod standardowym adresem 1 Megabajta, gdzie oczekuje go Multiboot2 */
. = 1M;

.boot : {
/* Nagłówek Multiboot musi być na samym początku, żeby GRUB go wykrył /
KEEP((.multiboot_header))
}

.text : {
*(.text)
}
}