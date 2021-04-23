pub fn show_data_types() {
    // primitive types
    const MAX_8BIT: i8 = std::i8::MAX;
    const MIN_8BIT: i8 = std::i8::MIN;
    const MAX_U8BIT: u8 = std::u8::MAX;
    const MAX_16BIT: i16 = std::i16::MAX;
    const MIN_16BIT: i16 = std::i16::MIN;
    const MAX_U16BIT: u16 = std::u16::MAX;
    const MAX_32BIT: i32 = std::i32::MAX;
    const MIN_32BIT: i32 = std::i32::MIN;
    const MAX_U32BIT: u32 = std::u32::MAX;
    const MAX_64BIT: i64 = std::i64::MAX;
    const MIN_64BIT: i64 = std::i64::MIN;
    const MAX_U64BIT: u64 = std::u64::MAX;
    const MAX_128BIT: i128 = std::i128::MAX;
    const MIN_128BIT: i128 = std::i128::MIN;
    const MAX_U128BIT: u128 = std::u128::MAX;
    const MAX_ISIZE: isize = std::isize::MAX;
    const MIN_ISIZE: isize = std::isize::MIN;
    const MAX_USIZE: usize = std::usize::MAX;

    let decimal = 99_333;
    let hex = 0xfff;
    let octal = 0o777;
    let binary = 0b101_010;
    let byte = b'A'; // u8 only;

    let sum = 1 + 1;
    let boolean = false;
    // let boolean: bool = true;
    let character: char = '😻';

    // tuple and array
    // tuple
    let tup: (i32, u32, char) = (-32, 32, '😻');
    let (x, y, z) = tup;
    let tuple: (i32, u32, char) = (-32, 32, '😻');
    println!("tuple[2] = {}", tuple.2);


    // array
    let arr = [1, 2, 3, 4, 5];
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let arr_index = arr[3];
    
    // for-each iterate
    for element in arr {
        println!(" {} ", element);
    }

    let outer_variable = {
        let inner_variable = 16;
        inner_variable + inner_variable
    };
}

fn get_five() -> i32 {
    5
}

fn auto_increment(x: i32)->i32{
    x + 1
}