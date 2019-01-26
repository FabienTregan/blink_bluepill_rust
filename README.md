# blink_bluepill_rust

Trying to blink an LED on a 1.35€ "blue pill" STM32F103C8 board.
I guess things won't work on first try so I take notes in this file.

# Chapter 1 and 2 of The Embedded Rust Book. (Install to "Hello, world!")

## Installing

I already hade Rustup installed. I removed the unsued toolchains and then followed instructions in chapter 1.3 and 1.3.3 (I used Windows, don't judge) of [The Embedded Rust Book](https://rust-embedded.github.io/book/intro/index.html).
I also wanted to add cargo-generate as told in chapter 1.2 (`cargo install cargo-generate`), but at some point I required to install msvc which is a really 1.1Gb download just a C compiler on windows. Since it is an optionnal step I skept it.

I upgraded y aliexpress clone ST-LINK V2 firmware to latest version using `ST-LinkUpgrade.exe` found in "ST Link utility" by ST.

## UNEXPECTED idcode: 0x2ba01477

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

## New project from template

I skept the installation of cargo-generate (because of msvc), so I could not use it to generate the Rust project from the template. I also did not want to create them by cloning the git repository (because I already hade an existing git repo with this readme.md file, so I just download [https://github.com/rust-embedded/cortex-m-quickstart/archive/master.zip] and unziped it in a blue_pill_blinky` subdirectory.and changed the project name to `blue_pill_blinky` in the blue_pill_blinky\Cargo.toml` file (twice)

## memory.x

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

## Compiling for the proper

We want to compile for our microcontoller. A microcontoler is an microprocessor packages with things like RAM, Flash memory, digital to analog converters, timers... Rust needs to know for which mcu we want to compile. The stm32f103 has a Cortex-M3 core (which is a proprietary but standard core found on many mcu form different manufacturers). Its architecture is called "ARMv7-M", this information is in the datasheet but I got it from [wikipedia](https://en.wikipedia.org/wiki/ARM_Cortex-M#Cortex-M3). So in `.cargo/config`, for the Blue Pill it will be:
```
[build]
target = "thumbv7m-none-eabi"    # Cortex-M3

```
`thumb` here relate to the instruction-set we want to use. Since Cortex-M only support the newer Thumb instructon (which is a 16 bits instructions set, as opposed to the older 32 bits ARM set,  it's faster and take less space, see [wikipedia](https://en.wikipedia.org/wiki/ARM_architecture#Thumb) again). 

## Deleting the target directory

I renamed the project, hence the project's directory name. This caused Cargo to be unable to compile (not finding the linker file). The solution was to delete the target directory and build again.

## Changing the dependency and main.rs

The compilation never ended, so I replaced the content of my `main.rs` file with the content of the hello-world found in `template/` directory (it came with the template).
I'm not sur this step is needed bu I did it.

## Switching the linker

The compilation never ended, stucked at step 32/33. Fortunately some comment in the `.cargo/config` file cought my attention:
```
  # LLD (shipped with the Rust toolchain) is used as the default linker
   "-C", "link-arg=-Tlink.x",

  # if you run into problems with LLD switch to the GNU linker by commenting out this line
  # "-C", "linker=arm-none-eabi-ld",

```
I commented the line for the LLD linker and uncommented the one for the GNU linker and could complete the build.

## openocd.cfg

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

## Starting gdb

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

## Trying to execute 

I tryed following the chapter 2.2 form there, but "next" was not of much help when gdb could not find the debugging symbol in my binary. So I tryed running the code (`continue` send to openocd from gdb) but nothing appeared in the openocd console. I was expecting an "Hello, world!".

I quit openocd and gdb, and use `STM32 ST-LINK Utility.exe` from ST. I clicked "connect the target" and look if the flash of the  Blue Pill seemed to contain the "Hello, world!" string: it did not. I reset the Blue Pill and it start blinking. It seems I did not flash the firmware.

## Semihosting works!

After chatting on IRC, I tryed to use the GCC toolchain instead of just the GNU linker (see comments in `.cargo/config`) and it compiled and I could upload and exectue the firmware.

This is the value that works for me for rustflags in `.cargo/config`file:
```
rustflags = [
  "-C", "linker=arm-none-eabi-gcc",
  "-C", "link-arg=-Wl,-Tlink.x",
  "-C", "link-arg=-nostartfiles",
]
```
I now can build:
```
D:\code\rust\blink_bluepill_rust\blue_pill_blinky>cargo build
[...]
   Compiling cortex-m-rt-macros v0.1.5
    Finished dev [unoptimized + debuginfo] target(s) in 40.82s
```
Now I can continue try flashing the firmware again and debugging it. I understood that I was not using the  `openocd.dbg` file provided by the template, so here is what I do now:

1. Start OpenOCD
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
Info : Target voltage: 3.203691
Info : cs32f1x.cpu: hardware has 6 breakpoints, 4 watchpoints
Info : Listening on port 3333 for gdb connections
```
(I start from the directory where `openocd.cfg` file is, so I don't need to provide the `-f interface/stlink-v2-1.cfg -f target/cs32f1x.cfg`. And remember you might or might not need to make and use the `cs32f1x.cfg` file instead of `target/stm32f1x.cfg`)

2. Start gdb
 ```
 D:\code\rust\blink_bluepill_rust\blue_pill_blinky>arm-none-eabi-gdb -x openocd.gdb target\thumbv7m-none-eabi\debug\blue_pill_blinky
d:\Program Files (x86)\GNU Tools ARM Embedded\8 2018-q4-major\bin\arm-none-eabi-gdb.exe: warning: Couldn't determine a path for the index cache directory.
GNU gdb (GNU Tools for Arm Embedded Processors 8-2018-q4-major) 8.2.50.20181213-git
Copyright (C) 2018 Free Software Foundation, Inc.
[...]
Type "apropos word" to search for commands related to "word"...
Reading symbols from target\thumbv7m-none-eabi\debug\blue_pill_blinky...
core::sync::atomic::compiler_fence (order=32) at libcore/sync/atomic.rs:2351
2351    libcore/sync/atomic.rs: No such file or directory.
Breakpoint 1 at 0x8000f68: file C:\Users\Fabien\.cargo\registry\src\github.com-1ecc6299db9ec823\cortex-m-rt-0.6.7\src\lib.rs, line 550.
Function "UserHardFault" not defined.
Make breakpoint pending on future shared library load? (y or [n]) [answered N; input not from terminal]
Breakpoint 2 at 0x80015aa: file C:\Users\Fabien\.cargo\registry\src\github.com-1ecc6299db9ec823\panic-halt-0.2.0\src\lib.rs, line 32.
Breakpoint 3 at 0x8000402: file src\main.rs, line 13.
semihosting is enabled
Loading section .vector_table, size 0x400 lma 0x8000000
Loading section .text, size 0x1220 lma 0x8000400
Loading section .rodata, size 0x2ac lma 0x8001620
Start address 0x8000f26, load size 6348
Transfer rate: 17 KB/sec, 2116 bytes/write.
Note: automatically using hardware breakpoints for read-only addresses.
halted: PC: 0x08000f7c
DefaultPreInit ()
    at C:\Users\Fabien\.cargo\registry\src\github.com-1ecc6299db9ec823\cortex-m-rt-0.6.7\src\lib.rs:559
559     pub unsafe extern "C" fn DefaultPreInit() {}
(gdb) _
```
I now add the `-x openocd.gdb` parameter which is a script that does some things for us (like connecting gdb to openocd). Since the script is ran before we can use the `file` command to tell gdb where the elf file for the firmware is, we add the path to this as the last argument to gdb.
When the script is ran, you will see some information displayed in the other shell (the one with openocd running). The `semihosting is enabled` tells you that semihosting is activated. As the Rust Embedded Book explains, this allows us to basically use the debugger as stdout, hence display messages in OpenOCD.

3. step through
after using the `next` command in dgb, I finally got the expected message in OpenOCD:
```
[...]
Info : halted: PC: 0x08000626
Hello, world!
Info : halted: PC: 0x08000412
[...]
```

# Chapter 3 of The Embedded Rust Book (First led blinking)

Up to now, I have not done much thing wich is specific to the stm32f103c8 (clone) I use:
 * I installed ARM toolchain for Cortex-M (and Cortex-R) but this covers all the mcus in Cortex-M family (ARM design the core, and license the design to different manufacturers who produce them with differents options and package them with different peripherals)
 * I configured the `thumbv7m` target in `.cargo/config` (which covers all the Cortex-M3)
 * I changed the `idcode` in OpenOCD so I can tell it which `idcode` to expect from my clone
 * I set the proper size and base address for the flash and sram in the `memory.x` file.

 Now let's try to follow the Chapter 3 of The Rust Embedded Book, adapting the peripheral to the one available on the stm32f103. At first I want to follow a rather close to the metal approach (writting to Special Fucntion Registers, which are registers each having a fixed address in the address space of the MCU which serve to control the peripherals on the MCU).

 ## Timer

 At first I wander why the datasheet of the stm32f103 didn't give information about the special function registers used to control the timers. The thing is that the timer are not designed by ST (manufacturer of the stm32's), but standard Cortex peripherals designed by ARM. The information are in [ARM's Cortex-M3 documentation](http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0552a/Babieigh.html) and the System Timer has the same SFR (Special Function Registers) at the same address as the System Timer on the Cortex-M4 used by the authors of the book (which is an STM32F3DISCOVERY with a Cortex-M4F STM32F303VCT6 microcontroller)

 I wanted to go step by step, and execute even the first steps of the [3.1](https://rust-embedded.github.io/book/peripherals/a-first-attempt.html)("A First Attempt") chapter. It could have been easy to rewrite (or even copy/paste) but I learn more by rewritting the code from The ERB ("Embedded Rust Book" is a nice name but it's annoying when you type it so often :) ), unfortunately this line did not compile:
 ```
 let time = unsafe { std::ptr::read_volatile(&mut systick.cvr) };
 ```
The reason is that we compile for a microcontroller, hence want to get ride of the many things that comes in Rust standard lib. I edited my frist code which started with the `#![no_std]` attribute which tells the Rust compiler not to use this library. Of course you can not use `std::ptr::read_volatile` then because it is in the standard library (that's what the `std` stands for : standard).

I went to the Rust Embedded IRC channel to discuss this issue, it appeared the standard library does not exist for Cortex-M. The standard library wraps and adds functionnalities on the Core library and these additions are not wanted (because of limited ressource) or even possible ("jamesmunns: The Standard Library has all sorts of dependencies on things like filesystems, networking concepts, heap allocations, etc."). Fortunately, `std::ptr::read_volatile` is just a proxy for `core::ptr::read_volatile`, so we can use the Core Library instead. (This hade already been reported to the ERB team, but was dormant. Someone on the IRC channel made a pull request five minutes after I told them about my problem so you may not see it when you read the ERB.)

So, now we have something that should work:
```rust
#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

#[repr(C)]
struct SysTick {
    pub csr: u32,
    pub rvr: u32,
    pub cvr: u32,
    pub calib: u32,
}

#[entry]
fn main() -> ! {

    let systick = unsafe { &mut *(0xE000_E010 as *mut SysTick) };

    loop {
        let current_value_register = unsafe { core::ptr::read_volatile(&mut systick.cvr) };
        hprintln!("System timer current value is now {}.", current_value_register).unwrap();
    }
}
```
and after starting gdb and running (you need to `continue` twice, `c` is a shortcut for `continue`command) you get this fantastic output:
```
System timer current value is now 0.
System timer current value is now 0.
System timer current value is now 0.
System timer current value is now 0.
System timer current value is now 0.
System timer current value is now 0.
```
Not realy what we expected...

The code in [Chapter 3.1 of the ERB](https://rust-embedded.github.io/book/peripherals/a-first-attempt.html) aims at showing you how to create code, not how to use the timer on an stm32f. They hide some important things that can be found in the [ARM's Cortex-M3 documentation](http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0552a/Babieigh.html):
 * You need to set the Reload Value Register, which contain the value at which the timer will be reset when it reaches 0
 * You need to enable the counter (and eventually set the source clock you want to use, I will use internal processor clock because... why not)

 Hence the following code:
 ```Rust
#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

#[repr(C)]
struct SysTick {
    pub csr: u32,
    pub rvr: u32,
    pub cvr: u32,
    pub calib: u32,
}

#[entry]
fn main() -> ! {

    let systick = unsafe { &mut *(0xE000_E010 as *mut SysTick) };

    // Reload  Value Register set to 0x00FFFFFF
    // when timer starts or reachs 0, set automatically set is back to this value
    unsafe { core::ptr::write_volatile(&mut systick.rvr, 0x00FFFFFF) };
    
    // Timer Control and Status Register set so:
    // -Timer uses processor clock
    // -No exception is raised when value reaches zero
    // -Counter is enabled
    unsafe { core::ptr::write_volatile(&mut systick.csr, 0b000000000000000_0_0000000000000_101) };

    loop {
        let current_value_register = unsafe { core::ptr::read_volatile(&mut systick.cvr) };
        hprintln!("System timer current value is now {}.", current_value_register).unwrap();
    }
}
 ```
 Tadaaaa:
 ```
System timer current value is now 16777190.
System timer current value is now 16774224.
System timer current value is now 16771610.
System timer current value is now 16768996.
System timer current value is now 16766382.
System timer current value is now 16763768.
System timer current value is now 16761154.
System timer current value is now 16758540.
System timer current value is now 16755926.
System timer current value is now 16753312.
System timer current value is now 16750698.
System timer current value is now 16748084.
```

## Blinking the LED

Using the same way to access the proper SFR, it should be easy to blink the led that is on PC13 (PC13 is "Port C, pin 13". There is a pin of the stm32f which can supply current to an LED on the Blue Pill, and the voltage of this pin can be controller by the Port C, which can be controlled using the proper SFR)

It has not been as straight forward as I thought, mainly because I never used very few Cortex MCUs before. But one you understand how it works, that's super easy:
 * You need to activate the clock for the Port C (else Port C is sleeping, this is a power saving feature)
 * You need to configure the Port C bit 13 as an output
 * In order to find the address of a SFR, you need to look at the memory map diagram in the [Datasheet of the stm32f103](file:///C:/Users/Fabien/AppData/Local/Temp/cd00161566-1.pdf) or the [Reference Manual for STM32F101xx, STM32F102xx, STM32F103xx, STM32F105xx andSTM32F107xx advanced Arm®-based 32-bit MCUs](https://www.st.com/content/ccc/resource/technical/document/reference_manual/59/b9/ba/7f/11/af/43/d5/CD00171190.pdf/files/CD00171190.pdf/jcr:content/translations/en.CD00171190.pdf) to find the base address for the peripheral (you should find that the address space for Port C is 0x4001_1000 - 0x4001_13FF, hence base address is 0x4001_1000), and add the offset address for the SFR you want to access for this peripheral (or add the same offset to base address of another port if you want to control e.g. Port A or Port B).

 I will let you look in the reference manual about the SFR to control the Ports, but they lead to the following code:
 ```rust
 //! Prints "Hello, world!" on the host console using semihosting

#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

#[repr(C)]
struct SysTick {
    pub csr: u32,
    pub rvr: u32,
    pub cvr: u32,
    pub calib: u32,
}

#[repr(C)]
struct PortConfiguration {
    pub GPIOx_CRL: u32,
    pub GPIOx_CRH: u32,
    pub GPIOx_IDR: u32,
    pub GPIOx_ODR: u32,
    pub GPIOx_BSRR: u32,
    pub GPIOx_BRR: u32,
    pub GPIOx_LCKR: u32,
}

const PORT_C_BASE_ADDRESS: u32 = 0x4001_1000;
const RCC_APB2ENR_ADDRESS: u32 = 0x4002_1000 + 0x18;

#[entry]
fn main() -> ! {

    let systick = unsafe { &mut *(0xE000_E010 as *mut SysTick) };
    let port_c_sfr = unsafe { &mut *(PORT_C_BASE_ADDRESS as *mut PortConfiguration) };

    // Enables IO port C clock, disable many other that are probably already disabled.
    unsafe { core::ptr::write_volatile(RCC_APB2ENR_ADDRESS as *mut u32, 1 << 4) };

    // Reload  Value Register set to 0x00FFFFFF
    // when timer starts or reachs 0, set automatically set is back to this value
    unsafe { core::ptr::write_volatile(&mut systick.rvr, 0x00FFFFFF) };
    
    // Timer Control and Status Register set so:
    // -Timer uses processor clock
    // -No exception is raised when value reaches zero
    // -Counter is enabled
    unsafe { core::ptr::write_volatile(&mut systick.csr, 0b000000000000000_0_0000000000000_101) };

    // Port Configuration Register High for Port E:
    // -everything is floating input, exceptpin PC13 which is open drain output.
    unsafe { core::ptr::write_volatile(&mut port_c_sfr.GPIOx_CRH, 0b0100_0100_0110_0100_0100_0100_0100_0100 ) };

    loop {

        unsafe { core::ptr::write_volatile(&mut port_c_sfr.GPIOx_ODR, 0b0000000000000000_0010000000000000 ) };

        let current_value_register = unsafe { core::ptr::read_volatile(&mut systick.cvr) };
        hprintln!("System timer current value is now {}.", current_value_register).unwrap();

        unsafe { core::ptr::write_volatile(&mut port_c_sfr.GPIOx_ODR, 0b0000000000000000_0000000000000000 ) };

        let current_value_register = unsafe { core::ptr::read_volatile(&mut systick.cvr) };
        hprintln!("System timer current value is now {}.", current_value_register).unwrap();
    }
}
```
And it blinks !

Note that this code is completely hugly. My intent there was just to make sure I understood the 3.1 Chapter of The ERB and refactor making sure I understand every character I typed.

Also that this code has no code dedicated to spending some time beetwin turning the LED on and off. But since the semihosting is so slow, enough time is spent there (at least with default clock configuration).

If you let this code, you can not execute the firmware without the ST-Link connected and GDB started (the code would panic). If you remove the semihosting from the code, the led would blink so fast you won't see it blinking.

I made this quick modification which:
 * removes the message sending via semihosting
 * adds a `wait()` function which wait for the System Timer Current Statur Register bit 16 (COUNTFLAG) to reach 1. (this bit is automatically set to 1 when the counter reaches 0, and is automatically reset to 0 after it's read)

 So now I can plus the Blue Pill on an USB Charger and look at the LED blinking when I get asleep late at night:
 ```rust
 #![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m_rt::entry;

#[repr(C)]
struct SysTick {
    pub csr: u32,
    pub rvr: u32,
    pub cvr: u32,
    pub calib: u32,
}

#[repr(C)]
struct PortConfiguration {
    pub GPIOx_CRL: u32,
    pub GPIOx_CRH: u32,
    pub GPIOx_IDR: u32,
    pub GPIOx_ODR: u32,
    pub GPIOx_BSRR: u32,
    pub GPIOx_BRR: u32,
    pub GPIOx_LCKR: u32,
}

const PORT_C_BASE_ADDRESS: u32 = 0x4001_1000;
const RCC_APB2ENR_ADDRESS: u32 = 0x4002_1000 + 0x18;
const SYSTEM_TIMER_BASE_ADDRESS: u32 = 0xE000_E010;
const SYSTICK_COUNT_FLAG: u32 = 1 << 16;

#[entry]
fn main() -> ! {

    let systick = unsafe { &mut *(SYSTEM_TIMER_BASE_ADDRESS as *mut SysTick) };
    let port_c_sfr = unsafe { &mut *(PORT_C_BASE_ADDRESS as *mut PortConfiguration) };

    // Enables IO port C clock, disable many other that are probably already disabled.
    unsafe { core::ptr::write_volatile(RCC_APB2ENR_ADDRESS as *mut u32, 1 << 4) };

    // Reload  Value Register set to 0x000F0000
    // when timer starts or reachs 0, set automatically set is back to this value
    unsafe { core::ptr::write_volatile(&mut systick.rvr, 0x000FFFFF) };
    
    // Timer Control and Status Register set so:
    // -Timer uses processor clock
    // -No exception is raised when value reaches zero
    // -Counter is enabled
    unsafe { core::ptr::write_volatile(&mut systick.csr, 0b000000000000000_0_0000000000000_101) };

    // Port Configuration Register High for Port E:
    // -everything is floating input, exceptpin PC13 which is open drain output.
    unsafe { core::ptr::write_volatile(&mut port_c_sfr.GPIOx_CRH, 0b0100_0100_0110_0100_0100_0100_0100_0100 ) };

    loop {
        unsafe { core::ptr::write_volatile(&mut port_c_sfr.GPIOx_ODR, 0b0000000000000000_0010000000000000 ) };
        wait();
        unsafe { core::ptr::write_volatile(&mut port_c_sfr.GPIOx_ODR, 0b0000000000000000_0000000000000000 ) };
        wait();
    }
}

fn wait() -> () {
    let systick = unsafe { &mut *(SYSTEM_TIMER_BASE_ADDRESS as *mut SysTick) };
    while (unsafe { (core::ptr::read_volatile(&mut systick.csr) & SYSTICK_COUNT_FLAG ) == 0}) {
    }
}
```

# Switchingto HAL

Now that I've understood many things trying to do in rust exactly what I would have done in assembly, it is time to try using the Hardware Abstraction Layer and get rid of the unsafe code in my files. First I will import the crate and add two attributes to the `main.rs` which now starts with:
``` Rust
#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

extern crate panic_halt;
extern crate stm32f103xx_hal;
```
and add the crate to `cargo.toml`. The stm32f103xx_hal crate is not available from crates.io, so we need to fetch it from github:
```rust
stm32f103xx_hal = { git = "https://github.com/japaric/stm32f103xx_hal" }
```

Now `cargo build` will download the needed crates, and complain about all that unsafe code.

I will first try to deal with acessing the Port C.

The RCC register (which allow for activating the clock for Port C) will be dealt with by the code of the HAL, so I can remove this line:
```rust
    // Enables IO port C clock, disable many other that are probably already disabled.
    unsafe { core::ptr::write_volatile(RCC_APB2ENR_ADDRESS as *mut u32, 1 << 4) };
```
together with all the definition of RCC_APB2ENR_ADDRESS.

But for the HAL to be able to modify the RCC, I first must request the ownership on it, to I can pass it to the crate (this is Rust way of preventing conflicting modifications on the RCC):
```rust
use crate::stm32f103xx_hal::{
    prelude::*,
    device,
};

[...]

    let device_peripherals = device::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.constrain();
```
Now that I have a mutable reference on the RCC, I can pass it to the crate to get a mutable reference on Port C, and then on the pin to which the LED is connected:
```rust
    let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
```
To get the mutable reference to the pin, I need to tell the HAL that I want to use the pin in push_pull_output mode, so I no longer need these lines:
```rust
    // Port Configuration Register High for Port E:
    // -everything is floating input, exceptpin PC13 which is open drain output.
    unsafe { core::ptr::write_volatile(&mut port_c_sfr.GPIOx_CRH, 0b0100_0100_0110_0100_0100_0100_0100_0100 ) };
```
And now I can modify the state of the pin using the HAL, so I can replace:
```rust
unsafe { core::ptr::write_volatile(&mut port_c_sfr.GPIOx_ODR, 0b0000000000000000_0010000000000000 ) };
```
with:
```rust
led.set_high();
```
At this point, if I can try to comment the `#![deny(unsafe_code)]` attribute and `cargo build` this version that has HAL and safe access to the LED but still handles the timer in an unsafe and hugly way.

It seems it partially works: the LED blinks but at a very high frequency. I guess my `wait()` is not working because the HAL changed some settings on the system time or changed the frequency of the system clock.

I made a few tests to confirm that the problem was with `wait()` and not with the access to `PC13` (Port C, pin 13), and changing the `systick.rvr` (reset value of the timer, that means duration of the wait) did not change anything. So the System Counter must have been deactivated when accessing RCC to activate Port C clock.

So I try to use the HAL to access the timer too:
```rust
use nb::block;

use crate::stm32f103xx_hal::{
    prelude::*,
    device,
    timer::Timer,
};

[...]

    let cortex_peripherals = cortex_m::Peripherals::take().unwrap();
    let mut flash = device_peripherals.FLASH.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut timer = Timer::syst(cortex_peripherals.SYST, 5.hz(), clocks);

    loop {
        block!(timer.wait()).unwrap();
        block!(timer.wait()).unwrap();
        block!(timer.wait()).unwrap();
        block!(timer.wait()).unwrap();
        block!(timer.wait()).unwrap();
        led.set_high();
        block!(timer.wait()).unwrap();
        led.set_low();
        block!(timer.wait()).unwrap();
        led.set_high();
        block!(timer.wait()).unwrap();
        led.set_low();
    }
```
The `block!` macro comes from the `nb` (non blockng io layer) crate, so we add it to the `cargo.toml` file:
```
nb = "0.1.1"
```
And it works.

But I'm not really happy with this: I copy-pasted some of the code and don't understand why we need access to something called `FLASH` to use the clock.

The `freeze` trait signature is `pub fn freeze(self, acr: &mut ACR) -> Clocks`. After some research and guessing, I believe that because bits 0 to 2 of the Flash Access Control Register (ACR) sets the latency for writting to Flash memory. Since with mutable access to Clocks, you can change the system clock frequency, you probably need to adjust the latency, hence the need for it.

## Cleaning the code

Cleaning this code, I wanted to make a function to blink the led, and pass it what it requires. The `Timer` type is parameterized, and you can't use a generic `Time<A>` type because if does not provide `wait()`. I looked at the HAL code and the system timer implements a `CountDown` trait that defines `wait()`, unfortunately this trait is private so we can't use it in the signature. So for now I used Timer<SYST>, but the code will only work with the system timer.

I was ok with this, but passing the pin (PC13) lead to more problem: the type of the pin is PC13, making it impossible to pass another pin. The pin implements the OutputPin trait, but I could not understand in which crate this one ws defined so I can import it and use it as a signature. So I thought that the HAL I was using was not completely mature (it is on github but not on crates.io), so I tryed to switch to `stm32f1xx-hal = "0.1.1"`, but this one did not compile. The `stm32f1xx` crate covers the whole 1xx family, and you need to tell which one you want to use the the family, using Cargo's feature:
```
[dependencies.stm32f1xx-hal]
features = ["stm32f103", "rt"]
version = "0.1.1"
```
I update the code (mainly copy-pasting from the `examples/blinky.rs` file in the stm32f1xx-hal [source code](https://github.com/japaric/stm32f103xx-hal/blob/master/examples/blinky.rs)), but still didn't manage to fix the issue about `led` being typed as PC13 and not as an abstract `OutputPin`. After reading the code of the macro that generates this `PC13` and still not being able to understand how it was possible that the trait provides implementation for `set_low()` ad `set_high()` but I was still not able to cast the `PC13` to an `OutputPin`, I finally got it, chatting alone on IRC:

> 16:02	treg    Grrr, I really don't understand this:  
 16:04	treg	The macro for the gpios defines function `into_push_pull_output` which returns a `$PXi<Output<PushPull>>`  
 16:06	treg	later it provides implementation for the `OutputPin` trait (that is imported from `hal-embedded`) : `impl<MODE> OutputPin for $PXi<Output<MODE>> {`  
 16:06	treg	That makes sense, and I then can do: `let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh); led.set_low();`  
 16:07	treg	But then when I want to pass led as a parameter, using `OutputPin` as a trait (also used from `embedded_hal`), I get :  
 16:08	treg	`expected trait hal:relude:utputPin, found struct hal::gpio::gpioc:C13`  
 16:11	treg	noooooooooooo  
 16:12	treg	That was because I wasn't passing a mutable reference... The compilator's error message has not been very helpfull on this one 

This code finally compiles:
```rust
#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_halt;
extern crate stm32f1xx_hal as hal;
#[macro_use(block)]
extern crate nb;
extern crate embedded_hal;

use hal::prelude::*;
use hal::stm32;
use hal::timer::Timer;
use rt::{entry};
use cortex_m::peripheral::SYST;
use embedded_hal::digital::OutputPin;

#[entry]
fn main() -> ! {

    // Get control of the PC13 pin
    let device_peripherals = stm32::Peripherals::take().unwrap();
    let mut rcc = device_peripherals.RCC.constrain();
    let mut gpioc = device_peripherals.GPIOC.split(&mut rcc.apb2);
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    let cortex_peripherals = cortex_m::Peripherals::take().unwrap();
    let mut flash = device_peripherals.FLASH.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut timer = Timer::syst(cortex_peripherals.SYST, 5.hz(), clocks);

    led.set_high();
    loop {
        blink(&mut timer, &mut led, 2);
        wait(&mut timer, 10);
    }
}

fn blink(timer: &mut Timer<SYST>, led: &mut OutputPin, times: usize) -> () {
    for _n in 0..times {
        led.set_low();
        block!(timer.wait()).unwrap();
        led.set_high();
        block!(timer.wait()).unwrap();
    }
}

fn wait(timer: &mut Timer<SYST>, times: usize) -> () {
    for _n in 0..(times*2) {
        block!(timer.wait()).unwrap();
    }
}
```
Not only does it compile: it works :)