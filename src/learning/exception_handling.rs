use std::fs::File;
use std::io::ErrorKind;

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
    let file = match file {
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
    let f = File::open("shouldn't_found_this_file.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("shouldn't_found_this_file.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    // Recoverable Panic
    // TODO shortcut panic: unwrap and expect
}
