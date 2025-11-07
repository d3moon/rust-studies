/**
 * Conditional Functionality
 * ========================
 *
*/

fn main() {
    //    if condition {
    //    // code to execute if condition is true
    //    } else {
    //        // code to execute if condition is false
    //    }

    let some_number = 7;
    if some_number < 5 {
        println!("The number is less than 5.");
    } else {
        println!("The number is 5 or greater.");
    }

    let flag_1 = true;
    let flag_2: bool = false;

    if flag_1 || flag_2 {
        println!("At least one flag is true.");
    }

    if flag_1 && (flag_2 || some_number > 10) {
        println!("Complex condition met.");
    } else if some_number == 7 {
        println!("The number is exactly 7.");
    } else {
        println!("No conditions met.");
    }

    //println!("Enter a number:");

    // let mut input = String::new();
    // std::io::stdin()
    //     .read_line(&mut input)
    //     .expect("Failed to read line"); // Read user input e retorna um erro se falhar

    // let user_number: i32 = input.trim().parse().expect("Please enter a valid number"); // Converte a entrada para um número inteiro

    // Verifica se o número é par ou ímpar com base no input do usuário
    // if user_number % 2 == 0 {
    //     //qualquer número par dividido por 2 tem resto 0
    //     println!("The number {} is even.", user_number);
    // } else {
    //     println!("The number {} is odd.", user_number);
    // }

    // Condição como expressão na atribuição de variável.
    let condition: bool = { 4 == 4 };

    println!("{:?}", condition);

    
}
