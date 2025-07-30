#![no_std]

#[cfg(feature = "macro")]
pub use syscaller_wrap_macro::wrap_syscall;

/// Make a syscall with 0 arguments.
///
/// # Safety
///
/// This function directly invokes system calls which can have undefined behavior
/// if called with invalid syscall numbers or if the kernel state is inconsistent.
/// The caller must ensure the syscall number is valid for the target system.
#[inline(always)]
pub unsafe fn syscall0(n: usize) -> isize {
    let ret: isize;
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") n,
            lateout("rax") ret,
            lateout("rcx") _,
            lateout("r11") _,
        );
    }
    ret
}

/// Make a syscall with 1 argument.
///
/// # Safety
///
/// This function directly invokes system calls which can have undefined behavior
/// if called with invalid syscall numbers, invalid arguments, or if the kernel
/// state is inconsistent. The caller must ensure the syscall number and arguments
/// are valid for the target system.
#[inline(always)]
pub unsafe fn syscall1(n: usize, a1: usize) -> isize {
    let ret: isize;
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") n,
            in("rdi") a1,
            lateout("rax") ret,
            lateout("rcx") _,
            lateout("r11") _,
        );
    }
    ret
}

/// Make a syscall with 2 arguments.
///
/// # Safety
///
/// This function directly invokes system calls which can have undefined behavior
/// if called with invalid syscall numbers, invalid arguments, or if the kernel
/// state is inconsistent. The caller must ensure the syscall number and arguments
/// are valid for the target system.
#[inline(always)]
pub unsafe fn syscall2(n: usize, a1: usize, a2: usize) -> isize {
    let ret: isize;
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") n,
            in("rdi") a1,
            in("rsi") a2,
            lateout("rax") ret,
            lateout("rcx") _,
            lateout("r11") _,
        );
    }
    ret
}

/// Make a syscall with 3 arguments.
///
/// # Safety
///
/// This function directly invokes system calls which can have undefined behavior
/// if called with invalid syscall numbers, invalid arguments, or if the kernel
/// state is inconsistent. The caller must ensure the syscall number and arguments
/// are valid for the target system.
#[inline(always)]
pub unsafe fn syscall3(n: usize, a1: usize, a2: usize, a3: usize) -> isize {
    let ret: isize;
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") n,
            in("rdi") a1,
            in("rsi") a2,
            in("rdx") a3,
            lateout("rax") ret,
            lateout("rcx") _,
            lateout("r11") _,
        );
    }
    ret
}

/// Make a syscall with 4 arguments.
///
/// # Safety
///
/// This function directly invokes system calls which can have undefined behavior
/// if called with invalid syscall numbers, invalid arguments, or if the kernel
/// state is inconsistent. The caller must ensure the syscall number and arguments
/// are valid for the target system.
#[inline(always)]
pub unsafe fn syscall4(n: usize, a1: usize, a2: usize, a3: usize, a4: usize) -> isize {
    let ret: isize;
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") n,
            in("rdi") a1,
            in("rsi") a2,
            in("rdx") a3,
            in("r10") a4,
            lateout("rax") ret,
            lateout("rcx") _,
            lateout("r11") _,
        );
    }
    ret
}

/// Make a syscall with 5 arguments.
///
/// # Safety
///
/// This function directly invokes system calls which can have undefined behavior
/// if called with invalid syscall numbers, invalid arguments, or if the kernel
/// state is inconsistent. The caller must ensure the syscall number and arguments
/// are valid for the target system.
#[inline(always)]
pub unsafe fn syscall5(n: usize, a1: usize, a2: usize, a3: usize, a4: usize, a5: usize) -> isize {
    let ret: isize;
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") n,
            in("rdi") a1,
            in("rsi") a2,
            in("rdx") a3,
            in("r10") a4,
            in("r8")  a5,
            lateout("rax") ret,
            lateout("rcx") _,
            lateout("r11") _,
        );
    }
    ret
}

/// Make a syscall with 6 arguments.
///
/// # Safety
///
/// This function directly invokes system calls which can have undefined behavior
/// if called with invalid syscall numbers, invalid arguments, or if the kernel
/// state is inconsistent. The caller must ensure the syscall number and arguments
/// are valid for the target system.
#[inline(always)]
pub unsafe fn syscall6(
    n: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    a6: usize,
) -> isize {
    let ret: isize;
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") n,
            in("rdi") a1,
            in("rsi") a2,
            in("rdx") a3,
            in("r10") a4,
            in("r8")  a5,
            in("r9")  a6,
            lateout("rax") ret,
            lateout("rcx") _,
            lateout("r11") _,
        );
    }
    ret
}
