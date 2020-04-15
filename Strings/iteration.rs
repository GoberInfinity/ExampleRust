fn main() {
    let s = "Hello world!";
    let char_vec: Vec<char> = s.chars().collect();
    for c in char_vec {
        println!("{}", c);
    }
}