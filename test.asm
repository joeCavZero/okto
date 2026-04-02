@macro a 10
start:
    pushc 'o'
    lli $x, 1
    add
loop:
    jeq
    xor
    load $x, $y
    jneq