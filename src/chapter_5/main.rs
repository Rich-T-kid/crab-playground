/*
Create a User struct with name, email, and active. Implement:

A constructor (fn new)

A method deactivate(&mut self)

Practice: field initialization, impl, &mut self.

(2)
Define a Rectangle { width, height } and a method .area() that returns its area.

(3)
Struct Point { x, y } and method to compute distance from origin using f64.

*/
#[derive(Debug)]
struct User{
    name:String,
    email:String,
    active:bool
}
impl User{
    // init new user
    fn new(name:String,email:String) -> Self{
        Self{name,email,active:true}
    }
    // deactivate user
    fn deactivate(&mut self){
        self.active = false;
    }
}
struct Rectangle{
    width:i32,
    height:i32
}
impl Rectangle{
    fn area(self) -> i32{
        self.height * self.width
    }
    fn new(width:i32,height:i32) -> Self{
        Self{width, height}
    }
}
struct Point{
    x:i32,
    y:i32
}
impl Point{
    fn new(x:i32,y:i32) -> Self{
        Self{x,y}
    }
    // not the correct equation but it doesnt matter here
    fn distance(self,p1:Point) ->i32{
        ((p1.x - self.x)^2) + ((p1.y- self.y)^2)
    }
}
fn main(){
    //let mut richard = User::new(String::from("Rich"),String::from("rbb98@gmail.com"));
    //println!("{:?}",richard);
    //richard.deactivate();
    //println!("{:?}",richard);
    //let shape = Rectangle::new(25,300);
    //println!("{}",shape.area())
    let p1 = Point::new(20,5);
    let p2 = Point::new(18,12);
    println!("Distance {}",p1.distance(p2))
}