pub fn align_up(value: i32, alignment: i32) -> i32 {
    ((value + alignment - 1) / alignment) * alignment
}