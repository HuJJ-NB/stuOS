# Building
TARGET := riscv64gc-unknown-none-elf
MODE := release
LINKER := src/linker.ld
LINKER_SRC_PATH := src/linker
LINKED_APP_BINS := src/link_app.S
KERNEL_ELF := target/$(TARGET)/$(MODE)/os
KERNEL_BIN := $(KERNEL_ELF).bin
DISASM_TMP := target/$(TARGET)/$(MODE)/asm

# BOARD
BOARD ?= qemu
SBI ?= rustsbi
BOOTLOADER := ../bootloader/$(SBI)-$(BOARD).bin
K210_BOOTLOADER_SIZE := 131072

# KERNEL ENTRY
ifeq ($(BOARD), qemu)
	KERNEL_ENTRY_PA := 0x80200000
else ifeq ($(BOARD), k210)
	KERNEL_ENTRY_PA := 0x80020000
endif

# Run K210
K210-SERIALPORT	= /dev/ttyUSB0
K210-BURNER = ../tools/kflash.py

# Binutils
OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64

# Disassembly
DISASM ?= -x

build: env switch-check $(KERNEL_BIN)

switch-check:
ifeq ($(BOARD), qemu)
ifneq (last-qemu, $(wildcard last-qemu))
	rm -f last-k210
	touch last-qemu
	make clean
	echo clean last k210 env-sets.
endif
else ifeq ($(BOARD), k210)
ifneq (last-k210, $(wildcard last-k210))
	rm -f last-qemu
	touch last-k210
	make clean
	echo clean last qemu env-sets.
endif
endif

env:
	@echo Check if target $(TARGET) has been installed.
	@(rustup target list | grep "$(TARGET) (installed)") || rustup target add $(TARGET)
	cargo install cargo-binutils --vers =0.3.3
	rustup component add rust-src
	rustup component add llvm-tools-preview

$(KERNEL_BIN): $(KERNEL_ELF)
	@echo make binary file.
	@$(OBJCOPY) $(KERNEL_ELF) --strip-all -O binary $@

$(KERNEL_ELF): app-bin $(LINKER)
	@echo --build ruianOS.
	@echo ----Platform: $(BOARD)
	@cargo build --release
	@rm $(LINKED_APP_BINS)

$(LINKER): $(LINKER_SRC_PATH)/linker-head-$(BOARD).ld $(LINKER_SRC_PATH)/linker-body.ld
	@echo make new linker file.
	@cat $(LINKER_SRC_PATH)/linker-head-$(BOARD).ld $(LINKER_SRC_PATH)/linker-body.ld > $(LINKER)

app-bin:
	@echo --upgrade user apps.
	@cd ../user && make build

clean:
	@cargo clean
	@cd ../user && make clean
ifeq ($(LINKER) , $(wildcard $(LINKER)))
	@rm $(LINKER)
endif

disasm: $(KERNEL_ELF)
	@$(OBJDUMP) $(DISASM) $(KERNEL_ELF) | less

disasm-vim: $(KERNEL_ELF)
	@$(OBJDUMP) $(DISASM) $(KERNEL_ELF) > $(DISASM_TMP)
	@vim $(DISASM_TMP)
	@rm $(DISASM_TMP)

run: run-inner
	@echo test----finished----------------


run-inner: build
	@echo test----started-----------------
ifeq ($(BOARD),qemu)
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios $(BOOTLOADER) \
		-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA)
else
	(which $(K210-BURNER)) || (cd .. && git clone https://github.com/sipeed/kflash.py.git && mv kflash.py tools)
	@cp $(BOOTLOADER) $(BOOTLOADER).copy
	@dd if=$(KERNEL_BIN) of=$(BOOTLOADER).copy bs=$(K210_BOOTLOADER_SIZE) seek=1
	@mv $(BOOTLOADER).copy $(KERNEL_BIN)
	@sudo chmod 777 $(K210-SERIALPORT)
	python3 $(K210-BURNER) -p $(K210-SERIALPORT) -b 1500000 $(KERNEL_BIN)
	python3 -m serial.tools.miniterm --eol LF --dtr 0 --rts 0 --filter direct $(K210-SERIALPORT) 115200
endif

debug: build
ifeq ($(BOARD),qemu)
	@echo debug---started-----------------
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios $(BOOTLOADER) \
		-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA) \
		-s -S
	@echo debug---finished----------------
endif

tree:
	@tree -I target
	@cd ../user && make tree
tree-all:
	@tree
	@cd ../user && make tree-all

.PHONY: tree tree-all app-bin debug run build env clean disasm disasm-vim run-inner switch-check
