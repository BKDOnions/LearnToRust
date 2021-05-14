use std::fs::File;

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
    let file = match file {
        Ok(file) => file,
        Err(err_msg) => {
            panic!("File Open onError: {}", err_msg)
        }
    };
}
