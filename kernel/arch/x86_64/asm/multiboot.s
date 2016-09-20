  ;; multiboot.s
  ;; The code containing the multiboot header.

%define Magic 0xe85250d6  ; magic number (multiboot 2)
%define Arch  0           ; architecture 0 (protected mode i386)

section .multiboot_header
header_start:
  dd Magic               
  dd Arch                 
  dd header_end - header_start ; header length
  ;; checksum
  dd 0x100000000 - (Magic + Arch + (header_end - header_start))

  ;; insert optional multiboot tags here

  ;; required end tag
  dw 0    ; type
  dw 0    ; flags
  dd 8    ; size
header_end:
