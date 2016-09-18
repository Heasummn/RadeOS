launch_longmode:
  lgdt [gdt64.pointer]
  
  ; Update selectors
  mov ax, gdt64.data
  mov ss, ax
  mov ds, ax
  mov es, ax

  ; Far jump and start long mode!
  jmp gdt64.code:longmode_start


section .rodata
gdt64:
  .empty:
    dq 0 ; Empty entry
  .code: equ $ - gdt64
    ; Code segment
    dq (1 << descriptor_type) | (1 << present) | (1 << read_write) | (1 << executable) | (1 << code_seg)
  .data: equ $ - gdt64
    ; Data segment
    dq (1 << descriptor_type) | (1 << present) | (1 << read_write)
  ; Pointer to GDT, fitting the pointer description 
  .pointer:
    dw $ - gdt64 - 1
    dq gdt64