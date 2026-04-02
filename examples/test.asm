@macro MACRO1 10

@macro MACRO2 \
90

@macro MACRO(%a, %b) lxi $a, %a \ 
    lxi $x, %b

start:
    lli $x, 1
    add
loop:
    jeq
    xor
    load $x, $y
    jneq