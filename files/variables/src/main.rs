fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}",x);

    const MAX_POINTS: u32 = 100_00; //these are constants which are always immutable
    
    let spaces = "  ";
    let spaces = space.len(); //this wont be an error since we are declare another variable space
                             //which is different from the other which is a string.


    let mut spaces = "  ";  //meanwhile this can cause a error since you declared it as a mutable
    spaces = spaces.len(); //we can cannot change a string type to int (obviously!)
                            
}
