/*
•	 Given a list of integers, use a vector and return the mean (the average
value), median (when sorted, the value in the middle position), and
mode (the value that occurs most often; a hash map will be helpful
here) of the list.

•	 Convert strings to pig latin. The first consonant of each word is moved
to the end of the word and “ay” is added, so “first” becomes “irst-fay.”
Words that start with a vowel have “hay” added to the end instead
(“apple” becomes “apple-hay”). Keep in mind the details about UTF-8
encoding!
•	 Using a hash map and vectors, create a text interface to allow a user to
add employee names to a department in a company. For example, “Add
Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve
a list of all people in a department or all people in the company by
department, sorted alphabetically 
*/
use std::{collections::HashMap, hash::Hash};
fn main() {
    /*let mut nums = vec![
    3, 1, 4, 4, 5, 2, 6, 8, 9, 10, 3, 4, 7, 8, 9, 1, 2, 4, 6, 5,
    4, 4, 5, 3, 2, 1, 9, 7, 8, 6, 4, 4, 2, 3, 1, 5, 6, 7, 9, 10,
    ];
    let (mean,median,mode) = vec_stats(&mut nums);
    println!("mean:{mean}\t median:{median}\t mode:{mode}")
    
    let mut manager = Manager::new();

    manager.add_employee("Alice".into(), department::Engineering);
    manager.add_employee("Bob".into(), department::Engineering);
    manager.add_employee("Charlie".into(), department::Sales);

    manager.view_users(department::Engineering); // Alice, Bob
    manager.view_users(department::Sales);       // Charlie
    manager.view_users(department::Finance);     // None
     */

}
struct Manager{
    employee_to_depart: HashMap<String,department>, // Employee -> Department they belong to
    depart_to_employee: HashMap<department,Vec<String>> //Department -> list of employees
}
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum department {
    Engineering,
    Sales,
    Tech,
    Finance,
}
impl Manager{
    fn new() -> Self{
        Self{employee_to_depart:HashMap::new(),depart_to_employee: HashMap::new()}
    }
    fn add_employee(&mut self,name:String,department: department){
        // set up employee mapping and add that element to the Department
        self.employee_to_depart.insert(name.clone(), department.clone());
        // add name to department list
        self.depart_to_employee
            .entry(department)
            .or_insert(Vec::new())
            .push(name);
    }
    fn view_users(&self,department: department){
         match self.depart_to_employee.get(&department) {
            Some(users) => {
                println!("Employees in {:?}: {:?}", department, users);
            }
            None => {
                println!("No employees in {:?}", department);
            }
        }
    }
}









fn vec_stats(nums: &mut Vec<i32>)-> (i32,i32,i32){
    let mut table: HashMap<i32,i32> = HashMap::new();
    let mut mean = 0;
    for i in nums.iter(){
       table.entry(*i).and_modify(|v| *v += 1).
       or_insert(1);
       mean += *i
    }
    let size = nums.len() as i32;
    nums.sort();
    let middle = if nums.len() % 2 == 0 {
        let mid1 = nums[nums.len() / 2 - 1];
        let mid2 = nums[nums.len() / 2];
        (mid1 + mid2) / 2
        } else {
            nums[nums.len() / 2]
        };

    let max_key = table
        .iter()
        .max_by_key(|entry| entry.1)
        .map(|(key, _val)| key);
    let max_key = match max_key{
        Some(k)=> *k,
        None => 0
    };
    (mean/size,middle,max_key)
}
