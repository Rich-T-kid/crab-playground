/*Good — here’s a clean set of **mini programming assignments** after **Chapter 17 (Object-Oriented Programming in Rust)**:

---

**1. Traits as Interfaces (Basic OOP)**

- Define a trait `Animal` with a method `speak(&self) -> String`.
- Implement `Animal` for `Dog` and `Cat` structs.
- Write a function that takes a `&dyn Animal` and prints what it says.

---

**2. Trait Objects in a Collection**

- Create a `Vec<Box<dyn Animal>>` holding different types (Dog, Cat, etc.).
- Loop through the vector and call `speak` on each animal.

---

---
--





Would you want me to also group these by **easy / medium / hard** like I offered for Chapter 16?  
It would give you a clean sprint path if you want to knock them out fast! 🔥 */
fn main(){
    let pet = Dog{name:"Richard".to_string()};
    let pet1 = Cat{name:"Charles".to_string()};
    //speaker(pet);
    //speaker(pet1);
    let zoo:Vec<Box<dyn Animal>> = vec![Box::new(pet),Box::new(pet1)];
    group_speakers(zoo);
}
/*  
**1. Traits as Interfaces (Basic OOP)**

- Define a trait `Animal` with a method `speak(&self) -> String`.
- Implement `Animal` for `Dog` and `Cat` structs.
- Write a function that takes a `&dyn Animal` and prints what it says.
*/

trait Animal {
    fn speak(&self) -> String;
}

struct Dog{
    name:String
}

struct  Cat{
name:String
}
impl Animal for Dog {
    fn speak(&self) -> String {
        format!("my name is {}",self.name)
    }
}
impl Animal for Cat {
    fn speak(&self) -> String {
        format!("my name is {}",self.name)
    }
}

fn speaker<A:Animal>(animal : A){
    println!("{}",animal.speak())
}

fn group_speakers(col:Vec<Box<dyn Animal>>){
    for i in col.iter(){
        println!("{}",i.speak())
    }
}