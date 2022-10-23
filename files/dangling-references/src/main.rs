fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String{
    let s  = String::from("Reggie");

    &s;  //reference to the string s
}        //s goes out of scope and the memory is deallocated ( dangling pointer issue)

//a dangling pointer arise when objects that have an incoming reference is destructed
//without modifying the value of the pointer , so the pointer points to the deallocated
//memory


//the solution to fix this issue is to return the string directly
//instead of return a reference to the string