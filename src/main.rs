#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;


fn main() { 
    println!("What is your name?"); 
    let mut name = String::new(); 
    let greeting = "Nice to meet you"; 
    io::stdin().read_line(&mut name) 
        .expect("Didn't Receive Input"); 
    println!("Hello {}! {}", name.trim_end(), greeting); 
} 
 


fn main(){
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.141592;
    let age = "47";
    let mut age: u32 = age.trim().parse()
        .expect("Age wasn't assigned a number");
    age = age + 1;
    println!("I'm {} and I want ${}", age, ONE_MIL);
}



fn main(){
     println!("Max u32 : {}", u32::MAX);
     println!("Max u64: {}", u64::MAX);
     println!("Max f32 : {}", f32::MAX);
     println!("Max usize : {}", usize::MAX);
     println!("Max u128 : {}", u128::MAX);
     println!("Max u64 : {}", u64::MAX);
}

fn main() {
    let is_true = true;
    let my_grade = 'A';
}


fn main(){
    let num_3 = 5;
    let num_6 = 6;
    println!("5 + 6 = {}", num_3 + num_6);
    println!("5 - 6 = {}", num_3 - num_6);
    println!("5 * 6 = {}", num_3 * num_6);
    println!("5 / 6 = {}", num_3 / num_6);
    println!("5 % 6 = {}", num_3 % num_6);
}



fn main(){
    let age = 23;
    if (age >= 1) && (age <= 18){
        println!("Too young");
    } else if (age == 21) || (age == 50) {
        println!("You have earned privileges!");
    } else if age >= 65 {
        println!("Still kinda important");
    } else{
        println!("Not that important");
    }
}


fn main() {
    let mut my_age = 17;
    let can_vote = if my_age >= 18{
        true
    } else {
        false
    };
    println!("Can Vote : {}", can_vote);
}

fn main() {
    let age2 = 898989998;
    match age2{
        1..=18 => println!("You are young"),
        21 | 50 => println!("You are a middle aged fella"),
        65..=i32::MAX => println!("You are elderly!"),
        _=> println!("Woww, you sure, that is your age?"),
    };
}

fn main() {
    let my_age = 18;
    let voting_age = 18;
    match my_age.cmp(&voting_age){
        Ordering::Less => println!("Cannot Vote!"),
        Ordering::Greater => println!("Can Vote!"),
        Ordering::Equal => println!("Just got the right to vote"),
    };
}

fn main(){
    let arr_2 = [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25];
    println!("1st : {}", arr_2[0]);
    println!("Length of list : {}", arr_2.len());
    let mut loop_idx = 0;
    loop{
        if arr_2[loop_idx] % 2 == 0 {
            loop_idx += 1;
            continue;
        }
        if arr_2[loop_idx] == 23{
            break;
        }
        println!("Value : {}", arr_2[loop_idx]);
        loop_idx += 1;
    }


}

fn main(){
    let arr_2 = [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25];
    let mut loop_idx = 0;
    while loop_idx < arr_2.len(){
        println!("Arr : {}", arr_2[loop_idx]);
        loop_idx += 5;
    }
}

fn main(){
    let arr_2 = [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25];
    let mut loop_idx = 0;
    for val in arr_2.iter(){
         println!("Val : {}", val);
    }
}


fn main(){
    let my_tuple: (u8, String, f64) = (47, "Derek".to_string(), 50_000.00);

    println!("Name : {}", my_tuple.2);
    let(v1, v2, v3) = my_tuple;
    println!("Age : {}", v1);
}



fn main(){
    let mut st1 = String::new();
    st1.push('A');
    st1.push_str(" word");
    for word in st1.split_whitespace(){
        println!("{}", word);
    }
    let st2 = st1.replace("A", "Another");
    println!("{}", st2);
}