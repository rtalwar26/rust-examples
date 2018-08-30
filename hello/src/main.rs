use std::io::Write;
use std::str::FromStr;

fn main() {    
    let mut numbers = Vec::new();
    for arg in std::env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error parsing argument"))
    }
    
    if numbers.len() == 0 {
        writeln!(std::io::stderr(),"Usage: gcd NUMBER...").unwrap();
        std::process::exit(1);
    }
    let mut d = numbers[0];
    for m in &numbers[1..]{
        d = gcd(d,*m);
    }
    println!("The gcd of {:?} is {}",numbers,d );
}

fn gcd(mut m : u64, mut n : u64)->u64{
  assert!(n!=0 && m!=0);
  while m != 0 {
      if m < n{
          let t = m;
          m = n;
          n = t;
      }
      m = m %n ;
  }   
  n
}

#[test] //#[test] is an example of attribute
fn test_gcd(){
    assert_eq!(gcd(6,9),3 );
    assert_eq!(gcd(15,10),5 );
    assert_eq!(gcd(2*3*5*7*11,3*11*13*19),3*11);
}