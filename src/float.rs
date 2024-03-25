pub fn compare_float(){
    println!("{}", -f32::MIN);
    println!("{}", f32::MAX);

    assert_eq!(-f32::MIN, f32::MAX);
}