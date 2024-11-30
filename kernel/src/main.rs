// src/main.rs
#![no_std]
#![no_main]

mod mm;
mod syscall;
mod trap;
mod task;

mod lang_items;
mod sbi;

#[macro_use]
mod console;

use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));

#[unsafe(no_mangle)]
pub fn rust_main() -> !{
    clear_bss();
    print!("Hello World!\n");
    panic!("Shutdown machine!");
}

fn clear_bss(){
    unsafe extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });

}