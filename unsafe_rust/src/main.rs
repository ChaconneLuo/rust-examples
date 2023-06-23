use unsafe_rust::split_at_mut;
fn main() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is {}", *r1);
        println!("r2 is {}", *r2);

        let ref_address = 0x0000004usize;
        let _r = ref_address as *const i32;
        // println!("r is {}", *r);
    }

    let mut vec_array = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = split_at_mut(&mut vec_array, 3);
    println!("left is {:?}", left);
    println!("right is {:?}", right);
}
