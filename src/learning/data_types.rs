pub fn show_data_types() {
    /*
    About the keyword 'mut'
    w/o the 'mut', once the variable is assigned, it's unchangeable, aka immutable
    w/ the 'mut', the variable is changeable
    for the safety of concurrency, rust is designed to set the variables immutable as default;
     */
    let not_mutable = "This Variable Cannot Be Updated";
    let mut mutable = "This Variable Is Changeable Due To The 'mut' keyword";
    println!("{}", not_mutable);
    println!("{}", mutable);
    mutable = "It Can Be Muted Like This";
    println!("{}", mutable);
    /*
    but you can still assign a variable twice, makes 'let' like a 'new' in java,
     */
    let not_mutable = "But Still Have Way To Change The 'immutable''s Address On The RAM";
    println!("{}", not_mutable);

    /*
    but if it's 'const' assigned, its storage address is immutable
     */
    const NO_WAY_TO_MUTE_THIS: &'static str = "There is No way to mute this variable";
    println!("{}", NO_WAY_TO_MUTE_THIS);
    /*
    shadowing
    and it's not allowed on constraints;
     */
    let not_mutable_int:i8 = 10;
    println!("This Come From A Not Mutable Int: {}", not_mutable_int);
    let not_mutable_int:i16 = (not_mutable_int + 10) as i16;
    println!("Yet It's 'Muted' By Using Shadowing: {}", not_mutable_int);
}