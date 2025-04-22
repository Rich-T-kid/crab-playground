/*
 Message Enum
Define an enum Message with variants:

Text(String)

Ping

Move { x: i32, y: i32 }

Write a process(msg: Message) function that prints different output for each.
(2)
Implement -> fn maybe_add(a: Option<i32>, b: Option<i32>) -> Option<i32>
(3)
Define an enum:
enum Command {
    Quit,
    Help,
    Say(String),
}
Write a loop that takes string input ("quit", "help", etc.), matches it to a variant, and handles the command.

Practice: enums, string parsing, match control flow.

*/
enum Message{
    Text(String),
    Ping,
    Move{x:i32,y:i32}
}
fn manage_message(m:Message){
    match m{
        Message::Text(str) => println!("User passed in {}",str),
        Message::Ping => println!("User wants to check server is live"),
        Message::Move{x,y} => println!("Moving to x:{x} , y:{y} ")
    }
}
fn maybe_add(a: Option<i32>, b: Option<i32>) -> Option<i32>{
   if let (Some(x), Some(y)) = (a, b) {
        Some(x + y)
    } else {
        None
    } 
}
enum Command {
    Quit,
    Help,
    Say(String),
}
fn handleCommand(cmd : Command){
    match cmd{
        Command::Quit => println!("Exiting program"),
        Command::Help => println!("User Needs help"),
        Command::Say(str) => println!("User said {}",str),
    }
}
fn main(){
    //let m = Message::Text(String::from("Im the new pirate king"));
    //let m1 = Message::Ping;
    //let m2 = Message::Move{x:32,y:40};
   // manage_message(m);
    //manage_message(m1);
   // manage_message(m2);
    //let a = Some(5);
    //let b = Some(22);//Some(6);
    //let result = maybe_add(a,b);
    //println!("result:{:?}",result);
    let cmd = Command::Quit;
    let cmd1 = Command::Help;
    let cmd2 = Command::Say(String::from("Hello there"));
    handleCommand(cmd);
    handleCommand(cmd1);
    handleCommand(cmd2);
}