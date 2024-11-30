//src/lang_item.rs
use core::panic::PanicInfo;
use crate::{println, sbi::shutdown};


#[panic_handler]
fn panic(info: &PanicInfo) ->! {
    if let Some(location) = info.location() {
        println!(
            "Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message()
        );
    } else {
        println!("Panicked: {}", info.message());
    }
    shutdown(true)
}