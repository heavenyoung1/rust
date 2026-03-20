use std::{cmp::Ordering, io};
// use std::cmp::Ordering;

fn main() {
    println!("Угадайте число!");

    let secret_number = rand::random_range(0..=100);

    loop {
        println!("Пожалуйста введите свою догадку.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Не получилось прочитать строку");

        let guess: u32 = guess.trim().parse()
            .expect("Пожалуйста введите число!");

        println!("Вы загадали {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => print!("Слишком маленькое число!"),
            Ordering::Greater => print!("Слишком большое число!"),
            Ordering::Equal => {
                print!("Вы выиграли");
                break;
            }
        }
    }
}
