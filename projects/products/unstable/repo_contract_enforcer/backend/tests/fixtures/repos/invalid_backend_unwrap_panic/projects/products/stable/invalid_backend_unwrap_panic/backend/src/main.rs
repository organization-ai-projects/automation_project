fn main() {
    let v = Some(1);
    let _x = v.unwrap();
    panic!("boom");
}
