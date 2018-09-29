pub trait Summary {
    fn summarize(&self) -> String;
}
struct Tweet {
    text: String
}
impl Tweet{
    fn set_text(&mut self,text:String){
        
        self.text = String::from(&text[..])
    }
    fn get_text(&self)->String{
        
        String::from(&self.text[..])
    }
}
impl Summary for Tweet{
    fn summarize(&self)->String {
        format!("{}: {}", "tweet", self.text)        
    }
}
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
fn main() {
    let mut t = Tweet{text:String::from("yo1")};
    t.set_text(String::from("yo"));
    println!("text is {}",t.text);
    println!("text is {}",t.get_text());
    println!("summary is {}",t.summarize());
}
