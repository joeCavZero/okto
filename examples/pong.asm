@include "okto.h"


# BALL
@macro BALL_PTR 250
  @macro BALL_DX_PTR 250
  @macro BALL_DY_PTR 251

 @macro BALL_CIRCLE_PTR 252
  @macro BALL_X_PTR 252
  @macro BALL_Y_PTR 253
  @macro BALL_RADIUS_PTR 254
  @macro BALL_COLOR_PTR 255

# PADDLE 1
@macro PADDLE_1_PTR 245
 @macro PADDLE_1_RECT_PTR 245
  @macro PADDLE_1_X_PTR 245
  @macro PADDLE_1_Y_PTR 246
  @macro PADDLE_1_WIDTH_PTR 247
  @macro PADDLE_1_HEIGHT_PTR 248
  @macro PADDLE_1_COLOR_PTR 249

# PADDLE 2
@macro PADDLE_2_PTR 240
 @macro PADDLE_2_RECT_PTR 240
  @macro PADDLE_2_X_PTR 240
  @macro PADDLE_2_Y_PTR 241
  @macro PADDLE_2_WIDTH_PTR 242
  @macro PADDLE_2_HEIGHT_PTR 243
  @macro PADDLE_2_COLOR_PTR 244

@macro PROGRAM_STATIC_BASE 239

# =====================
@macro INC_SP(%a) \
    mv $a, $sp \
    li $b, %a \
    add \
    mv $sp, $a

@macro DEC_SP(%a) \
    mv $a, $sp \
    li $b, %a \
    sub \
    mv $sp, $a

.color
    # r g b a
    .byte 0, 0, 0, 0 @macro COLOR_TRANSPARENT 0
    .byte 0, 0, 0, 255 @macro COLOR_BLACK 1
    .byte 127, 127, 127, 255 @macro COLOR_GRAY 2
    .byte 255, 255, 255, 255 @macro COLOR_WHITE 3

.code
PROGRAM_STATIC_INIT:
  PROGRAM_STATIC_INIT_BALL:
    li $c, BALL_DX_PTR
    li $b, 1
    st $b, $c

    li $c, BALL_DY_PTR
    li $b, 1
    st $b, $c

    li $c, BALL_X_PTR
    li $b, 127
    st $b, $c

    li $c, BALL_Y_PTR
    li $b, 127
    st $b, $c

    li $c, BALL_RADIUS_PTR
    li $b, 4
    st $b, $c

    li $c, BALL_COLOR_PTR
    li $b, COLOR_WHITE
    st $b, $c

  PROGRAM_STATIC_INIT_PADDLE_1:
    li $c, PADDLE_1_X_PTR
    li $b, 8
    st $b, $c

    li $c, PADDLE_1_Y_PTR
    li $b, 8
    st $b, $c

    li $c, PADDLE_1_WIDTH_PTR
    li $b, 8
    st $b, $c

    li $c, PADDLE_1_HEIGHT_PTR
    li $b, 24
    st $b, $c

    li $c, PADDLE_1_COLOR_PTR
    li $b, COLOR_WHITE
    st $b, $c

  PROGRAM_STATIC_INIT_PADDLE_2:
    li $c, PADDLE_2_X_PTR
    li $b, 232
    st $b, $c

    li $c, PADDLE_2_Y_PTR
    li $b, 8
    st $b, $c

    li $c, PADDLE_2_WIDTH_PTR
    li $b, 8
    st $b, $c

    li $c, PADDLE_2_HEIGHT_PTR
    li $b, 24
    st $b, $c

    li $c, PADDLE_2_COLOR_PTR
    li $b, COLOR_WHITE
    st $b, $c


li $sp, PROGRAM_STATIC_BASE

MAIN:

    la GAME_LOOP
    swpx
    jmp

    la END
    swpx
    jmp

GAME_LOOP:
    la UPDATE
    swpx
    jmp

    la RENDER
    swpx
    jmp

    li $c, OKTO_SLEEP
    li $a, 2
    call

    la GAME_LOOP
    swpx
    jmp

UPDATE:
    swpx
    st $a, $sp
    mv $c, $b
    DEC_SP(1)
    st $c, $sp

    UPDATE_BALL:
        li $c, BALL_DX_PTR
        ld $a, $c
        li $c, BALL_X_PTR
        ld $b, $c
        add
        st $a, $c

        li $c, BALL_DY_PTR
        ld $a, $c
        li $c, BALL_Y_PTR
        ld $b, $c
        add
        st $a, $c
    
    UPDATE_PADDLE_1:
        li $c, OKTO_INPUT
        li $a, 1
        call

        # if up
        mv $c, $a
        la UPDATE_PADDLE_1_MOVEMENT_UP
        swpx
        mv $a, $c
        li $b, 0b0001_0000 
        and # a := a <and> b   
        jeq

        # else
        la UPDATE_PADDLE_1_MOVEMENT_END
        swpx
        jmp

        UPDATE_PADDLE_1_MOVEMENT_UP:
            li $c, PADDLE_1_Y_PTR
            ld $a, $c
            li $b, 1
            sub
            st $a, $c

            la UPDATE_PADDLE_1_MOVEMENT_END
            swpx
            jmp
        UPDATE_PADDLE_1_MOVEMENT_DOWN:
            la UPDATE_PADDLE_1_MOVEMENT_END
            swpx
            jmp
        UPDATE_PADDLE_1_MOVEMENT_END:

    ld $c, $sp
    INC_SP(1)
    ld $a, $sp
    mv $b, $c
    swpx
    jmp

RENDER:
    swpx
    st $a, $sp
    mv $c, $b
    DEC_SP(1)
    st $c, $sp

        li $c, OKTO_CLEAR
        call

        RENDER_BALL:
            li $a, BALL_CIRCLE_PTR
            li $c, OKTO_RENDER_CIRCLE
            call
        RENDER_PADDLE_1:
            li $a, PADDLE_1_RECT_PTR
            li $c, OKTO_RENDER_RECT
            call
        RENDER_PADDLE_2:
            li $a, PADDLE_2_RECT_PTR
            li $c, OKTO_RENDER_RECT
            call
        li $c, OKTO_PRESENT
        call

    ld $c, $sp
    INC_SP(1)
    ld $a, $sp
    mv $b, $c
    swpx
    jmp


END:
    li $c, OKTO_EXIT
    call