fn main() {
    let s1 = String::from("Hashirama");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() function returns the length of a String
    return (s, length);
}