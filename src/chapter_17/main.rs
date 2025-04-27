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

**3. State Pattern with Box<dyn Trait>**

- Model a simple blog post:
  - Create a `Post` struct that holds a state inside a `Box<dyn State>`.
  - States: `Draft`, `PendingReview`, `Published`.
- Implement methods like `request_review()` and `approve()` that move the post to the next state.

---

**4. Polymorphism with Dynamic Dispatch**

- Build a trait `Drawable` with a method `draw(&self)`.
- Create two structs `Circle` and `Square` that implement `Drawable`.
- Write a function `render_scene` that takes a `&[Box<dyn Drawable>]` and calls `draw` on each one.

---

**5. Implement Static Dispatch with Generics**

- Instead of using `dyn Trait`, write a function that uses generics with trait bounds.
- Example: A function `print_twice<T: Animal>(animal: T)` that calls `speak` twice.

---

**6. Default Methods in Traits**

- Create a trait `Summary` with a default implementation of `summarize(&self) -> String`.
- Implement `Summary` for a `NewsArticle` struct.
- Override the default for a `Tweet` struct.

---

**7. State Machine: Light Switch**

- Create a `LightSwitch` struct.
- It can be either `On` or `Off` using a State trait.
- Implement `turn_on()` and `turn_off()` methods that change its internal state.

---

**Extra Challenge (Harder if you want):**

**8. Complex State Pattern: Download Manager**

- Model a download manager with states:
  - `NotStarted`
  - `Downloading`
  - `Completed`
  - `Failed`
- Each state should have different allowed transitions.

---

**Summary:**

| Topic | Assignment |
|:------|:-----------|
| Traits | Build traits like interfaces |
| Trait Objects | Dynamic polymorphism |
| State Pattern | Model changing internal state with traits |
| Static Dispatch | Use generics instead of `dyn` |
| Default Methods | Provide default behavior in traits |
| State Machine | Model a system that changes behavior over time |

---

Would you want me to also group these by **easy / medium / hard** like I offered for Chapter 16?  
It would give you a clean sprint path if you want to knock them out fast! 🔥 */
fn main(){}