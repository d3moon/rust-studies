use std::io; // ou std::io::stdin()

/**
 * Loops em Rust
 * ========================
 *
*/

fn main() {
    // Loop infinito
    /*
     * Roda para sempre.
     * Pare com Ctrl+C ou break.
     * Raro sozinho → use com break.
     */
    // loop {
    //     println!("Loop infinito");
    // }

    let my_number = 3;
    let mut guess = false;

    println!("Adivinhe o número! (1-20)");

    // Ex 1
    // Loop while -> Enquanto a condição for falsa, faça isso
    // while guess != true {
    //     let mut number = String::new();
    //     std::io::stdin()
    //         .read_line(&mut number) // Lê a linha do input
    //         .expect("Falha ao ler a linha");

    //     let number = number
    //         .trim()
    //         .parse::<u32>()
    //         .expect("Por favor, digite um número!");

    //     /*
    //      * Lê input → converte para u8.
    //      * Se acertar → guess = true → sai do loop.
    //      * Senão → repete.
    //      */
    //     if number == my_number {
    //         println!("Você adivinhou o número!");
    //         guess = true;
    //     } else if number < my_number {
    //         println!("Muito baixo! Tente novamente.");
    //     } else {
    //         println!("Muito alto! Tente novamente.");
    //     }
    // }

    // let mut input = String::new();
    // io::stdin().read_line(&mut input).expect("Erro");

    //let num: u8 = input.trim().parse().expect("Não é número");

    // For Loop
    // 0..5 → exclusivo (0 até 4)
    // 0..=5 → inclusivo (0 até 5)
    for i in 0..5 {
        println!("{}", i) // 0,1,2,3,4
    }

    let vec = [20, 30, 40, 50];

    // Itera até o tamanho do vetor
    for i in 0..vec.len() {
        println!("{} e valor {}", i, vec[i]) // 0 e valor 20 ...
    }

    // Itera sobre um vetor temporário
    for i in [20, 30, 40, 50] {
        println!("{}", i);
    }

    


}
