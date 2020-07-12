fn main(){
    let mut a = 1;
    let b = 2;
    let c = {
            a = a + b;
            a = b
        };
    a = 1;
    println!("{:?}", a = b);
    println!("{:?}", a);
    println!("{:?}", c);
}