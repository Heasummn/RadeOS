ARCH_DIR 		:= $(SRC_DIR)/arch/$(ARCH)

ASM_FILES		:= $(wildcard $(ARCH_DIR)/asm/*.s)
ASM_OBJECT 		:= $(patsubst $(ARCH_DIR)/%.s, \
	$(BUILD_DIR)/%.o, $(ASM_FILES))

LD_SCRIPT		:= $(ARCH_DIR)/linker.ld

LD				:= $(ARCH)-elf-ld
TARGET			:= $(ARCH)-unknown-linux-gnu
RUST_OS 		:= target/$(TARGET)/debug/lib$(SNAKE_NAME).a
LD_FLAGS		:= -n --gc-sections -T $(LD_SCRIPT) -o $(KERNEL) $(ASM_OBJECT) $(RUST_OS)


all: $(KERNEL)

clean:
	@rm -rf build target

$(KERNEL): cargo $(RUST_OS) $(ASM_OBJECT) $(LD_SCRIPT)
	@$(LD) $(LD_FLAGS)

cargo:
	@xargo build --target $(TARGET)

$(BUILD_DIR)/%.o: $(ARCH_DIR)/%.s
	@mkdir -p $(shell dirname $@)  # Make the build dir.
	@nasm -felf64 $< -o $@
