use std::fs::File;
use std::io::{Error, ErrorKind, Read};

/// Almost all programming languages has exception handling, not if trashes
///```
/// enum Result<T, E> {
///     Ok(T),
///     Err(E),
/// }
/// ```
///By using `enum Result`, exceptions can be easily handled with proper warnings
///
pub fn exception_handling_detail() {
    let file = File::open("shouldn't_found_this_file.txt");
    // Matching Err Options
    // let file = match file {
    //     Ok(file) => file,
    //     Err(err_msg) => {
    //         panic!("File Open onError: {}", err_msg)
    //     }
    // };
    let _file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("shouldn't_found_this_file.txt") {
                Ok(file_created) => file_created,
                Err(err_msg) => panic!("Problem(s) at creating file, msg: {:?}", err_msg),
            },
            other_error_msg => panic!("Problem(s) at opening file, msg: {:?}", other_error_msg),
        },
    };
    // Same by using closure
    let _f = File::open("shouldn't_found_this_file.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("shouldn't_found_this_file.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    // Recoverable Panic
    let _file = File::open("shouldn't_found_this_file.txt").expect("Failed To Open The File shouldn't_found_this_file.txt");
    let _file = std::fs::remove_file("shouldn't_found_this_file.txt").unwrap();

    println!("{:?}", read_filename_from_file());
}

// Err(Os { code: 2, kind: NotFound, message: "The system cannot find the file specified." })
fn read_filename_from_file() -> Result<String, Error> {
    let file = File::open("shouldn't_found_this_file.txt");
    let mut file = match file {
        Ok(file) => file,
        Err(err_msg) => return Err(err_msg)
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(err_msg) => Err(err_msg)
    }
}
