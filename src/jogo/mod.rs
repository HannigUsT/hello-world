extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub(crate) fn jogo_adv() {
    println!("Jogo da adivinhação, digite um número até acertar. range: 1 a 10.");
    let secret_number: u32 = rand::thread_rng().gen_range(1..=10);
    loop {
        println!("Por favor, coloque a sua tentativa:");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Falhou em ler a linha.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Muito pequeno."),
            Ordering::Greater => println!("Muito alto."),
            Ordering::Equal => {
                println!("Você venceu!!!");
                break;
            }
        }
    }
}
