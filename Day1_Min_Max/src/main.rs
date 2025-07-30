fn main() {
    println!("Hello, world!");
    assert_eq!(i16::MIN, -32768);

    // type inference
    let x = 7;
    let y = 3 + 4;
    assert_eq!(x, y, "We are testing {} and {} ", x, y);

    assert_eq!(i16::MAX, 32767);
    assert_eq!(i8::MAX, 127);
    assert_eq!(i8::MIN, -128);
}
