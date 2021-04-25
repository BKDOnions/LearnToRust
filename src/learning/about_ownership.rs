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

use std::ops::Index;

pub fn detail_of_ownership() {
    let str = "hello";
    println!("Declarations like \'let str = \"{}\";\' \n\
    Cannot append by using String::push_str()", str);
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
    println!("In fact they got different addresses: var_a: {:p}; var_b: {:p};", &var_a, &var_b);

    let str_a = String::from("hello");
    let str_b = str_a;
    // these should be like now:
    //      str_a == str_b
    // but in fact nothing is owned by str_a because 'let str_b = str_a;' transfers ownership
    // Explain:
    //      Copying a complete string cost a lot of memory by operating on heap
    //      String has traits like: ptr->where to find them,length->how long are they and capacity->maximum of the amount
    //      And yet the 'let str_b = str_a;' copies those traits, but as you notice the pointer is pointing the same way
    //      When exiting the scope, it calls drop() for both str_a and str_b in order and drops the pointer at str_a so that it catches error by dropping again at str_b
    //      To avoid that rust-lang is considering str_a is no longer valid after 'let str_b = str_a;'

}
