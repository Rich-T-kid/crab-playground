// lifeTimes, genrics , traits
// 1. Generic Stack
// Create a Stack<T> struct that uses Vec<T> internally.
// Implement push, pop, peek, and a method to check if the stack is empty.


// 2. Generic Max Function
// Write a function:
// fn find_max<T: PartialOrd>(list: &[T]) -> Option<&T>
// It should return the largest element in a slice or None if the slice is empty.
struct Stack<T:PartialOrd>{
    internal_stack: Vec<T>
}
impl<T:PartialOrd> Stack<T>{
    fn new() -> Self{
        Self{internal_stack:vec![]}
    }
    fn push(&mut self,value: T) {
        self.internal_stack.push(value);
    }
    fn pop(&mut self) -> Option<T>{
        self.internal_stack.pop()
    }
    fn peak(&self) -> &T{
        let last_idx = self.internal_stack.len() - 1;
        &self.internal_stack[last_idx]
    }
    fn is_empty(&self) -> bool{
        self.internal_stack.len() == 0
    }
}
fn max_val<T:PartialOrd>(collection :&Vec<T>) -> &T{
    let mut result = &collection[0];
    for element in collection.iter(){
        if *element > *result{
            result = &element
        }
    }
    result
}
fn main(){
    /*let mut stack = Stack::new();
    stack.push(20);
    stack.push(15);
    println!("peak result: {}",stack.peak());
    stack.push(92);
    println!("peak result: {}",stack.peak());
    stack.push(9023);
    println!("peak result: {}",stack.peak());

    stack.pop();
    stack.pop();
    println!("peak result: {}",stack.peak());
    println!("stack empty?: {}",stack.is_empty());
    while !stack.is_empty(){
        println!("peak result: {}",stack.peak());
        let value = match stack.pop(){
            Some(v) => v,
            None => 0,
        };
        println!("found value {} in stack",value)
    }
    let numbers = vec![10, 50, 30, 20, 40];
    let floats = vec![1.5, 2.7, 0.3, 5.5, 4.2];
    let letters = vec!['a', 'z', 'b', 'y', 'c'];
    let words = vec!["apple", "banana", "pear", "grape", "orange"];

    println!("Max number: {}", max_val(&numbers));
    println!("Max float: {}", max_val(&floats));
    println!("Max letter: {}", max_val(&letters));
    println!("Max word: {}", max_val(&words));
    */  
    let square = Square{
        l:10,
        w:20
    };
    let rectangle = Rectangle{
        l:10,
        w:20
    };
    draw(&square);
    draw(&rectangle);

}

pub trait Drawable {
    fn draw(&self) -> String;
}
struct Rectangle{
    l: i32,
    w: i32
}
struct Square {
    l: i32,
    w: i32

}
impl Drawable for Rectangle {
    fn draw(&self) -> String {
        let area = self.l * self.w;
        format!("Drawing a rectangle with {} area",area)
    }
}
impl Drawable for Square {
    fn draw(&self) -> String {
        let area = self.l * self.w;
        format!("Drawing a Square with {} area",area)
        
    }
}
fn draw<T:Drawable>(canvas: &T) {
    println!("{} ", canvas.draw());
}
fn longest<'a>(x:&'a str,y:&'a str) -> &'a str{
    if x.len() > y.len(){
        return x
    }
    y
}
// 1. Drawable Trait
// Define a trait Drawable with a method draw(&self) -> String.
// Implement Drawable for two types, like Circle and Square.
// Write a function that takes a slice of &dyn Drawable and calls draw on each.



// 1. Longest String
// Write a function:
// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str
// It should return the longer of the two string slices.


// 2. ImportantExcerpt Struct
// Define a struct ImportantExcerpt<'a> with a field part: &'a str.
// Implement a method on ImportantExcerpt that returns the part with an added comment like "Important: {part}".
