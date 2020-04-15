fn main(){
    //The standard library integer types have checked_* methods for common operations
    assert_eq!((u32::max_value() - 2).checked_add(1), Some(u32::max_value() - 1));
    assert_eq!((u32::max_value() - 2).checked_add(3), None);
}