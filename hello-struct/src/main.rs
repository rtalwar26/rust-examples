#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
    label: String
}
impl Rectangle{
    fn area(&self)->u32{
        return self.width * self.height;
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(side:u32) -> Rectangle {
        Rectangle {width:side,height:side,label:String::from("yo")}
    }
}
fn main() {
    let rect1 = Rectangle { width: 30, height: 50,label:String::from("yo") };
    let rect2 = Rectangle { width:30,height: 49,label:String::from("yo")};

    let square = Rectangle::square(30);
    println!("can hold {}",rect1.can_hold(&rect2) );
    println!("area of rect2 {}",rect2.area());
    println!("rect1 is {:#?}", rect1);
    println!("rect1 is {:?}", rect1);
    println!("rect1 area is {}", rect1.area());
    println!("square  is {:#?}", square);
    
}