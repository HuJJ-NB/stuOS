TARGET := riscv64gc-unknown-none-elf
MODE := release
APP_DIR := src/bin
TARGET_DIR := target/$(TARGET)/$(MODE)
BUILD_DIR := build
OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64
PY := python3

TEST ?= 0
ifeq ($(TEST), 0)
	APPS :=  $(filter-out $(wildcard $(APP_DIR)/test*.rs), $(wildcard $(APP_DIR)/*.rs))
else
	APPS :=  $(wildcard $(APP_DIR)/test$(TEST)*.rs)
endif
ELFS := $(patsubst $(APP_DIR)/%.rs, $(TARGET_DIR)/%, $(APPS))

elf:
	@$(foreach app, $(APPS),  echo ----find $(app); echo ------build $(patsubst $(TARGET_DIR)/%, $(TARGET_DIR)/%, $(app));)
	@cargo build --release

binary: pre
	@echo $(APPS)
	@cargo build --release
	@$(foreach elf, $(ELFS), \
		$(OBJCOPY) $(elf) --strip-all -O binary $(patsubst $(TARGET_DIR)/%, $(TARGET_DIR)/%.bin, $(elf)); \
		cp $(elf) $(patsubst $(TARGET_DIR)/%, $(TARGET_DIR)/%.elf, $(elf));)

pre: clean
	@mkdir -p $(BUILD_DIR)/bin/
	@mkdir -p $(BUILD_DIR)/elf/
	@mkdir -p $(BUILD_DIR)/app/
	@$(foreach t, $(APPS), cp $(t) $(BUILD_DIR)/app/;)

build: binary
	@$(foreach t, $(ELFS), cp $(t).bin $(BUILD_DIR)/bin/;)
	@$(foreach t, $(ELFS), cp $(t).elf $(BUILD_DIR)/elf/;)

clean:
	@cargo clean
ifeq ($(BUILD_DIR) , $(wildcard $(BUILD_DIR)))
	@rm -rf $(BUILD_DIR)
endif

tree:
	@tree -I "target|build"
tree-all:
	@tree

.PHONY: pre clean build binary elf tree tree-all
