global start

%define multiboot_load 0x36d76289
%define cpuid_longmode_check 0x80000000
%define longmode_age 0x80000001
%define longmode_bit 1 << 29
%define PAE_bit 1 << 5
%define longmode_bit_efer 1 << 8
%define paging_bit 1 << 31

section 32
bits 32
start:
  mov esp, stack_top ; Update the stack pointer
  
  ; Run checks to ensure we can do what we are doing
  call multiboot_check
  call cpuid_check
  call longmode_check

  ; Enable paging
  call setup_page_tables
  call enable_paging

  hlt


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


setup_page_tables:
  ; Set first P4 entry to P3 table
  mov eax, p3_table
  or eax, 3 ; 3 == 0b01 | 0b10. Set read and present
  mov [p4_table], eax

  ; Set first P3 entry to point to P2 table
  mov eax, p2_table
  or eax, 3
  mov [p3_table], eax

  ; TODO: Map each P2 entry to a page
  ; To do this, we have to loop!

  mov ecx, 0 ; A counter to count how many pages we've seen
  .map_page_loop:
    mov eax, 0x200000 ; 2 MiB
    mul ecx ; Get the current start address of the page
    or eax, 0b10000011 ; present + write + huge
    mov [p2_table + (ecx * 8)], eax ; Since each entry is 8 bits, get our current entry

    ; Loop induction stage
    inc ecx
    cmp ecx, 512 ; Each page has 512 entries
    jne .map_page_loop

    ret

enable_paging:
  mov eax, p4_table
  mov cr3, eax  ; For some weird reason, we can't load P4 directly to cr3, so we use a temp

  ; Enable PAE
  mov eax, cr4
  or eax, PAE_bit
  mov cr4, eax

  ; set the long mode bit in the EFER MSR (model specific register)
  mov ecx, 0xC0000080
  rdmsr
  or eax, longmode_bit_efer
  wrmsr

  ; Actually enable paging
  mov eax, cr0
  or eax, paging_bit
  mov cr0, eax

  ret

; Print a nice red on white ERROR: and it's argument
; Uses argument from al
error:
  ; 4f == RED on white
  mov dword [0xb8000], 0x4f524f45 ; 0x45 = E, 0x52 = R
  mov dword [0xb8004], 0x4f4f4f52 ; 0x52 = R, 0x4f = O
  mov dword [0xb8008], 0x4f3a4f52 ; 0x52 = R, 0x3a = :
  mov word  [0xb800c], 0x4f20     ; 0x20 = ' '
  mov byte  [0xb800e], al         ; Put parameter byte
  mov byte  [0xb800f], 0x4f
  hlt

section .bss
align 4096
p4_table:
  resb 4096
p3_table:
  resb 4096
p2_table:
  resb 4096
stack_bottom: ; The stack grows downwardsh, so the bottom is actually at the top
  resb 64
stack_top:
