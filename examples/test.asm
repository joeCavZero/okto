@include "okto.h"
.code
    lchr $a, 'p'
    li $c, OKTO_PRINTLN_CHAR
    call

    la EXIT
        li $c, OKTO_PRINTLN_DOUBLE_UNSIGNED
        call
    swpx
    jmp
    nope
a:
    li $a, 111
    li $c, OKTO_PRINTLN_UNSIGNED
    call
EXIT:
    li $c, OKTO_EXIT
    call
