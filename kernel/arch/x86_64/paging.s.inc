; Setup the page tables, as the name implies
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

; Enable paging, as the name implies
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