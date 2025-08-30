// Isso é um comentario

/*
Isso é um comentario
*/

fn main() {
    println!("Hello, world!");
    print!("Hello, world!"); // nao pula linha
    println!("{}", 10);
    println!("{}, {}", "Hello", "Tate!"); // Usa parametros para imprimir
    print!(
        "Lorem 
    ipsum dolor sit
     amet"
    );

    println!("\n\n This is going to be printed after one line "); // Pula linha
    println!("\t This will have some empty space at the beginning"); // Adiciona espaço tab
    println!("This is some text which will be overwritten \r this text will only aparecer"); // Retorna ao inicio da linha
    println!("This will print single quote \' and this double quotes \""); // Adiciona aspas
    println!("This will print a backslash \\"); // Adiciona barra invertida
    println!("\n doing {2} from {1} years i {0} it", "like", 20, "programming"); // Adiciona variáveis por parâmetro de posição
    println!("{language} is a system programming languagem which is cool to {activity}", language = "Rust", activity = "learn"); // Adiciona variáveis por parâmetro de nome
    println!("The summation of 5 and 10 is {}", 5 + 10); // Adiciona expressão
    /*
     * ln significa que após a impressão o cursor vai para a próxima linha
     * Um macro [!] é uma funcao que executa outra função
     */
}
