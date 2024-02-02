fn main(x : u64, y : u64, z: pub u64) {
    assert((x*y)-z==0);
}

#[test]
fn test_main() {
    main(5,5,25);
}
