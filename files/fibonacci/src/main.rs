fn main()
{
    let n = 8;
    println!("Your Fibonacci number is : {}" , fib(n));
}
fn fib(nums :i32) -> i32 {
    if nums < 2
    {
        return nums;
    }
    return fib(nums - 2) + fib(nums - 1);
}
