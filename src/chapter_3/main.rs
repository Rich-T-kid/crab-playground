// project after completing chapter 3 of the rust book
fn main(){
   // let temp : i32 = 400;
    //println!("original Temp:{} to Celsius {}",temp,toCel(temp))
    //let n = 7;
    //println!("{}'th fibonachi number is {} ",n , fib(n))
}
// 1 , 1 , 2 , 3 , 5 , 8, 13 , 21, 34 ....
fn fib(n : i32) -> i32{
    if n <= 2{
        return 1;
    }
    return fib(n-1) + fib(n-2);
}
fn toCel(Fahrenheit: i32) -> i32{
    (Fahrenheit - 32) * 5/9
}

fn toFar(Cel:i32) -> i32{
    (Cel * 9/5) + 32
}
/*
Convert temperatures between Fahrenheit and Celsius.
•	 Generate the nth Fibonacci number.
•	 Print the lyrics to the Christmas carol “The Twelve Days of Christmas,”
taking advantage of the repetition in the song

*/