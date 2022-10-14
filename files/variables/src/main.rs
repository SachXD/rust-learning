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
                            
    let decimal = 98_222; //decimal
    let hex = 0xff; //hexadecimal
    let octal = 0o77; //octadecimal
    let binary = 0b1111_0000; //binary number
    
    //addition
    let sum = 4 + 5;

    //subtraction
    let difference  = 5 - 4;

    //multipication
    let product = 4*30;

    //division
    let quotient = 56.7 / 32.2;

    //remainder
    let remainder = 43 % 5;

    //boolean type
    let t = true;

    let f:bool = false; //explicit type annotation
    
    //character type
    let c = 'z';
    let f = 'x'; //alphabets, chinese,mandarin,japanese is supported in-built in the programming language
                 // even emojis!! how cool is rust!
    
    //COMPOUND TYPES INCOMINGG!

    //tuples
    let tup: (i32,f64,u8) = (500,6.4,1);

    let (x,y,z) = tup;

    println!("The value of y is : {}",y); //this process is called destructing.
                                          //splitting the contents in a tuple into individual parts

    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    //acessing each individual element like we do in java and other langs

    //arrays
    let a = [1,2,3,4,5,6];

    //always vectors when you're confused of using which to use among arrays and vectors.

    let first = a[0];
    let second = a[1];
    
    another_function;
    not_another_function(1,2);
}
fn another_function(){
    println!("Hello, sekhai");
}

fn not_another_function(x: i32, y: i32){
    println!("The value of x is :  {}", x);
    println!("The value of y is : {}",y);
}
