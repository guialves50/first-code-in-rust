use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Jogo da Adivinhação\n");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Faça um chute\n");

        println!("{}", secret_number);

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = guess.trim().parse().expect("Faça seu chute");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("O número é maior\n"),
            Ordering::Greater => println!("O número é menor\n"),
            Ordering::Equal => {
                println!("\nVocê ganhou!");
                break;
            }
        }
    }
}
