# Memory

Memory is a core component of the **OKTO Virtual Machine**. It stores the
program code loaded from the `.code` section and the bytes used by the stack
during execution. Memory is organized as a fixed linear array of bytes with
65,536 addressable positions, from `0x0000` to `0xFFFF`.

Each instruction in OKTO is encoded as one byte. This allows the program counter
to point directly to the next instruction address without multiplying or
aligning the address.

You can read more about instruction encoding in the
[formats documentation](/docs/formats.md).

## Memory Structure

The virtual machine memory is divided into two main regions: instruction memory
and stack memory.

| Address Range       | Size       | Usage              |
|:-------------------:|:----------:|:-------------------|
| `0x0000` - `0xFEFF` | 65,280 B   | Instruction memory |
| `0xFF00` - `0xFFFF` | 256 B      | Stack memory       |

When a program starts, the bytes generated from the `.code` section are copied
to the beginning of memory. The remaining memory is left unspecified by the VM,
so programs should initialize every value they depend on.

## Instruction Memory

Instruction memory stores the executable bytes produced by the compiler. The
first byte of the `.code` section is loaded at address `0x0000`, the second byte
at address `0x0001`, and so on.

The VM only fetches instructions from addresses below `0xFF00`. If the program
counter reaches the stack region, execution fails with an instruction memory
out-of-bounds error.

You can read more about available instructions in the
[instructions documentation](/docs/instructions.md).

## Program Counter

The program counter (`pc`) is a 16-bit register that stores the address of the
current instruction. During execution, the VM fetches the byte at `pc`, decodes
it into an instruction, and then increments `pc` by one.

Control flow instructions can change the program counter. Instructions such as
`jmp`, `jeq`, and `jgt` load the next address from the `x` register. When a jump
happens, the old `pc` value is stored back into `x`, which allows routines to
return by jumping through `x` again.

## Stack Memory

Stack memory is the last 256 bytes of VM memory. It starts at `0xFF00` and ends
at `0xFFFF`. Stack addresses used by instructions are 8-bit offsets, so offset
`0x00` maps to real address `0xFF00`, and offset `0xFF` maps to real address
`0xFFFF`.

The `sp` register is an 8-bit general register used by convention as the stack
pointer. It starts at `0xFF`, which points to the top of the stack region.

The `ld` and `st` instructions access stack memory:

| Instruction | Meaning              |
|:-----------:|:---------------------|
| `ld $a, $b` | Load `*$b` into `$a` |
| `st $a, $b` | Store `$a` into `*$b` |

## Console Memory

OKTO also has console-specific memory regions. These are not part of the main VM
memory array. They are loaded from custom data sections and are used by the
fantasy console interface.

| Section    | Memory Size | Usage          |
|:----------:|:-----------:|:---------------|
| `.color`   | 256 B       | Color data     |
| `.palette` | 256 B       | Palette data   |
| `.sprite`  | 65,536 B    | Sprite data    |
| `.audio`   | 65,536 B    | Audio data     |

Data directives such as `.byte`, `.double`, `.char`, `.string`, `.stringz`, and
`.space` generate bytes for these sections. Numeric `.double` values are stored
in Big Endian order, with the most significant byte first.

You can read more about these sections and directives in the
[directives documentation](/docs/directives.md).

## Example of Memory Usage

```python
.code
    li $sp, 255
    li $a, 72
    st $a, $sp
```

The code above produces instruction bytes at the start of VM memory. The
expanded `li` pseudo-instructions are resolved into real one-byte instructions
before execution.

| Address  | Value        | Meaning             |
|:--------:|:------------:|:--------------------|
| `0x0000` | `0b11111100` | `lli $sp, 15`       |
| `0x0001` | `0b11111101` | `lai $sp, 15`       |
| `0x0002` | `0b10000000` | `lli $a, 8`         |
| `0x0003` | `0b01000001` | `lai $a, 4`         |
| `0x0004` | `0b11001011` | `st $a, $sp`        |

After the `st` instruction runs, the value `72` is stored at stack offset
`0xFF`, which maps to the real memory address `0xFFFF`.

> For more on pseudo-instruction expansion, see the
> [pseudo-instructions documentation](/docs/pseudo-instructions.md).
>
> For include files and macros used before parsing, see the
> [processors documentation](/docs/processors.md).
