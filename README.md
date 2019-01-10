# blink_bluepill_rust

Trying to blink an LED on a 1.35€ "blue pill" STM32F103C8 board.

# UNEXPECTED idcode: 0x2ba01477

After following chapter 1.3 and 1.3.3 of [The Embedded Rust Book](https://rust-embedded.github.io/book/intro/install.html), I tryed starting OpenOCD to check that if found my STLink-V2-1 programmer and my Blue Pill board. The Book says to type : `openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg`but since my board has an stm32f103, I used `openocd -f interface/stlink-v2-1.cfg -f target/stm32f1x.cfg`:

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
