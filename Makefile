.PHONY: x86 arm run _run

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



# This will only run on x86 Windows machines with Hyper-V.
run:
	sudo make _run

_run: x86
	-powershell -c "Set-VMVideo -VMName ModOS \
	-ResolutionType Single -HorizontalResolution 600 -VerticalResolution 1024"
	-powershell -c "Mount-VHD -Path ModOS.vhdx"
	-mkdir M:/EFI
	-mkdir M:/EFI/BOOT
	-cp "EFI/BOOT/BOOTX64.EFI" "M:/EFI/BOOT/BOOTX64.EFI"
	-powershell -c "Dismount-VHD -Path ModOS.vhdx;Start-VM -Name ModOS"
	-vmconnect localhost ModOS
	-powershell -c "Stop-VM -Name ModOS -TurnOff"
