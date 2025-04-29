use std::vec;

//Smart Pointers
/*

Box<T> mini-assignments:

1. Implement a basic **Linked List** using `Box`.  
   Each node should hold a number and a `Box<Node>` pointing to the next node (or None at the end).

2. Build a simple **Tree** where each node has a value and a single child, using `Box`.  
   Walk through the tree and print all the values.

3. Create a **recursive enum** for an arithmetic expression: `Add`, `Mul`, and `Value`, using `Box` to nest expressions.  
   Write a function that evaluates the expression.

---

Rc<T> mini-assignments:

1. Build a **shared tree** where multiple branches share the same child node.  
   Use `Rc` so multiple parents can point to the same leaf.

2. Create a simple **graph** where two nodes share a reference to a common third node (like A and B both pointing to C).

3. Write a **reference counter tracker**:  
   Create an `Rc<String>`, clone it a few times, and print how many strong references exist after each clone and drop.

---

RefCell<T> mini-assignments:

1. Build a **mutable counter** where you wrap an integer inside `RefCell` so you can mutate it even if the outer struct is immutable.

2. Create a **shared cache**:  
   A struct that holds a `RefCell<HashMap<String, String>>`, so you can insert and read from the cache even through shared references.

3. Combine `Rc<RefCell<T>>`:  
   Make a **shared mutable linked list** where multiple parts of your program can hold shared references to the list and still mutate the next node.

---

**Quick cheat summary of what you’re hitting:**

| Smart Pointer | Mini Projects build...  |
|:--------------|:-------------------------|
| Box<T>        | Ownership, heap allocation, recursion |
| Rc<T>         | Shared ownership, reference counting |
| RefCell<T>    | Interior mutability, runtime borrow checking |

---

*/
fn main(){
}

//Smart Pointers
/*
Box<T> mini-assignments:

1. Implement a basic **Linked List** using `Box`.  
   Each node should hold a number and a `Box<Node>` pointing to the next node (or None at the end).

2. Build a simple **Tree** where each node has a value and a single child, using `Box`.  
   Walk through the tree and print all the values.

3. Create a **recursive enum** for an arithmetic expression: `Add`, `Mul`, and `Value`, using `Box` to nest expressions.  
   Write a function that evaluates the expression.

---
*/

struct Node<T>{
   value: T,
   next:Option<Box<Node>>
}
impl Node<T>{
   fn new<T>(value:T) -> Self{
      Self{value,next:None}
   }
   fn append<T>(&mut self, other: Node<T>) {
      self.next = other
   }
}