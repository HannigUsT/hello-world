extern crate rand;

// use rand::Rng;
// use std::cmp::Ordering;
// use std::io;

pub(crate) fn jogo_adv() {
    //     println!("Jogo da adivinhação, digite um número até acertar. range: 1 a 10.");
    //     let secret_number: u32 = rand::thread_rng().gen_range(1..=10);
    //     loop {
    //         println!("Por favor, coloque a sua tentativa:");

    //         let mut guess: String = String::new();

    //         io::stdin()
    //             .read_line(&mut guess)
    //             .expect("Falhou em ler a linha.");

    //         let guess: u32 = match guess.trim().parse() {
    //             Ok(num) => num,
    //             Err(_) => continue,
    //         };

    //         match guess.cmp(&secret_number) {
    //             Ordering::Less => println!("Muito pequeno."),
    //             Ordering::Greater => println!("Muito alto."),
    //             Ordering::Equal => {
    //                 println!("Você venceu!!!");
    //                 break;
    //             }
    //         }
    //     }
}

pub(crate) fn ownership() {
    // const _STRING_MUTABLE_OWNERSHIP: &str = "O ịmportante é ser feliz";

    // let mut _string_mutable_ownership = String::from("O ịmportante ");

    // _string_mutable_ownership.push_str("é ser feliz");

    // println!("{}", _string_mutable_ownership);
    // println!("{}", _STRING_MUTABLE_OWNERSHIP);

    // -> Borrowed value

    // let _string_mutable_ownership = String::from("O ịmportante é ser feliz.");
    // let _string_mutable_ownership_copy: String = _string_mutable_ownership;

    // println!("{}", _string_mutable_ownership_copy);

    // -> Deep Copy heap data

    // let string_mutable_ownership = String::from("O ịmportante é ser feliz.");

    // let deep_copy_string_mutable_ownership_copy: String = string_mutable_ownership.clone();

    // println!("{}", string_mutable_ownership);

    // -> Ownership and Functions

    // let s: String = String::from("hello"); // s comes into scope

    // let a: String = takes_ownership(s);

    // println!("{}", a);

    // let x = 5;

    // makes_copy(x);

    // println!("{}", x);

    // -> Return Values and Scope

    // let s1 = String::from("hello");

    // let (s1, len) = calculate_length(s1);

    // println!("The length of '{}' is {}.", s1, len);
}

// fn takes_ownership(some_string: String) -> String {
//     println!("{}", some_string);
//     some_string
// }

// fn makes_copy(some_integer: i32) {
//     println!("{}", some_integer);
// }

// fn calculate_length(s1: String) -> (String, usize) {
//     let length = s1.len();
//     (s1, length)
// }

pub(crate) fn references_and_borrowing() {
    // let s1 = String::from("hello");

    // let len = calculate_length(&s1);

    // println!("The length of '{}' is {}.", s1, len);

    // -> Mutable Reference

    let mut s = String::from("hello");

    println!("{}", s);

    let a: () = change(&mut s);

    println!("{:?}", a);
}

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

fn change(some_string: &mut String) {
    println!("{}", some_string);
    some_string.push_str(", world");
}
