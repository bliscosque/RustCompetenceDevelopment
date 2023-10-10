fn main() {
    let x=10;
    let z= { 
        2*x; //statement, does not evaluate to a value so z=()
    };
    println!("z: {:?}",z);
    let z={
        2*x // expression... evaluates to a value that is "returned" to z
    };
    println!("z: {}",z);
}
