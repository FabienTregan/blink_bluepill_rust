23:32	nezza	Quick question about cortex-m... https://github.com/rust-embedded/cortex-m/blob/master/src/peripheral/nvic.rs#L87 how does this work
23:32	nezza	without using ptr::?
23:33	nezza	i would have assumed to write you still need to do something like this https://github.com/rust-embedded/cortex-m/blob/master/src/peripheral/nvic.rs#L135
23:35	nezza	ah, because you're always calling it on ptr()?
23:36	adamgreig	the icer member of NVIC has type volatile_register::RW 
23:36	adamgreig	which implements write()
23:36	adamgreig	which calls set() on a contained vcell::VolatileCell
23:36	adamgreig	https://docs.rs/volatile-register/0.2.0/src/volatile_register/lib.rs.html#82
23:36	adamgreig	which calls core:tr::write_volatile
23:36	adamgreig	https://docs.rs/vcell/0.1.0/src/vcell/lib.rs.html#51
23:37	nezza	and where does write_volatile get the address from? That's where I'm confused  
23:37	adamgreig	it's the address of the icer member in the NVIC instance
23:37	nezza	I see that the address is set here: https://github.com/rust-embedded/cortex-m/blob/master/src/peripheral/mod.rs#L232
23:38	adamgreig	that's for CBP
23:38	adamgreig	https://github.com/rust-embedded/cortex-m/blob/master/src/peripheral/mod.rs#L422 for nvic
23:38	nezza	it's just an example, i have a peripheral that's at an address
23:38	adamgreig	yes
23:38	treg	speaking about RW : yesterday I tryed to understand how it prevents "non volatil" access to be optimised, and since I could not fine link with llvm volatile thing, I guessed that it's because it uses llvm inlining mechanisme but was not sure about this.
23:38	therealprof	treg: stm32f1 is a PAC not a HAL impl.
23:39	adamgreig	all NVIC instances in cortex-m deref to a nvic::RegisterBlock struct at a specific address
23:39	nezza	adamgreig: yea, where does that deref take place?
23:39	adamgreig	so if you have an NVIC, you can call any methods of a RegisterBlock and rust will deref to that registerblock at the constant address
23:39	therealprof	treg: both stm32f1 and stm32f103xx provide abstractions over all registers
23:39	adamgreig	any time you call a method on a NVIC or access a member of a RegisterBlock
23:39	nezza	yea but where is the address of the NVIC set? 
23:39	nezza	it's on 0xE000_E100
23:40	therealprof	stm32f1 is part of stm32-rs which aims to provide a fixed and common abstraction over all vendor provided SVD files while stm32f103xx is a one-off half-fixed version for the STM32F103 only.
23:40	nezza	https://github.com/rust-embedded/cortex-m/blob/master/src/peripheral/mod.rs#L422 i see this, but how does
23:40	nezza	//!         let mut peripherals = Peripherals::take().unwrap();
23:40	nezza	 peripherals.DWT.enable_cycle_counter();
23:40	nezza	work then
23:41	adamgreig	nezza: there's a Peripherals struct which contains one of each of the peripheral structs and that can all live anywhere in memory
23:41	nezza	as in, how does peripharls.DWT know to write to 0xE000_1000+whatever
23:41	adamgreig	it contains an NVIC struct
23:41	adamgreig	the NVIC struct instance can also live anywhere in memory (or nowhere at all since it doesn't contain anything except phantomdata)
23:41	nezza	i get that it can live anywhere, but how does enable work when it doesn't write to the address where NVIC actually lives
23:41		*** denisvasilik quit (Ping timeout: 121 seconds)
23:41	adamgreig	when you access a field or method on an NVIC instance which is implemented on a nvic::RegisterBlock instead, rust derefs the nvic to a registerblock
23:42	adamgreig	and specifically it derefs it to a constant address registerblock which is always at the address in the ptr() method of NVIC
23:42	nezza	ok so registerblock calls ptr? 
23:42	nezza	(simplified)
23:42	adamgreig	no, deref() no NVIC calls ptr to get the address
23:42	adamgreig	then casts that to a pointer-to-registerblock and derefs it
23:42	adamgreig	https://github.com/rust-embedded/cortex-m/blob/master/src/peripheral/mod.rs#L430
23:42	nezza	AH 
23:42	nezza	yes
23:42	nezza	now
23:42	nezza	 thank you 
23:43	adamgreig	np
23:44		*** nagisa quit (Client exited)
23:45	treg	adamgreig: is all of this a zero cost abstraction ?
23:46	adamgreig	yea
23:46	adamgreig	in the end you just get a str instruction to the final memory address
23:46	adamgreig	well... so long as you compile in release mode, hah
23:46	treg	so pretty 
23:46	adamgreig	maybe
23:46	adamgreig	i am coming to the conclusion that it might be zero cost on hardware but has non-zero conceptual cost
23:47	adamgreig	oh well 
23:47	treg	I hope to understand all of this deeply enough to give a talk at local rust group 
23:47	nezza	adamgreig: haha  still, thanks a lot, that helped massively
23:47	treg	Is an abstraction with zro conceptual cost technically possible ? 
23:48	adamgreig	i'll leave that one for the philosophers
23:48	treg	they'll probably say that an abstraction is not a cost it's a revenue, but don't try to review their code.
23:48	nezza	maybe one more question, why are a lot of the peripherals using ptr() directly when it shouldn't be necessary?
23:49	adamgreig	a lot of the time you'll see that in methods that don't take self
23:49	adamgreig	so they don't actually have an NVIC instance available
23:49	nezza	true
23:49	adamgreig	deref() requires &self
23:49	nezza	any reason they don't take self? (I'm writing a periphal I'm planning to PR and want to do it 'right'  )
23:50	adamgreig	so you can't call registerblock methods or access its fields on a method without self
23:50	treg	(is their a documentation about "deref() no NVIC calls ptr to get the addresse" ? I'm afraid oreilly's book won't cover it  )
23:50	adamgreig	(to model not having exclusive access to the peripheral)
23:50	adamgreig	calling ptr() gets you the pointer but dereferencing it is unsafe
23:50	adamgreig	so those methosd without self can be used from any context without having an actual nvic instance around
23:50		*** mocatta quit (The TLS connection was non-properly terminated.)
23:51	adamgreig	ugh, sorry, i'm all over the place
23:51	adamgreig	the gist is the static methods (without self) don't need to have exclusive access to the peripheral
23:51	adamgreig	appropriate for read-only methods, or accessing write-only fields, but not appropriate for anything with read-modify-write semantics
23:51	nezza	ok roger
23:51	adamgreig	extremely useful to provide methods that user code wants to call all over the place without having to have passed around an NVIC
23:51	nezza	will try to keep that in mind
23:52	adamgreig	implementing those methods requires unsafe{} because they can't use the NVIC instance to just deref, they have to get the pointer and deref it themselves (which is unsafe)
23:52	treg	(I copy pasted this discussion, is it ok with both of you if it ends up in my notes on github ?)
23:52	nezza	what an odysee, to test my stuff I first had to patch qemu  
23:52	adamgreig	so they have to 'promise' they won't do anything bad
23:52	adamgreig	that's fine treg, this channel is also logged publically
23:52	nezza	treg: sounds good!
23:52	adamgreig	treg: so we could just only do methods with self, but then they're less convenient to use
23:52	adamgreig	so the push is to do static methods whenver it is possible to do so safely, but those must use ptr() because they don't have a self to deref
23:53	therealprof	Oh noes, we're gonna end up on TV (or something) 
23:53	adamgreig	treg: about deref - the book might cover the Deref trait and how that works
23:54	adamgreig	the rest is a specific cortex-m thing which you sort of just have to read the code, there's not much of it
23:54	adamgreig	it's these ~10 lines https://github.com/rust-embedded/cortex-m/blob/master/src/peripheral/mod.rs#L419-L432
23:55	therealprof	Hm, just noticed the abstraction someone did for the LEDs on the STM32F407DISCO actually consumes the whole register block…
23:55	therealprof	Was gonna copy that for the STM32F072DISCO but that's so not going to fly for me. 