# Instruction Formats

The **OKTO** instructions are grouped into three formats, each defining how a
single byte is decoded by the virtual machine. Understanding these formats is
essential for reading OKTO machine code and for knowing how assembly operands
fit inside an 8-bit instruction.

| Format | Structure Example | Description                                                       |
|:------:|:------------------|:------------------------------------------------------------------|
| Alpha  | `lli $a, imm4`    | One register and a 4-bit immediate value. Used for load commands. |
| Beta   | `mv $a, $b`       | Two registers. Used for move, load, and store instructions.       |
| Gamma  | `add`             | No explicit operands. Uses fixed registers or VM state.           |

## Details

- **Alpha**:  
  Used for instructions that operate on one register and a 4-bit immediate
  value.  
  Example: `lli $a, 8` loads the lower nibble `0x8` into `$a`.

- **Beta**:  
  Used for instructions that operate on two registers.  
  Example: `mv $a, $b` copies the value of `$b` into `$a`.

- **Gamma**:  
  Used for instructions that have no explicit operands. These instructions use
  fixed registers, special registers, or the VM interface.  
  Example: `add` adds `$a` and `$b`, storing the result in `$a`.

## Register Codes

Registers are encoded with two bits.

| Register | Code   |
|:--------:|:------:|
| `$a`     | `00`   |
| `$b`     | `01`   |
| `$c`     | `10`   |
| `$sp`    | `11`   |

## Alpha Format

The Alpha format is used by `lli`, `lai`, and `lxi`.

```text
iiii rr oo
```

| Bits      | Field     | Description                  |
|:---------:|:----------|:-----------------------------|
| `7..4`    | `iiii`    | 4-bit immediate value        |
| `3..2`    | `rr`      | Destination register         |
| `1..0`    | `oo`      | Operation code               |

| Instruction | Opcode |
|:-----------:|:------:|
| `lli`       | `00`   |
| `lai`       | `01`   |
| `lxi`       | `10`   |

Example:

```python
lli $a, 8
```

| Field | Value  |
|:-----:|:------:|
| `iiii` | `1000` |
| `rr`   | `00`   |
| `oo`   | `00`   |

This produces `0b10000000`.

## Beta Format

The Beta format is used by `mv`, `ld`, and `st`.

```text
ss dd oo 11
```

| Bits      | Field     | Description                  |
|:---------:|:----------|:-----------------------------|
| `7..6`    | `ss`      | Source register              |
| `5..4`    | `dd`      | Destination register         |
| `3..2`    | `oo`      | Operation code               |
| `1..0`    | `11`      | Beta format marker           |

| Instruction | Opcode |
|:-----------:|:------:|
| `mv`        | `00`   |
| `ld`        | `01`   |
| `st`        | `10`   |

Example:

```python
st $a, $sp
```

| Field | Value  |
|:-----:|:------:|
| `ss`   | `11`   |
| `dd`   | `00`   |
| `oo`   | `10`   |
| marker | `11`   |

This produces `0b11001011`.

## Gamma Format

The Gamma format is used by all instructions without explicit operands.

```text
oooo 1111
```

| Bits      | Field     | Description                  |
|:---------:|:----------|:-----------------------------|
| `7..4`    | `oooo`    | Operation code               |
| `3..0`    | `1111`    | Gamma format marker          |

| Instruction | Byte         |
|:-----------:|:------------:|
| `add`       | `0b00001111` |
| `sub`       | `0b00011111` |
| `and`       | `0b00101111` |
| `or`        | `0b00111111` |
| `xor`       | `0b01001111` |
| `not`       | `0b01011111` |
| `shr`       | `0b01101111` |
| `shl`       | `0b01111111` |
| `jmp`       | `0b10001111` |
| `jeq`       | `0b10011111` |
| `jgt`       | `0b10101111` |
| `incsp`     | `0b10111111` |
| `decsp`     | `0b11001111` |
| `swpf`      | `0b11011111` |
| `swpx`      | `0b11101111` |
| `call`      | `0b11111111` |

## Codop Extension

OKTO uses the lowest bits of each instruction byte to identify the instruction
format. This works as a compact codop extension: some bit patterns are reserved
as markers, and the remaining bits are interpreted differently depending on the
marker.

If the instruction does not end with `11`, it is interpreted as Alpha:

```text
iiii rr oo
```

In this case, `oo` can be `00`, `01`, or `10`. The pattern `11` is reserved for
other formats.

If the instruction ends with `11`, the VM checks the next two bits. When those
bits are not `11`, the instruction is interpreted as Beta:

```text
ss dd oo 11
```

Here, `oo` can be `00`, `01`, or `10`. The pattern `11` is again reserved, this
time for Gamma instructions.

If the lower nibble is `1111`, the instruction is interpreted as Gamma:

```text
oooo 1111
```

This extension method allows OKTO to represent register-immediate,
register-register, and no-operand instructions inside a single byte.

> For a list of which instructions use each format, see the
> [instructions documentation](/docs/instructions.md).
>
> For pseudo-instruction expansion, see the
> [pseudo-instructions documentation](/docs/pseudo-instructions.md). For
> register codes and register behavior, see the
> [registers documentation](/docs/registers.md).
