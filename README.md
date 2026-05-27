<div align="center">
  <img src="docs/images/okto-mascot.png" width="300" />
</div>

<h1 align="center">OKTO</h1>

An 8-bit virtual machine, assembly programming language, and fantasy console for
small programs, experiments, and games.

---

## Idea

**OKTO** is built around a compact 8-bit instruction set where each native
instruction fits in a single byte. Programs are written in OKTO assembly,
compiled into a small binary format, and executed by the virtual machine.

The VM exposes a minimal register set, stack-based memory access, pseudo-
instructions for common assembly patterns, and a console interface for graphics,
input, audio, and terminal-style calls.

---

## Documentation

### Assembly

To learn about the OKTO assembly language, read:

- [Instructions](/docs/instructions.md): Learn about all native OKTO instructions.
- [Pseudo-Instructions](/docs/pseudo-instructions.md): Learn how higher-level assembly commands expand into native instructions.
- [Instruction Formats](/docs/formats.md): Understand the binary structure used to encode instructions.
- [Registers](/docs/registers.md): Understand the general and special registers used by the VM.

### Memory

To understand how OKTO stores and accesses data during execution, read:

- [Memory](/docs/memory.md): Learn about instruction memory, stack memory, and console memory regions.

### Project

OKTO currently includes:

- an assembler for OKTO assembly files
- a virtual machine for executing compiled programs
- a fantasy console interface
- example programs under [`examples`](/examples)
