use std::{fs::{File, self}, io::ErrorKind, io::Read, io};
fn main() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating tje file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating tje file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    let f = File::open("hello.txt").unwrap();
    let f = File::open("hello.txt").expect("Custom panic message");

    let result = provide_exeptions();
    let result2 = provide_exeptions2();
    let result3 = provide_exeptions3();
    let result4 = provide_exeptions4();
}
fn provide_exeptions() -> Result<String, io::Error> {
    let f = File::open("user_name.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(error) => return Err(error),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(error) => Err(error),
    }
}
fn provide_exeptions2() -> Result<String, io::Error> {
    let mut f = File::open("user_name.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
fn provide_exeptions3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("user_name.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
fn provide_exeptions4() -> Result<String, io::Error> {
    fs::read_to_string("user_name.txt")
}