@include "okto.h"

# ==== STATICS ====
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

# ==== AUXILIARY MACROS ====
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

# ==== CONSTS ====
@macro BALL_RADIUS 4
@macro BALL_MIN_Y 8
@macro BALL_MAX_Y 240

@macro BALL_X_SPEED 1
@macro BALL_X_SPEED_NEGATIVE 254

@macro PADDLE_SPEED 3

@macro PADDLE_MIN_Y 8
@macro PADDLE_MAX_Y 224

@macro PADDLE_HEIGHT 24
@macro PADDLE_HALF_HEIGHT 12

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
    li $b, BALL_X_SPEED_NEGATIVE
    st $b, $c

    li $c, BALL_DY_PTR
    li $b, BALL_X_SPEED
    st $b, $c

    li $c, BALL_X_PTR
    li $b, 127
    st $b, $c

    li $c, BALL_Y_PTR
    li $b, 127
    st $b, $c

    li $c, BALL_RADIUS_PTR
    li $b, BALL_RADIUS
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
    li $b, PADDLE_HEIGHT
    st $b, $c

    li $c, PADDLE_1_COLOR_PTR
    li $b, COLOR_WHITE
    st $b, $c

  PROGRAM_STATIC_INIT_PADDLE_2:
    li $c, PADDLE_2_X_PTR
    li $b, 240
    st $b, $c

    li $c, PADDLE_2_Y_PTR
    li $b, 8
    st $b, $c

    li $c, PADDLE_2_WIDTH_PTR
    li $b, 8
    st $b, $c

    li $c, PADDLE_2_HEIGHT_PTR
    li $b, PADDLE_HEIGHT
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
    li $a, 5
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
        UPDATE_BALL_COLLISION:
            UPDATE_BALL_COLLISION_TOP:
                la UPDATE_BALL_COLLISION_TOP_NO_COLLISION
                swpx

                li $a, BALL_Y_PTR
                ld $a, $a
                li $b, BALL_RADIUS
                sub                  # a := ball.y - ball.radius

                li $b, BALL_MIN_Y
                jgt                  # if (ball.y - ball.radius) > BALL_MIN_Y

                li $c, BALL_DY_PTR
                li $a, BALL_X_SPEED
                st $a, $c

                UPDATE_BALL_COLLISION_TOP_NO_COLLISION:

            UPDATE_BALL_COLLISION_FLOOR:
                la UPDATE_BALL_COLLISION_FLOOR_NO_COLLISION
                swpx

                li $a, BALL_Y_PTR
                ld $a, $a
                li $b, BALL_RADIUS
                add                  # a := ball.y + ball.radius

                li $b, BALL_MAX_Y
                jlt                  # if (ball.y + ball.radius) < BALL_MAX_Y

                li $c, BALL_DY_PTR
                li $a, BALL_X_SPEED_NEGATIVE
                st $a, $c

                UPDATE_BALL_COLLISION_FLOOR_NO_COLLISION:


            UPDATE_BALL_COLLISION_PADDLE_1:
                # x - - - - - - - -
                la UPDATE_BALL_COLLISION_PADDLE_1_NO_COLLISION_X_GT
                swpx

                li $c, PADDLE_1_X_PTR
                ld $a, $c        # b = p1_x

                li $c, PADDLE_1_WIDTH_PTR
                ld $b, $c
                add              # a = p1_x + p1_width

                mv $c, $a # c := (p1.x + p1.w)

                li $a, BALL_X_PTR
                ld $a, $a        # a = ball_x
                li $b, BALL_RADIUS
                sub # a = (ball.x - ball.radius)
                
                mv $b, $c

                jgt # if (ball.x - ball.radius) > (p1.x + p1.w)

                # - - - - - - - -
                la UPDATE_BALL_COLLISION_PADDLE_1_NO_COLLISION_X_LT
                swpx

                li $c, PADDLE_1_X_PTR
                ld $c, $c        # c = p1.x

                li $a, BALL_X_PTR
                ld $a, $a        # a = ball_x
                li $b, BALL_RADIUS
                add # a = (ball.x + ball.radius)
                
                mv $b, $c

                jlt # if (ball.x - ball.radius) < (p1.x)

                # y - - - - - - - -
                la UPDATE_BALL_COLLISION_PADDLE_1_NO_COLLISION_Y_GT
                swpx

                li $c, PADDLE_1_Y_PTR
                ld $a, $c        # b := p1.y

                li $c, PADDLE_1_HEIGHT_PTR
                ld $b, $c
                add              # a := p1.y + p1.h

                mv $c, $a # c := (p1.y + p1.h)

                li $a, BALL_Y_PTR
                ld $a, $a        # b = ball.y
                li $b, BALL_RADIUS
                sub # a = (ball.y - ball.radius)
                
                mv $b, $c

                jgt # if (ball.y - ball.radius) > (p1.y + p1.h)
                
                # - - - - - - - -
                la UPDATE_BALL_COLLISION_PADDLE_1_NO_COLLISION_Y_LT
                swpx

                li $c, PADDLE_1_Y_PTR
                ld $c, $c        # c := p1.y

                li $a, BALL_Y_PTR
                ld $a, $a        # a = ball.y
                li $b, BALL_RADIUS
                add # a = (ball.y + ball.radius)
                
                mv $b, $c

                jlt # if (ball.y + ball.radius) > (p1.y)

                # ==== COLISÃO DETECTADA ====
                li $c, BALL_DX_PTR
                li $a, BALL_X_SPEED
                st $a, $c

                la UPDATE_BALL_COLLISION_PADDLE_1_AUX_FUNC
                swpx

                li $c, OKTO_RANDOM
                call
                # a := random()
                li $b, 0b_0000_0011
                and
                # a := 0~3
                mv $b, $a

                li $c, OKTO_RANDOM
                call
                # a := random()
                mv $c, $b # c := 0~3
                li $b, 0b1
                and
                # a := 1 ou 0

                mv $b, $c # b := 0~3

                mv $c, $a # c := 1 ou 0
                li $a, 0
                sub # a := ( 0 - [0~3] )
                # a = 253~3 ; b = 0~3 ; c = 1/0
                swpf # b <-> f ; f := 0~3
                mv $b, $a # b := 253~3
                mv $a, $c # a := 1/0
                mv $c, $b # c := 253~3
                
                li $b, 0b1
                # a = 1/0 ; b = 1
                jeq
                # else
                swpf # b <-> f ; b = 0~3
                li $a, 0
                # a = 0 ; b = 0~3
                add
                mv $c, $a
              UPDATE_BALL_COLLISION_PADDLE_1_AUX_FUNC:
                li $b, BALL_DY_PTR
                st $c, $b

                UPDATE_BALL_COLLISION_PADDLE_1_NO_COLLISION_X_GT:
                UPDATE_BALL_COLLISION_PADDLE_1_NO_COLLISION_X_LT:
                UPDATE_BALL_COLLISION_PADDLE_1_NO_COLLISION_Y_GT:
                UPDATE_BALL_COLLISION_PADDLE_1_NO_COLLISION_Y_LT:

            UPDATE_BALL_COLLISION_PADDLE_2:
                # x - - - - - - - -
                la UPDATE_BALL_COLLISION_PADDLE_2_NO_COLLISION_X_GT
                swpx

                li $c, PADDLE_2_X_PTR
                ld $a, $c        # a = p2_x

                li $c, PADDLE_2_WIDTH_PTR
                ld $b, $c
                add              # a = p2_x + p2_width

                mv $c, $a        # c := (p2.x + p2.w)

                li $a, BALL_X_PTR
                ld $a, $a        # a = ball_x
                li $b, BALL_RADIUS
                sub              # a = (ball.x - ball.radius)

                mv $b, $c

                jgt              # if (ball.x - ball.radius) > (p2.x + p2.w)

                # - - - - - - - -
                la UPDATE_BALL_COLLISION_PADDLE_2_NO_COLLISION_X_LT
                swpx

                li $c, PADDLE_2_X_PTR
                ld $c, $c        # c = p2.x

                li $a, BALL_X_PTR
                ld $a, $a        # a = ball_x
                li $b, BALL_RADIUS
                add              # a = (ball.x + ball.radius)

                mv $b, $c

                jlt              # if (ball.x + ball.radius) < (p2.x)

                # y - - - - - - - -
                la UPDATE_BALL_COLLISION_PADDLE_2_NO_COLLISION_Y_GT
                swpx

                li $c, PADDLE_2_Y_PTR
                ld $a, $c        # a := p2.y

                li $c, PADDLE_2_HEIGHT_PTR
                ld $b, $c
                add              # a := p2.y + p2.h

                mv $c, $a        # c := (p2.y + p2.h)

                li $a, BALL_Y_PTR
                ld $a, $a        # a = ball.y
                li $b, BALL_RADIUS
                sub              # a = (ball.y - ball.radius)

                mv $b, $c

                jgt              # if (ball.y - ball.radius) > (p2.y + p2.h)

                # - - - - - - - -
                la UPDATE_BALL_COLLISION_PADDLE_2_NO_COLLISION_Y_LT
                swpx

                li $c, PADDLE_2_Y_PTR
                ld $c, $c        # c := p2.y

                li $a, BALL_Y_PTR
                ld $a, $a        # a = ball.y
                li $b, BALL_RADIUS
                add              # a = (ball.y + ball.radius)

                mv $b, $c

                jlt              # if (ball.y + ball.radius) < (p2.y)

                # ==== COLISÃO DETECTADA ====
                li $c, BALL_DX_PTR
                li $a, BALL_X_SPEED_NEGATIVE
                st $a, $c

                la UPDATE_BALL_COLLISION_PADDLE_2_AUX_FUNC
                swpx

                li $c, OKTO_RANDOM
                call
                # a := random()
                li $b, 0b_0000_0011
                and
                # a := 0~3
                mv $b, $a

                li $c, OKTO_RANDOM
                call
                # a := random()
                mv $c, $b        # c := 0~3
                li $b, 0b1
                and
                # a := 1 ou 0

                mv $b, $c        # b := 0~3

                mv $c, $a        # c := 1 ou 0
                li $a, 0
                sub              # a := ( 0 - [0~3] )
                # a = 253~3 ; b = 0~3 ; c = 1/0
                swpf             # b <-> f ; f := 0~3
                mv $b, $a        # b := 253~3
                mv $a, $c        # a := 1/0
                mv $c, $b        # c := 253~3

                li $b, 0b1
                # a = 1/0 ; b = 1
                jeq
                # else
                swpf             # b <-> f ; b = 0~3
                li $a, 0
                # a = 0 ; b = 0~3
                add
                mv $c, $a
              UPDATE_BALL_COLLISION_PADDLE_2_AUX_FUNC:
                li $b, BALL_DY_PTR
                st $c, $b

                UPDATE_BALL_COLLISION_PADDLE_2_NO_COLLISION_X_GT:
                UPDATE_BALL_COLLISION_PADDLE_2_NO_COLLISION_X_LT:
                UPDATE_BALL_COLLISION_PADDLE_2_NO_COLLISION_Y_GT:
                UPDATE_BALL_COLLISION_PADDLE_2_NO_COLLISION_Y_LT:
        UPDATE_BALL_MOVEMENT:
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
        # if up
        li $c, OKTO_INPUT
        li $a, 1
        call

        mv $c, $a
        la UPDATE_PADDLE_1_MOVEMENT_UP
        swpx
        mv $a, $c
        li $b, 0b0001_0000 
        and # a := a <and> b   
        jeq

        # if down
        li $c, OKTO_INPUT
        li $a, 1
        call

        mv $c, $a
        la UPDATE_PADDLE_1_MOVEMENT_DOWN
        swpx
        mv $a, $c
        li $b, 0b0010_0000 
        and # a := a <and> b   
        jeq

        # else
        la UPDATE_PADDLE_1_MOVEMENT_END
        swpx
        jmp

        UPDATE_PADDLE_1_MOVEMENT_UP:
            li $c, PADDLE_1_Y_PTR
            ld $a, $c
            li $b, PADDLE_SPEED
            sub

            # if a > b: goto Y_LT_MIN_PADDLE_Y
            mv $c, $a # c := a
            la UPDATE_PADDLE_1_MOVEMENT_UP_Y_GT_MIN_PADDLE_Y
            swpx
            mv $a, $c
            li $b, PADDLE_MIN_Y
            jgt
            # else:
            li $a, PADDLE_MIN_Y
          UPDATE_PADDLE_1_MOVEMENT_UP_Y_GT_MIN_PADDLE_Y:
            li $c, PADDLE_1_Y_PTR
            st $a, $c

            la UPDATE_PADDLE_1_MOVEMENT_END
            swpx
            jmp
        UPDATE_PADDLE_1_MOVEMENT_DOWN:
            li $c, PADDLE_1_Y_PTR
            ld $a, $c
            li $b, PADDLE_SPEED
            add

            # if a < b: goto Y_GT_MAX_PADDLE_Y
            mv $c, $a # c := a
            la UPDATE_PADDLE_1_MOVEMENT_DOWN_Y_LT_MAX_PADDLE_Y
            swpx
            mv $a, $c
            li $b, PADDLE_MAX_Y
            jlt
            # else:
            li $a, PADDLE_MAX_Y
          UPDATE_PADDLE_1_MOVEMENT_DOWN_Y_LT_MAX_PADDLE_Y:
            li $c, PADDLE_1_Y_PTR

            st $a, $c

            la UPDATE_PADDLE_1_MOVEMENT_END
            swpx
            jmp
        UPDATE_PADDLE_1_MOVEMENT_END:

    UPDATE_PADDLE_2:
        li $c, BALL_Y_PTR
        ld $a, $c

        li $b, PADDLE_HALF_HEIGHT
        sub
        
        li $c, PADDLE_2_Y_PTR

        st $a, $c
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