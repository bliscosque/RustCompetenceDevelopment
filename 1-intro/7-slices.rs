fn main() {
    let a=[1,2,3,4,5];
    let slice=&a[1..3]; //type: &[i32]
    println!("{:?}",slice);
}
