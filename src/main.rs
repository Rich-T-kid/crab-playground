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

**4. Handle Poisoned Mutex**

- Create a `Mutex<i32>` and spawn a thread that locks the mutex and then panics.
- In the main thread, try to lock the mutex and **recover** the data using `.into_inner()` from the `PoisonError`.

---

**5. Arc + Mutex Shared State**

- Create an `Arc<Mutex<Vec<i32>>>`.
- Spawn 5 threads.
- Each thread should push a unique number into the shared vector.
- After all threads join, print the final vector (it should have 5 numbers).

---

**6. Parallel Data Processing (Mini Batch Work)**

- Given a `Vec<i32>`, split it into chunks of 5.
- Spawn a thread for each chunk that computes the sum of its numbers.
- Collect the results and print the total sum.

Example:
Input = `[1,2,3,4,5,6,7,8,9,10]`  
Thread 1 sums 1+2+3+4+5 = 15  
Thread 2 sums 6+7+8+9+10 = 40  
Total = 55

---

**7. Bounded Channel**

- Use `std::sync::mpsc::sync_channel` to create a bounded channel with capacity 2.
- Spawn a producer thread that sends 5 messages.
- Spawn a consumer thread that receives messages with a slight delay between receives.

(You’ll see the producer blocking after filling up the 2-slot buffer.)

---

**Extra challenge (Harder if you want):**

**8. Arc<Mutex<HashMap>> Cache**

- Build a cache shared across threads:
  - `Arc<Mutex<HashMap<String, String>>>`
  - Each thread inserts a key-value pair.
- After all threads are done, print the full contents of the cache.

---

**Summary:**

| Topic | Assignment |
|:-----|:-----------|
| Threads | Spawn multiple threads |
| Channels | Send and receive data between threads |
| Mutex | Safe shared mutation |
| Arc + Mutex | Shared state across threads |
| Handling panic | Deal with poisoned Mutex |
| Parallel work | Split tasks among threads |

---

Would you also want me to organize these by **easy / medium / hard** if you want a smart path to finishing them faster?  
🔥 Easy if you want to sprint through!
 */
use std::thread;
use std::time::Duration;
fn main() {
    let v = vec![1,2,3,5,6];

    let t = thread::spawn( move || {
        println!("{:?}",v);
        thread::sleep(Duration::from_millis(1));
    });
    
    t.join().expect("Couldnt join threads")


}