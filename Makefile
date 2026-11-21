.PHONY: x86 arm

x86:
	cargo build --target x86_64-unknown-uefi
	-mkdir EFI
	-mkdir EFI/BOOT
	cp "target/x86_64-unknown-uefi/debug/modos.efi" "EFI/BOOT/BOOTX64.EFI"

arm:
	cargo build --target aarch64-unknown-uefi
	-mkdir EFI
	-mkdir EFI/BOOT
	cp "target/aarch64-unknown-uefi/debug/modos.efi" "EFI/BOOT/BOOTAA64.EFI"