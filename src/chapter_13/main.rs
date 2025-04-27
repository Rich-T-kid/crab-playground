use std::{iter::Filter, path::Iter};

// CHAPTER 13 iterators and closours
/*

1. Write a function that takes a closure and a number, applies the closure to the number, and returns the result. Example call: apply(|x| x * 2, 5) -> returns 10. Hint: function signature should look like: fn apply<F>(func: F, x: i32) -> i32 where F: Fn(i32) -> i32.

2. Given a Vec<i32>, write a function that uses a closure and filter to keep only even numbers. Return the filtered numbers as a new Vec<i32>. Bonus: Make it generic so you can filter based on any condition passed by a closure.

3. Write your own my_map function that takes a vector, applies a closure to each element, and returns a new vector with transformed elements. Example: let nums = vec![1, 2, 3]; let doubled = my_map(nums, |x| x * 2); Result: doubled == vec![2, 4, 6]. Hint: Use .iter() and .map() inside your function, then collect.

4. Build a struct Counter that starts from 1, each call to .next() increments it by 1, and it stops after reaching 5. Implement the Iterator trait manually.


6. Given a Vec<i32>, chain filter, map, and sum to keep only even numbers, square them, and add up the results. Example: Input [1, 2, 3, 4] -> keep [2, 4] -> square [4, 16] -> sum 20. Write this as one chained expression.

7. Given a Vec<String>, sort it by string length using a closure with .sort_by_key(). Example: let mut words = vec!["apple".to_string(), "bat".to_string(), "zebra".to_string()]; After sorting, you get: ["bat", "apple", "zebra"].

8. Implement a mini pipeline where you take a vector, pass it through a sequence of closures, each closure modifies or filters the vector, and end with a final reduced result like a sum or join. Example idea: filter out odd numbers, square the even numbers, sum the results.


Now it’s pure text, numbered for you to paste and work through easily.  
Let me know if you want a second harder batch when you finish these.
*/
fn main(){
    let nums = vec![1,2,3,4];
    let result = operations(&nums);
    println!("result is {result}")
}
//1. Write a function that takes a closure and a number, applies the closure to the number, and returns the result. Example call: apply(|x| x * 2, 5) -> returns 10. 
//Hint: function signature should look like: fn apply<F>(func: F, x: i32) -> i32 where F: Fn(i32) -> i32.
fn apply<F>(func:F,val:i32) -> i32 where F:Fn(i32) -> i32{
    func(val)
}
//2. Given a Vec<i32>, write a function that uses a closure and filter to keep only even numbers. Return the filtered numbers as a new Vec<i32>. Bonus: Make it generic so you can filter based on any condition passed by a closure.
fn only_even<T, Filter>(elements: Vec<T>, mut f: Filter) -> Vec<T>
where
    T: Clone,
    Filter: FnMut(&T) -> bool,
{
    elements.iter().filter(|&x| f(x)).cloned().collect()
}
//3. Write your own my_map function that takes a vector, applies a closure to each element, and returns a new vector with transformed elements. Example: let nums = vec![1, 2, 3]; let doubled = my_map(nums, |x| x * 2); Result: doubled == vec![2, 4, 6]. Hint: Use .iter() and .map() inside your function, then collect.
fn my_map<T, U, F>(elements: &Vec<T>, func: F) -> Vec<U>
where
    F: Fn(&T) -> U,
{
    elements.iter().map(|x| func(x)).collect()
}

//6. Given a Vec<i32>, chain filter, map, and sum to keep only even numbers, square them, and add up the results. Example: Input [1, 2, 3, 4] -> keep [2, 4] -> square [4, 16] -> sum 20. Write this as one chained expression.
fn operations(elements: &Vec<i32>) -> i32{
    elements.iter().filter(|x| *x % 2 == 0).map(|x| x*x).sum()
}

//7. Given a Vec<String>, sort it by string length using a closure with .sort_by_key(). Example: let mut words = vec!["apple".to_string(), "bat".to_string(), "zebra".to_string()]; After sorting, you get: ["bat", "apple", "zebra"].
fn sort_str(words: &mut Vec<String>) {
    words.sort_by_key(|x| x.len());
}
//4. Build a struct Counter that starts from 1, each call to .next() increments it by 1, and it stops after reaching 5. Implement the Iterator trait manually.
struct Counter{
    val:  i32
}
impl Counter{
    fn new() -> Self{
        Self{val:0}
    }
}
impl Iterator for Counter{
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        self.val += 1;
        if self.val <=5{
            Some(self.val)
        }else{
            None
        }
    }
}
