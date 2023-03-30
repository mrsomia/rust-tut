fn main() {
    let r1 = [1,2,3,4];
    let r2 = &r1[..2];
    println!("{:?}, {:?}", r1, r2);
}
