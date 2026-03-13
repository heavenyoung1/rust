use std::io;

fn main() {
    println!("Угадайте число!");

    println!("Пожалуйста введите свою догадку.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Не получилось прочитать строку");

    println!("Вы загадали {}", guess);
}
