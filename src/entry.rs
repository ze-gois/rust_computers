#![no_std]
#![no_main]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use userspace::info;

#[derive(Debug)]
pub struct Origin;

ample::trait_implement_primitives!();

#[unsafe(no_mangle)]
pub extern "C" fn entry(stack_pointer: userspace::target::arch::PointerType) -> ! {
    let stack_pointer = userspace::target::arch::Pointer(stack_pointer);
    let stack = userspace::memory::Stack::from_pointer(stack_pointer);
    stack.print();

    let target_path = match stack.arguments.get(1) {
        Some(argument) if !argument.pointer.0.is_null() => {
            use userspace::traits::Str;
            <&str>::from_null_terminated_pointer(argument.pointer.0 as *const u8)
        }
        _ => "/usr/bin/ls",
    };

    match userspace::file::format::elf::execute_from_path(target_path, stack_pointer.0) {
        Ok(()) => unsafe { core::hint::unreachable_unchecked() },
        Err(error) => {
            info!("ELF execution failed: {:?}\n", error);
            userspace::target::os::syscall::exit(126);
        }
    }
}
