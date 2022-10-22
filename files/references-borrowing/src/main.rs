fn main() {
    let s1 = String::from("Dattebayo");

    let l = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, l);

}

fn calculate_length(s1: &String) -> usize {
    return s1.len();      //returns length
}
