@include "okto.h"

.color
    # r g b a
    .byte 0, 0, 0, 0 @macro COLOR_TRANSPARENT 0
    .byte 0, 0, 0, 255 @macro COLOR_BLACK 1
    .byte 127, 127, 127, 255 @macro COLOR_GRAY 2
    .byte 255, 255, 255, 255 @macro COLOR_WHITE 3
.palette
    PALETTE_PLAYER: .double 0x0234 # 1, 2, 3, 4
.code
    li $c, 90
    li $b, 10
    st $b, $c

    li $c, 91
    li $b, 20
    st $b, $c

    li $c, 92
    li $b, 255
    st $b, $c

    li $c, 93
    li $b, 255
    st $b, $c

    li $c, 94
    li $b, 100
    st $b, $c

    li $c, 95
    li $b, COLOR_WHITE
    st $b, $c

loop:

    li $c, 90
    ld $a, $c
    li $b, 1
    add
    st $a, $c

    li $c, OKTO_CLEAR
    call

    li $a, 90
    li $c, OKTO_RENDER_LINE
    call

    li $c, OKTO_PRESENT
    call

    la loop
    swpx
    jmp
    
END:
    li $c, OKTO_EXIT
    call