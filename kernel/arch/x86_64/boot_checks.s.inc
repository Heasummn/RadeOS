
; Make sure the kernel was loaded by a multiboot compliant bootloader
multiboot_check:
  ; The specification says that the "multiboot_load" value defined above
  ;   must be within eax
  cmp eax, multiboot_load
  jne .No_Multiboot
  ret

.No_Multiboot:
  mov al, "1"
  jmp error


; Check that the cpuid command is available
cpuid_check:
  ; CPUID is supported if we can invert the ID bit of EFLAGS

  pushfd  ; Push EFLAGS onto stack
  pop eax ; Copy EFLAGS into eax

  mov ecx, eax ; Copy into ecx to store the old version

  ; Invert the ID bit in eax, but not in EFLAGS
  xor eax, 0x200000

  ; Copy inverted eax back into EFLAGS
  push eax
  popfd

  ; Now, if we restore EFLAGS, and the ID bit is inverted, then CPUID is available
  ; So let's do exactly that. Store EFLAGS into eax again 
  pushfd
  pop eax

  ; Restore the ID bit, by using the copy in ecx
  ; Doing all this was just a test, we don't want to actually invert the ID bit
  push ecx
  popfd

  ; If eax hasn't been changed, then the bit wasn't flipped, and CPUID isn't available
  cmp eax, ecx
  jz .No_CPUID
  ret

.No_CPUID:
  mov al, "2"
  jmp error


; Make sure we have longmode available
longmode_check:
  mov eax, cpuid_longmode_check ; Set A-register to cpuid longmode check magic
  cpuid
  cmp eax, longmode_age       ; CPU is too old if eax is < than longmode_age
  jb .No_Longmode

  ; If our CPU is old enough to boot into longmode, let's check if we have longmode
  mov eax, longmode_age
  cpuid
  test edx, longmode_bit ; Check if the long mode bit is set
  jz .No_Longmode
  ret

.No_Longmode:
  mov al, "3"
  jmp error