ARCH_DIR 		:= $(SRC_DIR)/arch/$(ARCH)


ASM_FILES		:= $(wildcard $(ARCH_DIR)/*.s)
ASM_OBJECT 	:= $(patsubst $(ARCH_DIR)/%.s, \
	$(BUILD_DIR)/%.o, $(ASM_FILES))

LD_SCRIPT		:= $(ARCH_DIR)/linker.ld

LD					:= x86_64-elf-ld
LD_FLAGS		:= -n -T $(LD_SCRIPT) -o $(KERNEL) $(ASM_OBJECT)

all: $(KERNEL)

clean:
	@rm -rf build

$(KERNEL): $(ASM_OBJECT) $(LD_SCRIPT)
	@$(LD) $(LD_FLAGS)

$(BUILD_DIR)/%.o: $(ARCH_DIR)/%.s
	@mkdir -p $(shell dirname $@)  # Make the build dir.
	@nasm -felf64 $< -o $@
