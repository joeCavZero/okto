.color
    # r g b a
    .byte 0, 0, 0, 0 # transparent
    .byte 0, 0, 0, 255 # black
    .byte 127, 127, 127, 255 # gray
.palette
    PALETTE_PLAYER: .double 0x1234 # 1, 2, 3, 4