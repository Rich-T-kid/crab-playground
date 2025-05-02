/*
Alright — h(ere’s a **clean set of mini programming assignments** after **Chapter 18 (Patterns and Matching)**:

---

**1. Match Basics**

- Write a function that takes an `i32` and:
  - Returns "Negative" if it's less than 0,
  - "Zero" if it's 0,
  - "Positive" if it's greater than 0,
using a `match` expression.

---

**2. Destructure a Tuple**

- Create a tuple `(i32, i32, i32)`.
- Write a function that uses a `match` to print the first element if it’s 0, otherwise print "Not starting with zero".

---
)
**3. Match Enums**

- Define an enum `Direction` with variants `North`, `South`, `East`, `West`.
- Write a function that matches on `Direction` and prints what movement it represents.

---

**5. Match Guards**

- Write a function that takes an `Option<i32>`.
- Use a match with a guard to:
  - Return "Even number" if Some(x) where x is even,
  - "Odd number" if Some(x) where x is odd,
  - "No number" if None.

---

**6. Binding in Patterns**

- Given a `Message::Hello { id: i32 }` enum variant,  
  match it and print if the id is in the range 3..=7, otherwise print the id.

---

**7. Ignoring parts with `_` and `..`**

- Given a large tuple `(i32, i32, i32, i32, i32)`,
  match it and only check the second and fourth elements.

---

**Extra Challenge (Harder if you want):**

**8. Nested Enums and Structs**

- Create an enum `Operation` with `Add(Point, Point)` and `Move(Direction)`.
- Match deeply inside the enum to:
  - Compute addition if it's `Add`,
  - Print direction if it's `Move`.

---

**Summary:**

| Topic | Assignment |
|:------|:-----------|
| Match basics | Matching numbers, enums |
| Destructuring | Working with tuples, structs |
| Guards | Adding extra conditions in match arms |
| Bindings | Capture matched values |
| Ignoring parts | Skip unused values |
| Nested patterns | Match inside enums inside structs |

---

Would you also want me to group these by **easy / medium / hard** like before?  
That way you can knock out the easier ones first and build momentum 🔥.  
Let’s go if you want!
*/

fn main(){
  /*let items = vec![-2,0,-20,15,200,0,80];
  for x in items.iter(){
    println!("{} is {}",*x,match_basic(*x))
  }*/
  let valid = (0,20,40);
  let invalid = (20,0,0);
  println!("{:?} is {} ",valid,match_tuple(valid));
  println!("{:?} is {} ",invalid,match_tuple(invalid))
}

fn match_basic(val:i32) -> String{
  match val{
    val if val < 0 => String::from("Negative"),
    val if val == 0 => String::from("Zero"),
    val if val > 0 => String::from("Positive"),
    _ => String::from("Invalid")
  }
}

fn match_tuple(case:(i32,i32,i32)) -> String{
  match case{
    (0,..) => format!("Starts with 0"),
    _ => format!("Doesnt start with 0")
  }
}

fn guard(val:Option<i32>) -> String{
  match val{
    Some(x) if x % 2 == 0 => format!("{x} is odd"),
    Some(x) => format!("{x} is odd"),
    None => format!("No number")
  }
}


fn partial_match(val:(i32,i32,i32,i32)) -> i32{
  match val{
    (20,_,30,_) => 2,
    _ => 0
  }
}