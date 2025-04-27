/*
Alright — here’s a **clean set of mini programming assignments** after **Chapter 18 (Patterns and Matching)**:

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

**3. Match Enums**

- Define an enum `Direction` with variants `North`, `South`, `East`, `West`.
- Write a function that matches on `Direction` and prints what movement it represents.

---

**4. Destructure Structs**

- Define a `Point` struct with `x` and `y` fields.
- Write a function that matches on the struct to check:
  - If it’s on the x-axis (y == 0),
  - y-axis (x == 0),
  - Or neither.

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

fn main(){}