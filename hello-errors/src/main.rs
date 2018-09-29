use std::io;
use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;

/*the ? at the end of the File::open call will return 
the value inside an Ok to the variable f. If an error occurs, ? will 
return early out of the whole function and give any Err value to the calling code. T
he same thing applies to the ? at the end of the read_to_string call.

 */
fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut f = File::open("main.rs")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("src/main.rs");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn main() {
    let f = File::open("hello.txt");

    let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
            },
            other_error => panic!("There was a problem opening the file: {:?}", other_error),
        },
    };

/* If the Result value is the Ok variant,
 unwrap will return the value inside the Ok. 
 If the Result is the Err variant, unwrap will call the panic! macro for us. 
 Here is an example of unwrap in action:*/

    let _f2 = File::open("hello.txt").unwrap(); 

    /* expect, which is similar to unwrap,
     *  lets us also choose the panic! error message. Using expect instead of unwrap and
     *  providing good error messages can convey your intent and make tracking down the source of a panic easier */

    let _f3 = File::open("hello.txt").expect("Failed to open hello.txt");

    let output =  read_username_from_file().unwrap();
    println!("nane is {}", output);
}