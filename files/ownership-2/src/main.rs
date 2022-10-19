use std::mem::take;

fn main() {

    let s1 = gives_ownership();

    let s2 = String::from("Sensei");

    let s3 = takes_and_gives_back(s2);  //s2 -> takes_and_gives_back -> s3
}

fn gives_ownership() -> String{

    let s = String::from("Sarutobi"); //string has come into scope

    return s; //string is returned
}

fn takes_and_gives_back(s:String) -> String{

    return s;  //returns String
}
 //errors like unused variable is to be seen