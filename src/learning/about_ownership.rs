/*
Variable in RAM:
    Stack And Heap
    Stack:
        First In Last Out, push() in or pop() out of stack;
        All data In Stack has a known and fixed size
        Accessing data stored in stack is more faster than in heap:
            Stack:
                Fixed size;
                Push() is always on stack top and pop() from stack bottom;
                Only available to single data type;
                Known and fixed pointers to data;

            Heap:
                Allocating before use(Huge time cost):
                    Find the available space and mark them as taken;
                    Return the pointer;
                Accessing through pointers;
    Ownership rules:
        First, let’s take a look at the ownership rules.
        Keep these rules in mind as we work through the examples that illustrate them:
            1. Each value in Rust has a variable that’s called its owner.
            2. There can only be one owner at a time.
            3. When the owner goes out of scope, the value will be dropped. (GC of rust-lang also, automatically call drop() the get outta scope)

 */



pub fn detail_of_ownership() {
    let str = "hello";
    println!(
        "Declarations like \'let str = \"{}\";\' \n\
    Cannot append by using String::push_str()",
        str
    );
    // Compiler knows the content at compile time, it compile is into a fix memory
    // String literal is fast by sacrificing mutability
    // str.push_str(",world"); // Exception by compile-time
    let mut str = String::from("hello");
    str.push_str(", world");
    println!("While using String::from() and mut the string should be mutable;");

    // Moving ownership:
    let var_a = 5;
    let var_b = var_a;
    // These copies the value but only cost a few of memory by using stack;
    println!(
        "In fact they got different addresses: var_a: {:p}; var_b: {:p};",
        &var_a, &var_b
    );

    let str_a = String::from("hello");
    let heap_addr_of_str_a = str_a.as_ptr();
    print!("Address of str_a = {:p}, ", &str_a);
    let str_b = str_a;
    let heap_addr_of_str_b = str_b.as_ptr();
    println!(
        "and address of str_b = {:p}, they shouldn't be the same;",
        &str_b
    );
    println!("Both 'str_a' and 'str_b' have the same address on the heap like 'str_a' = {:p}, and 'str_b' = {:p}", heap_addr_of_str_a, heap_addr_of_str_b);
    // These should be like now:
    //      str_a == str_b
    // But in fact nothing is owned by str_a because 'let str_b = str_a;' transfers ownership
    // Explain:
    //      Copying a complete string cost a lot of memory by operating on heap
    //      String has traits like: ptr->where to find them,length->how long are they and capacity->maximum of the amount
    //      And yet the 'let str_b = str_a;' copies those traits, but as you notice the pointer is pointing the same way
    //      When exiting the scope, it calls drop() for both str_a and str_b in order and drops the pointer at str_a so that it catches error by dropping again at str_b
    //      To avoid that rust-lang is considering str_a is no longer valid after 'let str_b = str_a;'
}

/*
About borrowing and referencing:
    Borrowing:
        using '&' to borrow the variable, mutable but not owning the variable;
        using '&' is like creating a reference to the variable that about to borrow;
        the original variable won't be dropped even the reference is out of its lifecycle;
        borrowing follows the same rule of the original variable, like mutability, etc;
        Examples:
 */
pub fn borrowing_and_referencing() {
    let str = String::from("hello");
    this_transfers_ownership(str); // --- value moved here
    // println!("{}", str); // value 'str' borrowed here after move
    let str = String::from(", ");
    this_wont_transfers_ownership_by_passing_reference(&str);
    let mut str = String::from("world");
    this_wont_transfers_ownership_by_passing_reference_and_is_mutable(&mut str);

    // mutable reference has restrictions
    // variables can be borrowed once due to the thread safety
    let mut str = String::from("borrow a string"); // error[E0499]: cannot borrow `str` as mutable more than once at a time
    let _mut_borrow_once = &mut str; // -------- first mutable borrow occurs here
    let _mut_borrow_twice = &mut str; // second mutable borrow occurs here
    // println!("try use both of them: mut_borrow_once = {}, mut_borrow_twice = {};", borrow_once, borrow_twice); //----------- first borrow later used here
    // both 'borrow_once' and 'borrow_twice' wasn't out of lifecycle before println!()

    // similar problem happens like:
    let mut str = String::from("borrow a string"); // error[E0502]: cannot borrow `str` as mutable because it is also borrowed as immutable
    let _borrow_once = &str; // ---- immutable borrow occurs here
    let _mut_borrow_twice = &mut str; // mutable borrow occurs here
    // println!("try use all of them: borrow_once = {}, borrow_twice = {};", borrow_once, mut_borrow_twice); // ----------- immutable borrow later used here
}

fn this_transfers_ownership(_str: String) {
    println!("Here 'str' from the original transfers its ownership to this scope, and drop() out of this scope;");
}

fn this_wont_transfers_ownership_by_passing_reference(_str: &String) {
    // str.push_str("try modify the value"); // str: &mut String
    println!("But it still can't be changed due to the original mutability;");
}

fn this_wont_transfers_ownership_by_passing_reference_and_is_mutable(str: &mut String) {
    str.push_str(" is different now;");
    println!("{}", str);
}

// this function's return type contains a borrowed value, but there is no value for it to be borrowed from
// fn explain_dangling_reference()->&String{
//     let str = String::from("dangle this"); // variable 'str' starts its lifecycle here
//     &str // return the reference here
// } // variable 'str' ends its lifecycle here
