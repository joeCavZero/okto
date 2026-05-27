# Registers

The **OKTO Virtual Machine** uses a small register set designed for an 8-bit
architecture. Most instructions operate on 8-bit values, while addresses are
handled with a 16-bit jump register and a 16-bit program counter.

OKTO has four general-purpose registers that can be used directly in assembly
code, plus special registers managed by the virtual machine during execution.

## General-Purpose Registers

The general-purpose registers store 8-bit values and are directly accessible by
native instructions.

| Register | Size | Description                                                   |
|:--------:|:----:|:--------------------------------------------------------------|
| `$a`     | 8 b  | Main accumulator used by arithmetic and logic instructions.   |
| `$b`     | 8 b  | Secondary operand used by arithmetic and logic instructions.  |
| `$c`     | 8 b  | General register, commonly used to select interface calls.    |
| `$sp`    | 8 b  | Stack pointer, used as an offset into stack memory.           |

These registers are encoded with two bits in Alpha and Beta instructions. You
can read more about this encoding in the
[formats documentation](/docs/formats.md).

## Register Numbering

| Register | Code |
|:--------:|:----:|
| `$a`     | `00` |
| `$b`     | `01` |
| `$c`     | `10` |
| `$sp`    | `11` |

## The A Register

The `$a` register is the main accumulator. Arithmetic, bitwise, and shift
instructions use `$a` as both an input and the destination.

Example:

```python
li $a, 10
li $b, 2
add
```

After execution, `$a` contains `12`.

## The B Register

The `$b` register is the secondary operand for operations that use fixed
registers. Instructions such as `add`, `sub`, `and`, `or`, `xor`, `shl`, and
`shr` combine `$a` with `$b`.

It is also used with `$a` to form 16-bit addresses. The `swpx` instruction treats
`$b:$a` as a 16-bit Big Endian value and swaps it with `$x`.

## The C Register

The `$c` register is a general-purpose register. By convention, it is commonly
used to select VM interface calls before executing `call`.

Example:

```python
li $c, OKTO_PRINT_CHAR
lchr $a, 'A'
call
```

In this example, `$c` selects the print-character call and `$a` contains the
character argument.

## The Stack Pointer

The `$sp` register is an 8-bit register used by convention as the stack pointer.
It starts at `0xFF`, which points to the top of the stack memory region.

Stack memory occupies the address range `0xFF00` to `0xFFFF`. When `ld` or `st`
uses a register as an address, that register is treated as an 8-bit offset into
this stack region.

Example:

```python
li $sp, 255
li $a, 72
st $a, $sp
```

After execution, the value `72` is stored at stack offset `0xFF`, which maps to
the real memory address `0xFFFF`.

You can read more about stack memory in the
[memory documentation](/docs/memory.md).

## Special Registers

Special registers are part of the VM state, but they are not valid explicit
operands in assembly code.

| Register | Size | Description                                              |
|:--------:|:----:|:---------------------------------------------------------|
| `$f`     | 8 b  | Flag register used for carry, borrow, and shifted bits.  |
| `$x`     | 16 b | Jump register used by `jmp`, `jeq`, and `jgt`.           |
| `$pc`    | 16 b | Program counter, storing the next instruction address.   |
| `$ir`    | 8 b  | Instruction register, storing the current instruction.   |

## The Flag Register

The `$f` register stores status information produced by some operations.

The `add` instruction stores `1` in `$f` when addition overflows, and `0`
otherwise. The `sub` instruction stores `1` in `$f` when subtraction borrows,
and `0` otherwise.

Shift instructions store the shifted-out bits in `$f`.

Example:

```python
li $a, 255
li $b, 1
add
```

After execution:

| Register | Value |
|:--------:|:-----:|
| `$a`     | `0`   |
| `$f`     | `1`   |

## The X Register

The `$x` register stores a 16-bit jump target. The jump instructions `jmp`,
`jeq`, and `jgt` load the next program counter value from `$x`.

When a jump is taken, the current `$pc` value is stored back into `$x`. This
makes `$x` useful as a simple return address register.

Example:

```python
la ROUTINE
swpx
jmp
```

The `la` pseudo-instruction loads the address of `ROUTINE` into `$b:$a`. The
`swpx` instruction moves `$b:$a` into `$x`, and `jmp` jumps to the address stored
in `$x`.

## The Program Counter

The `$pc` register stores the address of the next instruction. Since each OKTO
instruction is one byte, `$pc` points directly to a byte address in instruction
memory.

During execution, the VM fetches the instruction at `$pc`, decodes it, and then
increments `$pc` by one. Jump instructions can replace `$pc` with the value in
`$x`.

## The Instruction Register

The `$ir` register stores the raw 8-bit instruction byte currently being
executed. It is updated automatically during the fetch stage and is mainly
useful for debugging or VM internals.

## Initial Values

When the VM starts, `$pc` is initialized to `0`, `$ir` is initialized to `0`, and
`$sp` is initialized to `0xFF`. The other registers are not guaranteed to start
with any specific value, so programs should initialize every register they
depend on.

You can read more about the instruction behavior in the
[instructions documentation](/docs/instructions.md).

For the binary encoding of register operands, see the
[formats documentation](/docs/formats.md). For stack memory details, see the
[memory documentation](/docs/memory.md).
