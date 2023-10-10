fn main() {
    let mut i:i32=0;
    i+=1;
    println!("{i}");
    let (mut x,y)=(10,20); // destructuring
    x+=10;
    println!("x: {x}, y: {y}");

    let n1=0.1;//f64 as def
    let n2=0.2;
    let res1=n1+n2==0.3;
    println!("n1+n2==0.3?{}",res1);
    let res2=n1 as f32 +n2 as f32 ==0.3_f32; //lowering the precision, they are equal...
    println!("n1+n2==0.3?{}",res2);

    let s:String=String::from("hello");
    let mut s1=s;
    s1.push_str(" world");
    println!("{s1}");
}
