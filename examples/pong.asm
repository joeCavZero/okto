@include "okto.h"

# ==== CONSTS ====

@macro BALL_RADIUS 5
@macro BALL_START_X 127
@macro BALL_START_Y 127
@macro BALL_SPEED_POSITIVE 1
@macro BALL_SPEED_NEGATIVE 255

@macro SCREEN_TOP 8
@macro SCREEN_BOTTOM 248
@macro SCREEN_LEFT 8
@macro SCREEN_RIGHT 248

@macro PADDLE_W 6
@macro PADDLE_H 24
@macro PADDLE_HALF_H 12
@macro PADDLE_SPEED 3
@macro PADDLE_MIN_Y 8
@macro PADDLE_MAX_Y 224

@macro PADDLE_1_START_X 12
@macro PADDLE_1_START_Y 112

@macro PADDLE_2_START_X 238
@macro PADDLE_2_START_Y 112

@macro INPUT_UP 0b0001_0000
@macro INPUT_DOWN 0b0010_0000
@macro INPUT_LEFT  0b0100_0000
@macro INPUT_RIGHT 0b1000_0000

@macro LINE_W 4

.color
    .byte 0, 0, 0, 0          @macro COLOR_TRANSPARENT 0
    .byte 0, 0, 0, 255        @macro COLOR_BLACK 1
    .byte 127, 127, 127, 255  @macro COLOR_GRAY 2
    .byte 255, 255, 255, 255  @macro COLOR_WHITE 3
    .byte 200, 255, 200, 255  @macro COLOR_GREEN 4
    .byte 255, 200, 200, 255  @macro COLOR_RED 5


@macro PUSH_VALUE(%val) \
  decsp \
  li $c, %val \
  st $c, $sp 

@macro PUSH_REG(%reg) \
  decsp \
  st %reg, $sp 

@macro POP_REG(%reg) \
  ld %reg, $sp \
  incsp
.code

