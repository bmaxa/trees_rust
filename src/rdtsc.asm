format elf64

public _rdtsc

section '.text' executable

_rdtsc:
    rdtsc
    shl rdx,32
    or rax,rdx
    ret
    
