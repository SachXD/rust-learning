fn main() {
    let y = 6;

    let x = (let y = 6); // this will not evaluate to anything and it was cause an error
                         // because x has nothing to bind to.
    let y = {
        let x = 3;  //this is a statement

        x + 1       //this is is a expression    
    };

    let x = five();

    println!("The value of x is: {}", x);
}

fn five() -> i32 {
    5
}

fn plus_one(x :i32) ->i32{
    x + 1
}
