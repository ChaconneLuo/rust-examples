pub fn split_at_mut(value: &mut [i32], position: usize) -> (&mut [i32], &mut [i32]) {
    let data = &mut value[..];
    if position > data.len() {
        panic!("Index out of bounds");
    }
    unsafe {
        (
            std::slice::from_raw_parts_mut(data.as_mut_ptr(), position),
            std::slice::from_raw_parts_mut(data.as_mut_ptr().add(position), data.len() - position),
        )
    }
}
