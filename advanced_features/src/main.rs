fn main() {
    let mut num = 10;

    let immut_ref = &num as *const i32;
    let mut_ref = &mut num as *mut i32;
    unsafe {
        print!("nums as immutable deference {}\n", *immut_ref);
        print!("nums as mutable deference {}", *mut_ref);
    }
}
