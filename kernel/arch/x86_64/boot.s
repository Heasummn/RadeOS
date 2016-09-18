global start
extern longmode_start

%define multiboot_load 0x36d76289
%define cpuid_longmode_check 0x80000000
%define longmode_age 0x80000001
%define longmode_bit 1 << 29
%define PAE_bit 1 << 5
%define longmode_bit_efer 1 << 8
%define paging_bit 1 << 31

;; GDT constants
%define read_write 41
%define executable 43
%define descriptor_type 44
%define present 47
%define code_seg 53

section 32
bits 32
start:
  mov esp, stack_top ; Update the stack pointer
  
  ; Run checks to ensure we can do what we are doing1
  call multiboot_check
  call cpuid_check
  call longmode_check

  ; Enable paging
  call setup_page_tables
  call enable_paging

  jmp launch_longmode
  
; Include the different stages of booting
%include "kernel/arch/x86_64/boot_checks.s.inc"
%include "kernel/arch/x86_64/paging.s.inc"
%include "kernel/arch/x86_64/gdt.s.inc"

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

stack_bottom: ; The stack grows downwards, so the bottom is actually at the top
  resb 64
stack_top:

