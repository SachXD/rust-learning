fn main() {
    let s  = String::from("hello world");

    takes_ownership(s);  //after this you cannot use s

    let x  = 5;

    makes_copy(x);   //after this you cannot use x
}

fn takes_ownership(some_string : String){
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32){
    println!("{}", some_integer);

}