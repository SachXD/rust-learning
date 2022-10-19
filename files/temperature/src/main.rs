fn main() {
    let celtemp = 42;
    let fahtemp = 56;

    println!("Your temperature in fahrenheit {}", c2f(celtemp));
    println!("Your temperature in celsius {}", f2c(fahtemp));
}

fn c2f(temp :i32) -> f64 {

    let fah = (temp * 9)/5 + 32 ; //conversion of celsius to fahrenheit
    return fah as f64 ; //returns fahrenheit temperature
}

fn f2c(temp :i32) -> f64 {

    let cel = ((temp - 32) * 5)/9 ; //conversion of fahrenheit to celsius
    return cel as f64 ; //returns celsius temperature
}
