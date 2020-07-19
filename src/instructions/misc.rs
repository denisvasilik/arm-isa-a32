
/// A no-operation. Useful to prevent delay loops from being optimized away.
#[inline]
pub fn nop() {
    unsafe {
        llvm_asm!("nop" :::: "volatile")
    }
}

#[inline]
pub unsafe fn svc(id: usize, nr: usize, arg: usize) -> usize {
    let mut asm_nr = nr;
    llvm_asm!("svc 0x00123456" : "+{r0}"(asm_nr) : "{r1}"(arg) :: "volatile");
    asm_nr
}
