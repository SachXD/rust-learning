// fn main() {
//     let s  = String::from("Senju");
//
//     change(&s);
// }

// fn change(some_string: &String) {
//
//     some_string.push_str(", Tsunade");  //this will be an error because you cannot
//                                                 //borrow immutable as mutable in rust
//
// }

fn main() {
    let mut s = String::from("Senju");
    change(&mut s);
}
fn change(some_string: &mut String) {
    some_string.push_str(", Tsunade");
}

//here we changed string to a mutable and we can make the reference mutable as well
//but mutable references has a big problem : you can have only one
//mutable reference to a particular piece of data in a particular scope