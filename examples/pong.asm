@include "okto.h"

# ==== CONSTS ====

@macro BALL_RADIUS 5
@macro BALL_START_X 127
@macro BALL_START_Y 127
@macro BALL_SPEED_POSITIVE 1
@macro BALL_SPEED_NEGATIVE 255

@macro SCREEN_TOP 8
@macro SCREEN_BOTTOM 248
@macro SCREEN_LEFT_LIMIT 4
@macro SCREEN_RIGHT_LIMIT 248

@macro PADDLE_W 8
@macro PADDLE_H 24
@macro PADDLE_HALF_H 12
@macro PADDLE_SPEED 3
@macro PADDLE_MIN_Y 8
@macro PADDLE_MAX_Y 224

@macro PADDLE_1_START_X 8
@macro PADDLE_1_START_Y 112

@macro PADDLE_2_START_X 240
@macro PADDLE_2_START_Y 112

@macro INPUT_UP 0b0001_0000
@macro INPUT_DOWN 0b0010_0000

.color
    .byte 0, 0, 0, 0         @macro COLOR_TRANSPARENT 0
    .byte 0, 0, 0, 255       @macro COLOR_BLACK 1
    .byte 127, 127, 127, 255 @macro COLOR_GRAY 2
    .byte 255, 255, 255, 255 @macro COLOR_WHITE 3

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
  li $c, COLOR_WHITE 
  st $c, $sp                    # 255
  PUSH_VALUE(BALL_RADIUS)       # 254
  PUSH_VALUE(BALL_START_Y)      @macro BALL_Y_PTR 253
  PUSH_VALUE(BALL_START_X)      @macro BALL_X_PTR 252
  @macro BALL_PTR 252

  PUSH_VALUE(COLOR_WHITE)       # 251
  PUSH_VALUE(PADDLE_H)          # 250
  PUSH_VALUE(PADDLE_W)          # 249
  PUSH_VALUE(PADDLE_1_START_Y)  @macro PADDLE_1_Y_PTR 248
  PUSH_VALUE(PADDLE_1_START_X)  @macro PADDLE_1_X_PTR 247
  @macro PADDLE_1_PTR 247

  PUSH_VALUE(COLOR_WHITE)       # 246
  PUSH_VALUE(PADDLE_H)          # 245
  PUSH_VALUE(PADDLE_W)          # 244
  PUSH_VALUE(PADDLE_2_START_Y)  @macro PADDLE_2_Y_PTR 243
  PUSH_VALUE(PADDLE_2_START_X)  @macro PADDLE_2_X_PTR 242
  @macro PADDLE_2_PTR 242

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

  # input
    # input up
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
    li $b, 1
    sub
    st $a, $c

    la UPDATE_NO_INPUT
    swpx
    jmp
  UPDATE_INPUT_DOWN:
    li $c, PADDLE_1_Y_PTR
    ld $a, $c
    li $b, 1
    add
    st $a, $c

  UPDATE_NO_INPUT:

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

  li $c, OKTO_PRESENT
  call

  POP_REG($b)
  POP_REG($a)
  swpx
  jmp

END:
  li $c, OKTO_EXIT
  call