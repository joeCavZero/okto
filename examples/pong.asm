@include "okto.h"

# ==== CONSTS ====

@macro BALL_RADIUS 4
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

@macro P1_START_X 8
@macro P1_START_Y 112

@macro P2_START_X 240
@macro P2_START_Y 112

@macro INPUT_UP 0b0001_0000
@macro INPUT_DOWN 0b0010_0000

.color
    .byte 0, 0, 0, 0         @macro COLOR_TRANSPARENT 0
    .byte 0, 0, 0, 255       @macro COLOR_BLACK 1
    .byte 127, 127, 127, 255 @macro COLOR_GRAY 2
    .byte 255, 255, 255, 255 @macro COLOR_WHITE 3

.code

END:
    li $c, OKTO_EXIT
    call