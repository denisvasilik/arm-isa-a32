
#[inline]
pub fn add(a: i32, b: i32) -> i32 {
    let c: i32;
    unsafe {
        llvm_asm!("add $0, $1, $2" : "=r"(c): "r"(a), "r"(b) :: "volatile")
    }
    c
}