INIT:
  li $c, 1                      
  st $c, $sp                    @macro BALL_DY_PTR 255 
  PUSH_VALUE(-1)                @macro BALL_DX_PTR 254
  PUSH_VALUE(COLOR_WHITE)       # 253
  PUSH_VALUE(BALL_RADIUS)       # 252
  PUSH_VALUE(BALL_START_Y)      @macro BALL_Y_PTR 251
  PUSH_VALUE(BALL_START_X)      @macro BALL_X_PTR 250
  @macro BALL_PTR 250

  PUSH_VALUE(COLOR_GREEN)       # 249
  PUSH_VALUE(PADDLE_H)          # 248
  PUSH_VALUE(PADDLE_W)          # 247
  PUSH_VALUE(PADDLE_1_START_Y)  @macro PADDLE_1_Y_PTR 246
  PUSH_VALUE(PADDLE_1_START_X)  @macro PADDLE_1_X_PTR 245
  @macro PADDLE_1_PTR 245

  PUSH_VALUE(1)                 @macro PADDLE_2_DY_PTR 244
  PUSH_VALUE(COLOR_RED)       
  PUSH_VALUE(PADDLE_H)          
  PUSH_VALUE(PADDLE_W)          
  PUSH_VALUE(PADDLE_2_START_Y)  @macro PADDLE_2_Y_PTR 240
  PUSH_VALUE(PADDLE_2_START_X)  @macro PADDLE_2_X_PTR 239
  @macro PADDLE_2_PTR 239

  # ==== LINES ====
  PUSH_VALUE(COLOR_WHITE)       
  PUSH_VALUE(LINE_W)
  PUSH_VALUE(SCREEN_BOTTOM)
  PUSH_VALUE(128)
  PUSH_VALUE(SCREEN_TOP)
  PUSH_VALUE(128)
  @macro MIDDLE_LINE_PTR 233 

  PUSH_VALUE(COLOR_WHITE)       
  PUSH_VALUE(LINE_W) 
  PUSH_VALUE(SCREEN_TOP) # y1
  PUSH_VALUE(SCREEN_LEFT) # x1
  PUSH_VALUE(SCREEN_TOP)
  PUSH_VALUE(SCREEN_RIGHT)
  @macro TOP_LINE_PTR 227 

  PUSH_VALUE(COLOR_WHITE)       
  PUSH_VALUE(LINE_W)
  PUSH_VALUE(SCREEN_TOP)
  PUSH_VALUE(SCREEN_RIGHT)
  PUSH_VALUE(SCREEN_BOTTOM)
  PUSH_VALUE(SCREEN_RIGHT)
  @macro RIGHT_LINE_PTR 221

  PUSH_VALUE(COLOR_WHITE)       
  PUSH_VALUE(LINE_W)
  PUSH_VALUE(SCREEN_BOTTOM)
  PUSH_VALUE(SCREEN_RIGHT)
  PUSH_VALUE(SCREEN_BOTTOM)
  PUSH_VALUE(SCREEN_LEFT)
  @macro BOTTOM_LINE_PTR 215

  PUSH_VALUE(COLOR_WHITE)       
  PUSH_VALUE(LINE_W)
  PUSH_VALUE(SCREEN_BOTTOM)
  PUSH_VALUE(SCREEN_LEFT)
  PUSH_VALUE(SCREEN_TOP)
  PUSH_VALUE(SCREEN_LEFT)
  @macro LEFT_LINE_PTR 209

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
  swpx # (b:a) = ra
  PUSH_REG($a)
  PUSH_REG($b)

  # ==== BALL ====
    # Y
      li $c, BALL_DY_PTR
      ld $b, $c

      li $c, BALL_Y_PTR
      ld $a, $c

      add

      st $a, $c
    # X
      li $c, BALL_DX_PTR
      ld $b, $c

      li $c, BALL_X_PTR
      ld $a, $c

      add

      st $a, $c
    # collisions
        #   paddle 1
      # if ball.left > p1.right: not colliding
      la UPDATE_BALL_COLL_PAD1_NOT_COLLIDING
      swpx

      li $c, BALL_X_PTR
      ld $a, $c
      li $b, BALL_RADIUS
      sub
      PUSH_REG($a) # ball.left

      li $c, PADDLE_1_X_PTR
      ld $a, $c
      li $b, PADDLE_W
      add
      mv $b, $a   # p1.right

      POP_REG($a) # ball.left
      jgt

      # if p1.left > ball.right: not colliding
      la UPDATE_BALL_COLL_PAD1_NOT_COLLIDING
      swpx

      li $c, PADDLE_1_X_PTR
      ld $a, $c
      PUSH_REG($a) # p1.left

      li $c, BALL_X_PTR
      ld $a, $c
      li $b, BALL_RADIUS
      add
      mv $b, $a    # ball.right

      POP_REG($a)  # p1.left
      jgt

      # if ball.top > p1.bottom: not colliding
      la UPDATE_BALL_COLL_PAD1_NOT_COLLIDING
      swpx

      li $c, BALL_Y_PTR
      ld $a, $c
      li $b, BALL_RADIUS
      sub
      PUSH_REG($a) # ball.top

      li $c, PADDLE_1_Y_PTR
      ld $a, $c
      li $b, PADDLE_H
      add
      mv $b, $a    # p1.bottom

      POP_REG($a)  # ball.top
      jgt

      # if p1.top > ball.bottom: not colliding
      la UPDATE_BALL_COLL_PAD1_NOT_COLLIDING
      swpx

      li $c, PADDLE_1_Y_PTR
      ld $a, $c
      PUSH_REG($a) # p1.top

      li $c, BALL_Y_PTR
      ld $a, $c
      li $b, BALL_RADIUS
      add
      mv $b, $a    # ball.bottom

      POP_REG($a)  # p1.top
      jgt

      # colliding
        li $c, BALL_DX_PTR
        li $a, 1
        st $a, $c

        li $c, OKTO_RANDOM
        call

        li $b, 0b0000_0011
        and

        PUSH_REG($a)

        la UPDATE_BALL_PAD1_SET_DY_NEGATIVE
        swpx

        POP_REG($a)
        li $b, 0
        jeq

        PUSH_REG($a)

        la UPDATE_BALL_PAD1_SET_DY_ZERO
        swpx

        POP_REG($a)
        li $b, 1
        jeq

        PUSH_REG($a)

        la UPDATE_BALL_PAD1_SET_DY_POSITIVE
        swpx

        POP_REG($a)
        li $b, 2
        jeq

        la UPDATE_BALL_PAD1_SET_DY_ZERO
        swpx
        jmp

        UPDATE_BALL_PAD1_SET_DY_NEGATIVE:
        li $c, BALL_DY_PTR
        li $a, 255
        st $a, $c

        la UPDATE_BALL_PAD1_END_SET_DY
        swpx
        jmp

        UPDATE_BALL_PAD1_SET_DY_ZERO:
        li $c, BALL_DY_PTR
        li $a, 0
        st $a, $c

        la UPDATE_BALL_PAD1_END_SET_DY
        swpx
        jmp

        UPDATE_BALL_PAD1_SET_DY_POSITIVE:
        li $c, BALL_DY_PTR
        li $a, 1
        st $a, $c

        UPDATE_BALL_PAD1_END_SET_DY:

    UPDATE_BALL_COLL_PAD1_NOT_COLLIDING:
    #   paddle 2
      # if ball.left > p2.right: not colliding
      la UPDATE_BALL_COLL_PAD2_NOT_COLLIDING
      swpx

      li $c, BALL_X_PTR
      ld $a, $c
      li $b, BALL_RADIUS
      sub
      PUSH_REG($a) # ball.left

      li $c, PADDLE_2_X_PTR
      ld $a, $c
      li $b, PADDLE_W
      add
      mv $b, $a   # p2.right

      POP_REG($a) # ball.left
      jgt

      # if p2.left > ball.right: not colliding
      la UPDATE_BALL_COLL_PAD2_NOT_COLLIDING
      swpx

      li $c, PADDLE_2_X_PTR
      ld $a, $c
      PUSH_REG($a) # p2.left

      li $c, BALL_X_PTR
      ld $a, $c
      li $b, BALL_RADIUS
      add
      mv $b, $a    # ball.right

      POP_REG($a)  # p2.left
      jgt

      # if ball.top > p2.bottom: not colliding
      la UPDATE_BALL_COLL_PAD2_NOT_COLLIDING
      swpx

      li $c, BALL_Y_PTR
      ld $a, $c
      li $b, BALL_RADIUS
      sub
      PUSH_REG($a) # ball.top

      li $c, PADDLE_2_Y_PTR
      ld $a, $c
      li $b, PADDLE_H
      add
      mv $b, $a    # p2.bottom

      POP_REG($a)  # ball.top
      jgt

      # if p2.top > ball.bottom: not colliding
      la UPDATE_BALL_COLL_PAD2_NOT_COLLIDING
      swpx

      li $c, PADDLE_2_Y_PTR
      ld $a, $c
      PUSH_REG($a) # p2.top

      li $c, BALL_Y_PTR
      ld $a, $c
      li $b, BALL_RADIUS
      add
      mv $b, $a    # ball.bottom

      POP_REG($a)  # p2.top
      jgt

      # colliding
        li $c, BALL_DX_PTR
        li $a, -1
        st $a, $c

        li $c, OKTO_RANDOM
        call

        li $b, 0b0000_0011
        and

        PUSH_REG($a)

        la UPDATE_BALL_PAD2_SET_DY_NEGATIVE
        swpx

        POP_REG($a)
        li $b, 0
        jeq

        PUSH_REG($a)

        la UPDATE_BALL_PAD2_SET_DY_ZERO
        swpx

        POP_REG($a)
        li $b, 1
        jeq

        PUSH_REG($a)

        la UPDATE_BALL_PAD2_SET_DY_POSITIVE
        swpx

        POP_REG($a)
        li $b, 2
        jeq

        la UPDATE_BALL_PAD2_SET_DY_ZERO
        swpx
        jmp

        UPDATE_BALL_PAD2_SET_DY_NEGATIVE:
        li $c, BALL_DY_PTR
        li $a, 255
        st $a, $c

        la UPDATE_BALL_PAD2_END_SET_DY
        swpx
        jmp

        UPDATE_BALL_PAD2_SET_DY_ZERO:
        li $c, BALL_DY_PTR
        li $a, 0
        st $a, $c

        la UPDATE_BALL_PAD2_END_SET_DY
        swpx
        jmp

        UPDATE_BALL_PAD2_SET_DY_POSITIVE:
        li $c, BALL_DY_PTR
        li $a, 1
        st $a, $c

        UPDATE_BALL_PAD2_END_SET_DY:

    UPDATE_BALL_COLL_PAD2_NOT_COLLIDING:
    # =============================================
    # collision top/bottom world
      # if ball.bottom > SCREEN_BOTTOM: dy = 255
      la UPDATE_BALL_WORLD_BOTTOM
      swpx

      li $c, BALL_Y_PTR
      ld $a, $c
      li $b, BALL_RADIUS
      add                    # ball.y + radius

      PUSH_REG($a)

      li $b, SCREEN_BOTTOM

      POP_REG($a)
      jgt

      # if SCREEN_TOP > ball.top: dy = 1
      la UPDATE_BALL_WORLD_TOP
      swpx

      li $a, SCREEN_TOP

      li $c, BALL_Y_PTR
      ld $b, $c
      mv $a, $b

      li $b, BALL_RADIUS
      sub                    # ball.y - radius

      mv $b, $a
      li $a, SCREEN_TOP

      jgt

      la UPDATE_BALL_WORLD_END
      swpx
      jmp

    UPDATE_BALL_WORLD_BOTTOM:
      li $c, BALL_DY_PTR
      li $a, 255
      st $a, $c

      la UPDATE_BALL_WORLD_END
      swpx
      jmp

    UPDATE_BALL_WORLD_TOP:
      li $c, BALL_DY_PTR
      li $a, 1
      st $a, $c

    UPDATE_BALL_WORLD_END:
    # =============================================
    # ball out of screen -> reset center

      # if ball.x > SCREEN_RIGHT
      la UPDATE_BALL_RESET
      swpx

      li $c, BALL_X_PTR
      ld $a, $c
      li $b, 252
      jgt

      # if SCREEN_LEFT > ball.x
      la UPDATE_BALL_RESET
      swpx

      li $a, 4

      li $c, BALL_X_PTR
      ld $b, $c

      jgt

      la UPDATE_BALL_RESET_END
      swpx
      jmp

    UPDATE_BALL_RESET:
      li $c, BALL_X_PTR
      li $a, BALL_START_X
      st $a, $c

      li $c, BALL_Y_PTR
      li $a, BALL_START_Y
      st $a, $c

    UPDATE_BALL_RESET_END:
  # ==== PADDLE 1 ====
  # input
    # input up
  li $a, 1    # player 1
  li $c, OKTO_INPUT
  call

  li $b, INPUT_UP
  and

  PUSH_REG($a)

  la UPDATE_INPUT_UP
  swpx

  POP_REG($a)

  li $b, INPUT_UP
  jeq

    # input up
  li $a, 1      # player 1
  li $c, OKTO_INPUT
  call

  li $b, INPUT_DOWN
  and

  PUSH_REG($a)

  la UPDATE_INPUT_DOWN
  swpx

  POP_REG($a)

  li $b, INPUT_DOWN
  jeq

  la UPDATE_NO_INPUT
  swpx
  jmp

  UPDATE_INPUT_UP:
    li $c, PADDLE_1_Y_PTR
    ld $a, $c
    li $b, PADDLE_SPEED
    sub
    st $a, $c

    la UPDATE_NO_INPUT
    swpx
    jmp
  UPDATE_INPUT_DOWN:
    li $c, PADDLE_1_Y_PTR
    ld $a, $c
    li $b, PADDLE_SPEED
    add
    st $a, $c

  UPDATE_NO_INPUT:
  UPDATE_CHECK_BOUNDS:
    # - - - - - - - -
    # top
    la UPDATE_CHECK_BOUNDS_NOT_SET_TO_TOP
    swpx

    li $c, PADDLE_1_Y_PTR
    ld $a, $c
      
    li $b, PADDLE_MIN_Y

    jgt # if a > b: goto

    st $b, $c
   UPDATE_CHECK_BOUNDS_NOT_SET_TO_TOP:
    # - - - - - - - -
    # bottom
    la UPDATE_CHECK_BOUNDS_SET_TO_BOTTOM
    swpx

    li $c, PADDLE_1_Y_PTR
    ld $a, $c
      
    li $b, PADDLE_MAX_Y

    jgt # if a > b: goto

    la UPDATE_CHECK_BOUNDS_NOT_SET_TO_BOTTOM
    swpx
    jmp
   UPDATE_CHECK_BOUNDS_SET_TO_BOTTOM:
    li $c, PADDLE_1_Y_PTR
    li $a, PADDLE_MAX_Y
    st $a, $c
   UPDATE_CHECK_BOUNDS_NOT_SET_TO_BOTTOM:
  # ==== PADDLE 2 ====
   # input left/right
    # left -> up
    li $a, 1
    li $c, OKTO_INPUT
    call

    li $b, INPUT_LEFT
    and

    PUSH_REG($a)

    la UPDATE_PADDLE_2_INPUT_LEFT
    swpx

    POP_REG($a)

    li $b, INPUT_LEFT
    jeq

      # right -> down
    li $a, 1
    li $c, OKTO_INPUT
    call

    li $b, INPUT_RIGHT
    and

    PUSH_REG($a)

    la UPDATE_PADDLE_2_INPUT_RIGHT
    swpx

    POP_REG($a)

    li $b, INPUT_RIGHT
    jeq

    la UPDATE_PADDLE_2_NO_INPUT
    swpx
    jmp

    UPDATE_PADDLE_2_INPUT_LEFT:
      li $c, PADDLE_2_Y_PTR
      ld $a, $c
      li $b, PADDLE_SPEED
      sub
      st $a, $c

      la UPDATE_PADDLE_2_NO_INPUT
      swpx
      jmp

    UPDATE_PADDLE_2_INPUT_RIGHT:
      li $c, PADDLE_2_Y_PTR
      ld $a, $c
      li $b, PADDLE_SPEED
      add
      st $a, $c

    UPDATE_PADDLE_2_NO_INPUT:
    UPDATE_PADDLE_2_CHECK_BOUNDS:
      # top
      la UPDATE_PADDLE_2_CHECK_BOUNDS_NOT_SET_TO_TOP
      swpx

      li $c, PADDLE_2_Y_PTR
      ld $a, $c
      li $b, PADDLE_MIN_Y

      jgt

      st $b, $c

    UPDATE_PADDLE_2_CHECK_BOUNDS_NOT_SET_TO_TOP:
      # bottom
      la UPDATE_PADDLE_2_CHECK_BOUNDS_SET_TO_BOTTOM
      swpx

      li $c, PADDLE_2_Y_PTR
      ld $a, $c
      li $b, PADDLE_MAX_Y

      jgt

      la UPDATE_PADDLE_2_CHECK_BOUNDS_NOT_SET_TO_BOTTOM
      swpx
      jmp

    UPDATE_PADDLE_2_CHECK_BOUNDS_SET_TO_BOTTOM:
      li $c, PADDLE_2_Y_PTR
      li $a, PADDLE_MAX_Y
      st $a, $c

    UPDATE_PADDLE_2_CHECK_BOUNDS_NOT_SET_TO_BOTTOM:
  #
  POP_REG($b)
  POP_REG($a)
  swpx
  jmp

RENDER:
  swpx # (b:a) = ra
  PUSH_REG($a)
  PUSH_REG($b)

  li $c, OKTO_CLEAR
  call

  li $a, BALL_PTR
  li $c, OKTO_RENDER_CIRCLE
  call

  li $a, PADDLE_1_PTR
  li $c, OKTO_RENDER_RECT
  call

  li $a, PADDLE_2_PTR
  li $c, OKTO_RENDER_RECT
  call

  li $a, MIDDLE_LINE_PTR
  li $c, OKTO_RENDER_LINE
  call

  li $a, TOP_LINE_PTR
  li $c, OKTO_RENDER_LINE
  call

  li $a, RIGHT_LINE_PTR
  li $c, OKTO_RENDER_LINE
  call

  li $a, BOTTOM_LINE_PTR
  li $c, OKTO_RENDER_LINE
  call

  li $a, LEFT_LINE_PTR
  li $c, OKTO_RENDER_LINE
  call

  li $c, OKTO_PRESENT
  call

  POP_REG($b)
  POP_REG($a)
  swpx
  jmp

END:
  li $c, OKTO_EXIT
  call
