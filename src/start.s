   .text
   .align  4
   .globl  _start
   .hidden _start
   .section .text._start
   .type   _start,@function
   _start:
       # Preserve the kernel-provided initial stack pointer across calls.
       # r12 is callee-saved by the System V x86_64 ABI.
       mov     %rsp, %r12

       # The stack must be 16-byte aligned immediately before a call.
       and     $-16, %rsp
       xor     %ebp, %ebp

       # Initialize BSS section to zero
       # bss_start and bss_end are provided by the linker script
       mov     $_bss_start, %rax
       mov     $_bss_end, %rcx
       cmp     %rcx, %rax
       je      bss_init_done

bss_zero_loop:
       movq    $0, (%rax)
       add     $8, %rax
       cmp     %rcx, %rax
       jb      bss_zero_loop

bss_init_done:
       # Run optional process initialization before handing off to Rust.
       call    flag_license

       # Pass the untouched Linux initial stack to the application entry point.
       mov     %r12, %rdi
       call    entry

       # entry has return type `!`; reaching here indicates a contract violation.
       ud2
