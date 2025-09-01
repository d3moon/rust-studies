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

    // Por padrão as variaveis sao imutaveis
    // Utilize mut para torná-las mutáveis

    /*
     * Regras para criacao de variaveis
     * - Só letras, numeros e undercores
     * Deve comecar com uma letra ou underscore
     * Case sensitive
     */

    /*
     * Tipos de dados
     *  - Escalares
     *         - Inteiros
     *              - Signed
     *                  - i8, i16, i32, i64, i128, isize
     *              - Unsigned
     *                  - u8, u16, u32, u64, u128, usize
     *        - Float
     *              - f32, f64
     *        - Boolean
     *              - true, false
     *        - Char
     *              - 'a', 'b', 'c'
    */

    println!("The maximum number in i8 is {}", std::i8::MAX);
    println!("The maximum number in u8 is {}", std::u8::MAX);
    println!("The maximum number in f32 is {}", std::f32::MAX);
    
    let c1: char = 'a';
    let c2: char = 'b';
    let c3: char = '\u{0063}'; // Unicode

    println!("{:?}", (c1, c2, c3));

    let mut x: i8 = 127;
    println!("{}", x);
    x = 126;
    let y: f64 = 5.0;
    let z: i128 = 10000000000000000;
    let a = 18 != 18;
    println!("{}", y);
    println!("{}", z);
    println!("{}", a);
    println!("{:?}", (x, y, z, a) ); // Imprime todos os valores em uma tupla. O ponto de interrogacao indica que queremos imprimir o valor composto por vários tipos
}
