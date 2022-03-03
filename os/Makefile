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

# KERNEL ENTRY
KERNEL_ENTRY_PA := 0x80200000

# Binutils
OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64

# Disassembly
DISASM ?= -x

build: env $(KERNEL_BIN)

env:
	@echo Check if target $(TARGET) has been installed.
	@(rustup target list | grep "$(TARGET) (installed)") || rustup target add $(TARGET)
	cargo install cargo-binutils --vers =0.3.3
	rustup component add rust-src
	rustup component add llvm-tools-preview

$(KERNEL_BIN): $(KERNEL_ELF)
	@echo make binary file.
	@$(OBJCOPY) $(KERNEL_ELF) --strip-all -O binary $@

$(KERNEL_ELF): app $(LINKER)
	@echo --build ruianOS.
	@echo ----Platform: $(BOARD)
	cargo build --release $(FLAGS)
	@rm $(LINKED_APP_BINS)

$(LINKER): $(LINKER_SRC_PATH)/linker-head-$(BOARD).ld $(LINKER_SRC_PATH)/linker-body.ld
	@echo make new linker file.
	@cat $(LINKER_SRC_PATH)/linker-head-$(BOARD).ld $(LINKER_SRC_PATH)/linker-body.ld > $(LINKER)

app:
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

run: build
	@echo test----started-----------------
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios $(BOOTLOADER) \
		-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA)
	@echo test----finished----------------

debug: build
	@echo debug---started-----------------
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios $(BOOTLOADER) \
		-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA) \
		-s -S
	@echo debug---finished----------------

tree:
	@tree -I target
	@cd ../user && make tree
	
tree-all:
	@tree -a
	@cd ../user && make tree-all

.PHONY: tree tree-all app debug run build env clean disasm disasm-vim
