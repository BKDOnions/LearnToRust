use crate::learning::about_enum::Coin::Quarter;
use crate::learning::about_enum::IPAddressTypes::{IPV4, IPV6};
use crate::learning::about_enum::USState::Nevada;

pub fn show_about_enums() {
    println!("Here is the enums with value: ");
    let local_loopback = IPV4(String::from("127.0.0.1"));
    let remote_loopback = IPV6(String::from("::1"));
    println!("Self calling: ");
    local_loopback.call();
    remote_loopback.call();
    println!("Data dump: \n {:#?} \n {:#?}", local_loopback, remote_loopback);
    println!("Enums w/ inner enums and operations");
    let nevada_quarter = Quarter(Nevada);
    println!("Here is the value of The Coin: {} \nData dump: \n{:#?}", show_matches(&nevada_quarter), nevada_quarter);
    println!("Going through 'Options': ");
    println!("Value of show_options(10): {}, show_options(None): is_none?{}", show_options(Some(10)).unwrap(), show_options(None).is_none());
    println!("Going wildcard: ");
    using_wildcard(1);
    using_wildcard(2);
    using_wildcard(3);
}

#[derive(Debug)]
pub enum IPAddressTypes {
    IPV4(String),
    IPV6(String),
}

impl IPAddressTypes {
    pub fn call(&self) {
        match self {
            IPAddressTypes::IPV4(val) => {
                println!("IP value is: {}", val)
            }
            IPAddressTypes::IPV6(val) => {
                println!("Ip value is: {}", val)
            }
        }
    }
}

#[derive(Debug)]
pub enum USState {
    Washington,
    California,
    Nevada,
    Etc,
}

#[derive(Debug)]
pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(USState),
}

pub fn show_matches(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            match state {
                USState::California => {
                    println!("it's CA Quarter");
                    25
                }
                USState::Nevada => {
                    println!("it's NV Quarter");
                    30
                }
                USState::Washington => {
                    println!("it's WA Quarter");
                    35
                }
                USState::Etc => {
                    println!("it's something else's Quarter");
                    40
                }
            }
        }
    }
}

pub fn show_options(var: Option<i32>) -> Option<i32> {
    match var {
        None => None,
        Some(i) => Some(i + 1),
    }
}

pub fn using_wildcard(var: i32) {
    match var {
        1 => println!("1:one"),
        3 => println!("3:three"),
        5 => println!("5:five"),
        _ => println!("{}:else", var),
    }
}