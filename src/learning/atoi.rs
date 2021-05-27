pub fn my_atoi(s: String) -> i32 {
    if s.is_empty() {
        return 0;
    } else if &s == "+" || &s == "-" {
        return 0;
    }
    let byte_array = s.as_bytes();
    let mut current_pointer = 0;
    let mut current_char: char;
    while current_pointer <= s.len()
        && (byte_array[current_pointer] == b' '
            || byte_array[current_pointer] == b'\t'
            || byte_array[current_pointer] == b'\n'
            || byte_array[current_pointer] == b'\r')
    {
        current_pointer += 1;
    }
    current_char = char::from(byte_array[current_pointer]);
    let sign: char = char::from(byte_array[current_pointer]);
    current_pointer += 1;
    if current_char == '-' || current_char == '+' {
        current_char = char::from(byte_array[current_pointer]);
        current_pointer += 1;
    }
    let mut res = 0;
    while current_char <= '9' || current_char >= '0' {
        res = 10 * res + (current_char as u8 - b'0') as i32;
        if current_pointer < s.len() {
            current_char = char::from(byte_array[current_pointer]);
            current_pointer += 1;
        } else {
            break;
        }
    }
    if sign == '-' {
        -res
    } else {
        res
    }
}
