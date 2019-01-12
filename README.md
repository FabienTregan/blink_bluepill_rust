# blink_bluepill_rust

Trying to blink an LED on a 1.35� "blue pill" STM32F103C8 board.
I guess things won't work on first try so I take notes in this file.

# Installing

I already hade Rustup installed. I removed the unsued toolchains and then followed instructions in chapter 1.3 and 1.3.3 (I used Windows, don't judge) of [The Embedded Rust Book](https://rust-embedded.github.io/book/intro/index.html).
I also wanted to add cargo-generate as told in chapter 1.2 (`cargo install cargo-generate`), but at some point I required to install msvc which is a really 1.1Gb download just a C compiler on windows. Since it is an optionnal step I skept it.

I upgraded y aliexpress clone ST-LINK V2 firmware to latest version using `ST-LinkUpgrade.exe` found in "ST Link utility" by ST.

# UNEXPECTED idcode: 0x2ba01477

After following chapters 1.3 and 1.3.3, I tryed starting OpenOCD to check that if found my STLink-V2-1 programmer and my Blue Pill board. The Book says to type : `openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg`but since my board has an stm32f103, I used `openocd -f interface/stlink-v2-1.cfg -f target/stm32f1x.cfg`:

```
D:\code\OpenOCD\bin>openocd -f interface/stlink-v2-1.cfg -f target/stm32f1x.cfg
GNU MCU Eclipse 64-bit Open On-Chip Debugger 0.10.0+dev-00352-gaa6c7e9b (2018-10-20-06:24)
Licensed under GNU GPL v2
For bug reports, read
        http://openocd.org/doc/doxygen/bugs.html
WARNING: interface/stlink-v2-1.cfg is deprecated, please switch to interface/stlink.cfg
Info : auto-selecting first available session transport "hla_swd". To override use 'transport select <transport>'.
Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
adapter speed: 1000 kHz
adapter_nsrst_delay: 100
none separate
Info : Listening on port 6666 for tcl connections
Info : Listening on port 4444 for telnet connections
Info : Unable to match requested speed 1000 kHz, using 950 kHz
Info : Unable to match requested speed 1000 kHz, using 950 kHz
Info : clock speed 950 kHz
Info : STLINK v2 JTAG v32 API v2 SWIM v7 VID 0x0483 PID 0x3748
Info : using stlink api v2
Info : Target voltage: 3.204230
Warn : UNEXPECTED idcode: 0x2ba01477
Error: expected 1 of 1: 0x1ba01477
in procedure 'init'
in procedure 'ocd_bouncer'
```

That does not work, the interesting lines are:
```
Warn : UNEXPECTED idcode: 0x2ba01477
Error: expected 1 of 1: 0x1ba01477
```

The idcode returned by the CPU does is not the expected one. That is not completely surprising: I bought the chipest board from aliexpress, and thought advertised havinf an stm32f103 chip from ST Micro, it comes with an advertised-as-perfect-replacement cs32f103c8t6 by CKS. It supposed to be a perfect clone (they do not even provide a datasheet for it), but this part returns a slightly different idcode.

The `idcode` is if cheap identifier. It is part of the [JTAG](https://en.wikipedia.org/wiki/JTAG) protocol. (We do not use JTAG here but the STLink protocol, which IIUC adds the possibility to use a simpler/cheaper connection between some ST Micro chips and the computer). At address (0x0) the protocol allow the chip to expose an identifier called `DPIDR` (for 'Debug Port Identification register', see chapter 2.2.5 of [ARM Debugger Interface Architecture Specification](https://static.docs.arm.com/ihi0031/d/debug_interface_v5_2_architecture_specification_IHI0031D.pdf). The documentation says that bits 28 to 31 contains `Revision code. The meaning of this field is IMPLEMENTATIONDEFINED.`.
Since only bits 28 and 29 are different, we can expect that the chip is still compatible, and create a new configuration file for OpenOCD tu just tell him to expect the actually received idcode.

I copied the `openocd\scripts\target\stm32f1x.cfg` file, naming the copy `cs32f1x.cfg` and changed:
 * the name of the chip:
```
if { [info exists CHIPNAME] } {
   set _CHIPNAME $CHIPNAME
} else {
   set _CHIPNAME cs32f1x
}

```
 * the idcode:
```
#jtag scan chain
if { [info exists CPUTAPID] } {
   set _CPUTAPID $CPUTAPID
} else {
   if { [using_jtag] } {
      # See STM Document RM0008 Section 26.6.3
      set _CPUTAPID 0x3ba00477
   } {
      # this is the SW-DP tap id not the jtag tap id
      set _CPUTAPID 0x2ba01477
   }
}
```

and could try running openocd again:

```
D:\code\OpenOCD\bin>openocd -f interface/stlink-v2-1.cfg -f target/cs32f1x.cfg
GNU MCU Eclipse 64-bit Open On-Chip Debugger 0.10.0+dev-00352-gaa6c7e9b (2018-10-20-06:24)
Licensed under GNU GPL v2
For bug reports, read
        http://openocd.org/doc/doxygen/bugs.html
WARNING: interface/stlink-v2-1.cfg is deprecated, please switch to interface/stlink.cfg
Info : auto-selecting first available session transport "hla_swd". To override use 'transport select <transport>'.
Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
adapter speed: 1000 kHz
adapter_nsrst_delay: 100
none separate
Info : Listening on port 6666 for tcl connections
Info : Listening on port 4444 for telnet connections
Info : Unable to match requested speed 1000 kHz, using 950 kHz
Info : Unable to match requested speed 1000 kHz, using 950 kHz
Info : clock speed 950 kHz
Info : STLINK v2 JTAG v32 API v2 SWIM v7 VID 0x0483 PID 0x3748
Info : using stlink api v2
Info : Target voltage: 3.205816
Info : cs32f1x.cpu: hardware has 6 breakpoints, 4 watchpoints
Info : Listening on port 3333 for gdb connections
_
```

It seems better.

(Thanks to tsman on eevblog forum)

# New project from template

I skept the installation of cargo-generate (because of msvc), so I could not use it to generate the Rust project from the template. I also did not want to create them by cloning the git repository (because I already hade an existing git repo with this readme.md file, so I just download [https://github.com/rust-embedded/cortex-m-quickstart/archive/master.zip] and unziped it in a blue_pill_blinky` subdirectory.and changed the project name to `blue_pill_blinky` in the blue_pill_blinky\Cargo.toml` file (twice)

# memory.x

Since the template is meant for stm32f4 with a differente quantity of flash than mine, I edited to `memory.x` file (which, I believe, is used to generate the linker scripts) with values I found in the stm32f103 datasheet. Hoping that the "C8" at the end of the marking of my CKS mcu means the same thing as the "C8" at the end of a genuin stm32f103, I guess this chip has 64kb ok flash (see chapter 7 of the datasheet) and it should have 20Kb of ram (first page of the datasheet). The memory map (chapter 4 of the same datasheet) tells me that flash memory starts at 0x0800.0000 and static ram starts at 0x2000-0000, giving the following content for the `memory.x` file:
```
/* Linker script for the STM32F103C8T6 */
MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 64K
  RAM : ORIGIN = 0x20000000, LENGTH = 20K
}
```
(I removed the comments from the template)

# Configuring the Core

We want to compile for our microcontoller. A microcontoler is an microprocessor packages with things like RAM, Flash memory, digital to analog converters, timers... Rust needs to know for which mcu we want to compile. The stm32f103 has a Cortex-M3 core (which is a proprietary but standard core found on many mcu form different manufacturers). Its architecture is called "ARMv7-M". Some ARM cores support an older instruction set, and some can be switched from the older ARM instruction set to the newer Thumb instruction set (smaller and faster). But the M3 only supports the newer Thumb instruction set. If you use a different Cortex-M mcu, have a look at [wikipedia](https://en.wikipedia.org/wiki/ARM_Cortex-M#Instruction_sets) to find which compilation target you want to configure in `.cargo/config`, for the Blue Pill it will be:
```
[build]
target = "thumbv7m-none-eabi"    # Cortex-M3

```

# Deleting the target directory

I renamed the project, hence the project's directory name. This caused Cargo to be unable to compile (not finding the linker file). The solution was to delete the target directory and build again.

# Changing the dependency and main.rs

The compilation never ended, so I replaced the content of my `main.rs` file with the content of the hello-world found in `template/` directory (it came with the template).
I'm not sur this step is needed bu I did it.

# Switching the linker

The compilation never ended, stucked at step 32/33. Fortunately some comment in the `.cargo/config` file cought my attention:
```
  # LLD (shipped with the Rust toolchain) is used as the default linker
   "-C", "link-arg=-Tlink.x",

  # if you run into problems with LLD switch to the GNU linker by commenting out this line
  # "-C", "linker=arm-none-eabi-ld",

```
I commented the line for the LLD linker and uncommented the one for the GNU linker and could complete the build.

# openocd.cfg

I edited the `openocd.cfg` file that came with the template to use the openocd configuration I made for my weird stm32 clone:
```
source [find target/cs32f1x.cfg]
```
Once this is done, I can run openocd from the same directory the `openocd.cfg` file is in and no longer need to pass the configuration for the CKS clone (nor for the ST-Link V2-1 which was the default configuration in the openocd.cfg file, I didn't need to change that but mays you do):
```
D:\code\rust\blink_bluepill_rust\blue_pill_blinky>d:\code\OpenOCD\bin\openocd.exe
GNU MCU Eclipse 64-bit Open On-Chip Debugger 0.10.0+dev-00352-gaa6c7e9b (2018-10-20-06:24)
Licensed under GNU GPL v2
For bug reports, read
        http://openocd.org/doc/doxygen/bugs.html
WARNING: interface/stlink-v2-1.cfg is deprecated, please switch to interface/stlink.cfg
Info : auto-selecting first available session transport "hla_swd". To override use 'transport select <transport>'.
Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
adapter speed: 1000 kHz
adapter_nsrst_delay: 100
none separate
Info : Listening on port 6666 for tcl connections
Info : Listening on port 4444 for telnet connections
Info : Unable to match requested speed 1000 kHz, using 950 kHz
Info : Unable to match requested speed 1000 kHz, using 950 kHz
Info : clock speed 950 kHz
Info : STLINK v2 JTAG v32 API v2 SWIM v7 VID 0x0483 PID 0x3748
Info : using stlink api v2
Info : Target voltage: 3.204230
Info : cs32f1x.cpu: hardware has 6 breakpoints, 4 watchpoints
Info : Listening on port 3333 for gdb connections
```

# Starting gdb

Start gdb by replace `<gbd>` in the command given in chapter 2.2. with the name of the executable of the gdb you downloaded from ST website. Also since I juste ran cargo build, cargo did not copy the source files in `examples/` then I used a different directory from the one given the Embedded Rust Book:
```
arm-none-eabi-gdb -d target\thumbv7m-none-eabi\debug\
```
but that did no seem to work. Anyway I could use the `file` command to tell where my firmware is, gdb to openocd running in another shell, and upload the firmware:

```
(gdb) file target/thumbv7m-none-eabi/debug/b
blue_pill_blinky    blue_pill_blinky.d  build/
(gdb) file target/thumbv7m-none-eabi/debug/blue_pill_blinky
A program is being debugged already.
Are you sure you want to change the file? (y or n) y
Reading symbols from target/thumbv7m-none-eabi/debug/blue_pill_blinky...
(No debugging symbols found in target/thumbv7m-none-eabi/debug/blue_pill_blinky)
(gdb) load
Start address 0x8000, load size 0
Transfer rate: 0 bits in <1 sec.
(gdb)
```
I guess I uploaded something because the Blue Pill stopped blinking the LED that was controller by the original firmware.

# Trying to execute 

I tryed following the chapter 2.2 form there, but "next" was not of much help when gdb could not find the debugging symbol in my binary. So I tryed running the code (`continue` send to openocd from gdb) but nothing appeared in the openocd console. I was expecting an "Hello, world!".

I quit openocd and gdb, and use `STM32 ST-LINK Utility.exe` from ST. I clicked "connect the target" and look if the flash of the  Blue Pill seemed to contain the "Hello, world!" string: it did not. I reset the Blue Pill and it start blinking. It seems I did not flash the firmware.
