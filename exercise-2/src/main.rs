fn main() {
    let a = 1;
    let b = Box::new(a);

    println!("{:p}", &a);
    println!("{:p}", &b);
    println!("{:p}", b);
}