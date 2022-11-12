// fn main() {
//     let mut name: &str = "Fabio";

//     name = "Henrique";

//     println!("Hello {}!", name);
// }

// fn main() {
//     let x: u64 = 32;
//     let f: f32 = 6.7;
//     let b: bool = true;
// }

// fn main() {
//     let number1 = 5;
//     let number2 = 5;

//     if number1 > number2 {
//         println!("{} > {}", number1, number2);
//     }
//     else {
//         println!("{} <= {}", number1, number2);
//     }
// }
use std::io;


fn convert_to_int(data_input: & String) -> i32 {
    let x = data_input.trim().parse::<i32>().unwrap();
    x

}


fn main() {

    let mut number1 = String::new();
    io::stdin().read_line(&mut number1).expect("Erro ao ler number1");

    let mut number2 = String::new();
    io::stdin().read_line(&mut number2).expect("Erro ao ler number1");

    if convert_to_int(&number1) > convert_to_int(&number2){
        println!("O numero {} eh maior que {}", number1, number2);
    }
    else {
        println!("O numero {} eh menor ou igual que {}", number1, number2);
    }

}