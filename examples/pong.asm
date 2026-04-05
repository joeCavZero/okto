@include "okto.h"

.color
    # r g b a
    .byte 0, 0, 0, 0 # transparent
    .byte 0, 0, 0, 255 # black
    a:.byte 127, 127, 127, 0b0000_1111 # gray
.palette
    PALETTE_PLAYER: .double 0x0234 # 1, 2, 3, 4
.code
loop:
    lla $a, a
    li $c, OKTO_PRINTLN_UNSIGNED
    call

    #la loop
    #swpx
    #jmp

END:
    li $c, OKTO_EXIT
    call