// Fearless concurenncy
/*Perfect — here’s a clean list of **mini programming assignments** after **Chapter 16 (Fearless Concurrency)**:

---

**1. Basic Thread Spawning**

- Write a program that spawns 3 threads.
- Each thread prints "Hello from thread X" where X is the thread number.
- Use `thread::sleep` to stagger their printing so you can see the order.

---

**2. Message Passing with Channels**

- Create a `mpsc` channel (multi-producer, single-consumer).
- Spawn 2 threads that each send 5 numbers into the channel.
- The main thread should receive all the numbers and print them as they come in.

---

**3. Shared Counter with Mutex**

- Create a `Mutex<i32>` inside an `Arc`.
- Spawn 10 threads that each increment the counter by 1.
- After joining all threads, print the final counter value (it should be 10).

---


**4. Arc + Mutex Shared State**

- Create an `Arc<Mutex<Vec<i32>>>`.
- Spawn 5 threads.
- Each thread should push a unique number into the shared vector.
- After all threads join, print the final vector (it should have 5 numbers).
 */
use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
fn main() {
    /*let v = vec![1,2,3,5,6];
    let mut threads = vec![];
    for i in 0..5{
    let t = thread::spawn( move || {
      println!("Hello from thread {}",i)
    });
    threads.push(t);
  }
  
  for t in  threads{
    t.join().unwrap()
  }
  
  let (tx,rx) = mpsc::channel();
  thread::spawn(move||{
    for i in 0..5{
      tx.send(i).unwrap();
    }
  });
  for i in rx{
    println!("Recievd {i}")
  }
  println!("Done!")
  */
  let m = Arc::new(Mutex::new(0));
  for _ in  0..10{
    let mtx = Arc::clone(&m);
    thread::spawn(move || {
      let mut c = mtx.lock().unwrap();
      println!("Counter is currently at {}, I'll increment it to {}", *c, *c + 1);
      *c += 1;
    });
  }
  println!("Counter ended at {}",m.lock().unwrap())
}