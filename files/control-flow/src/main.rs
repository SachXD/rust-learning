fn main() {
    let number = 3;
    if number < 5 {
        println!("number is less than 5");
    }
    else if number != 0{
        println!("number is something other than 0");
    }
    else {
        println!("number is greater than or equal to 5");
    }
}

fn meg(){
    let condition = true;
    let number = if condition{
        5
    }
    else{
        6
    };
    println("The value of number is : {}", number);
}

fn mangekyou(){
    let condition = true;
    let number = if condition{
        5
    }
    else{
        "six"                //this wont work since variable can be evaluated
                             //to only a single type and not both
    };
    println("The value of number is : {}", number);
}

fn rinnegan(){
    loop {
        println!("And this world shall know pain!") //infinite loop ; press CTRL + C to stop
    }
}

fn motel() {
    let number = 3;
    while number != 0 {
        println!("number is {}", number); //using while loop!

        number = number - 1;
    }
    println!("SIKE!")
}

fn sigma() {
    let arr = [1, 2, 3, 4, 5];
    let mut i = 0;

    while i < arr.len() {
        println!("Your Number is : {}", arr[i]); //finally using arrays and some usual stuff
        i = i + 1;  //this method of iterating is slow compared to the below code
    }
}

fn alpha() {
    let a = [12,23,34,45,56];

    for element in a.iter(){
        println!("the value is: {}", element); //this .iter() ensures safety in the code making sure no out-of bounds can occur
    }
}

