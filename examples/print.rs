fn main() {
    for arg in argv::iter() {
        println!("{}", arg.to_string_lossy());
    }
}
