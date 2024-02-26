extern crate rand;

// use rand::Rng;
// use std::cmp::Ordering;
// use std::io;

// pub(crate) fn jogo_adv() {
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
// }

// pub(crate) fn ownership() {
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
// }

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

// pub(crate) fn references_and_borrowing() {
// let s1 = String::from("hello");

// let len = calculate_length(&s1);

// println!("The length of '{}' is {}.", s1, len);

// -> Mutable Reference

// let mut s = String::from("hello");

// println!("{}", s);

// change(&mut s);

// println!("{:?}", s);

// let mut s = String::from("hello");

// let r1 = &s; // no problem
// let r2 = &s; // no problem
// println!("{} and {}", r1, r2);
// // variables r1 and r2 will not be used after this point

// let r3 = &mut s; // no problem
// println!("{}", r3);

// -> Dangling References

// let reference_to_s = dangle();
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
//     println!("{}", some_string);
// }

// fn dangle() -> String {
//     let s = String::from("hello");
//     s
// }

pub(crate) fn the_slice_type() {
    // let mut s = String::from("hello world");

    // let _word = first_word(&s);

    // println!("{}", _word);

    // s.clear();

    // -> String slices

    // let s = String::from("hello world");

    // let len = s.len();

    // let a = &s[..5];

    // println!("{} ", a);

    let mut s: String = String::from("hello world");

    let _ref_word = first_word(&s);

    let _copy_ref_word = _ref_word.clone();

    _ref_word.clear(); // error!

    println!("the first word is: {}", _copy_ref_word);
}

// Não faça isso aqui

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

// Faça isso aqui

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
