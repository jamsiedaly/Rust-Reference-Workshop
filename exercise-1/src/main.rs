fn main() {
    let name: String = "Your name".to_string();

    println!("{} address is {:p} on the stack", name, &name);
    println!("{} address is {:p} on the heap", name, name.as_ptr());
}
