#[allow(unused)]
pub fn show_data_types() {
    // primitive types
    const MAX_8BIT: i8 = i8::MAX;
    const MIN_8BIT: i8 = i8::MIN;
    const MAX_U8BIT: u8 = u8::MAX;
    const MAX_16BIT: i16 = i16::MAX;
    const MIN_16BIT: i16 = i16::MIN;
    const MAX_U16BIT: u16 = u16::MAX;
    const MAX_32BIT: i32 = i32::MAX;
    const MIN_32BIT: i32 = i32::MIN;
    const MAX_U32BIT: u32 = u32::MAX;
    const MAX_64BIT: i64 = i64::MAX;
    const MIN_64BIT: i64 = i64::MIN;
    const MAX_U64BIT: u64 = u64::MAX;
    const MAX_128BIT: i128 = i128::MAX;
    const MIN_128BIT: i128 = i128::MIN;
    const MAX_U128BIT: u128 = u128::MAX;
    const MAX_ISIZE: isize = isize::MAX;
    const MIN_ISIZE: isize = isize::MIN;
    const MAX_USIZE: usize = usize::MAX;

    let _decimal = 99_333;
    let _hex = 0xfff;
    let _octal = 0o777;
    let _binary = 0b101_010;
    let _byte = b'A'; // u8 only;

    let _sum = 1 + 1;
    let _boolean = false;
    // let boolean: bool = true;
    let _character: char = '😻';

    // tuple and array
    // tuple
    let tup: (i32, u32, char) = (-32, 32, '😻');
    let (x, y, z) = tup;
    println!("x = {}, y = {}, z = {}", x, y, z);
    let tuple: (i32, u32, char) = (-32, 32, '😻');
    println!("tuple[2] = {}", tuple.2);

    // array
    let _arr = [1, 2, 3, 4, 5];
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let _arr_index = arr[3];

    // for-each iterate
    for element in arr.iter() {
        println!(" {} ", element);
    }

    let outer_variable = {
        // 'inner_variable' only works inside the block
        // and it won't affect outer variable named same as 'inner_variable'
        let inner_variable = 16;
        // consider these as a return
        inner_variable + inner_variable
    };

    // About String:
    // Mostly used:
    //      String::from()
}

fn get_five() -> i32 {
    5
}

fn auto_increment(x: i32) -> i32 {
    x + 1
}
