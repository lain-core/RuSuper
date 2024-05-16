# RuSuper
gdb-style debugger of SNES roms. Upon starting the application, you will be prompted with a shell. You can use `help` to see the possible commands.

## Pre-requisites
    - Rustc 1.70.0+

## Usage
`cargo run filename`. You can apply the following command line arguments as well:

-  `--no-check`: Skip the checksum check for target rom and write to memory directly.
    - This is useful for test roms of very basic ASM.
    - This is NOT useful for retail ROMs, as the header and checksum are important to discern the mapping of the rom.

### Unimplemented/TBD
-   `--no-debug`: Run the program without executing the debugger. Currently hard coded to enable.

## Debugger Functionality
Either the prefix `$` or `0x` is allowed wherever an address literal is found to identify a hex value.

- `help` or `h`: Display the **h**elp screen.
- `p $XXXXXX`: **P**rint the byte and word values at address `$XXXXXX`
- `r` or `c`: **R**un (or **C**ontinue) the application until the next breakpoint is reached, or the program terminates.
- `q`, `exit`, `quit`: Terminate the application.

### Unimplemented/TBD
- Breakpoints
    - `b`: Create a **b**reakpoint at the current PC.
        - `b tag_name`:  Create a breakpoint at the current PC, with the tag name `tag_name`.
    - `b +X`: Set a breakpoint at the current PC, plus an offset value.
        - `b tag_name+X`: Set a breakpoint at the value represented by `tag_name` plus an offset value.
    - `b $XXXXXX`: Create a breakpoint at absolute address `$XXXXXX`.
        - `b $XXXXXX tag_name`: Create a breakpoint at absolute address `$XXXXXX` with the tag name `tag_name`
    - `b show`: Show all current breakpoints in a table, with tag names if applicable.
        - `b list`
    - `b del $XXXXXX`: Remove breakpoint at `$XXXXXX`
        - `b del tag_name`: Remove breakpoint associated with tag `tag_name`.

- Watches
    - `w $XXXXXX`: **W**atch for value changes at address X, and break if the value is modified.
        - `w $XXXXXX tag_name` Watch for value changes at absolute address `$XXXXXX` with the tag name `tag_name`

- Steps
    - `step`: Execute the next instruction.
        - `step +X` Step a number of times from the current instruction.

- Dump
    - TBD; Dump sections of memory to a .bin file for analysis.
    - TBD; Dump tags to an external file for re-use.
    - Areas to start with:
        - `loram`: 0x000000 - 0x3F1FFF+0x7E0000 - 0x7E1FFF (SNES LoRAM) to loram.bin in working dir.
        - `ppu`: 0x002000 - 0x3F3FFF (SNES PPU/APU) to apu.bin in working dir.
        - `controller`: 0x004000 - 0x3F41FF to controller.bin
        - `cpu`: 0x004200 - 0x3F5FFF to cpu.bin
        - `ram`: 0x7E0000 - 0x7FFFFF to ram.bin (includes slice of loram)
        - `tags`: Dump tags to a text file
        - `b`: Dump breakpoints to a text file

- Load
    - TBD: Load sections of memory from a .bin file for analysis.
    - TBD; Load tags from an external file for re-use.

## Project Status (5/16/24)
CPU Implementation: 2 out of 255 opcodes

Primary focus is: debugger implementation 