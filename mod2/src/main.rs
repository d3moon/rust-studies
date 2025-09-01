// Isso é um comentario

/*
Isso é um comentario
*/

fn main() {
    //     println!("Hello, world!");
    //     print!("Hello, world!"); // nao pula linha
    //     println!("{}", 10);
    //     println!("{}, {}", "Hello", "Tate!"); // Usa parametros para imprimir
    //     print!(
    //         "Lorem
    // ipsum dolor sit
    //  amet"
    //     );

    //     println!("\n\n This is going to be printed after one line "); // Pula linha
    //     println!("\t This will have some empty space at the beginning"); // Adiciona espaço tab
    //     println!("This is some text which will be overwritten \r this text will only aparecer"); // Retorna ao inicio da linha
    //     println!("This will print single quote \' and this double quotes \""); // Adiciona aspas
    //     println!("This will print a backslash \\"); // Adiciona barra invertida
    //     println!("\n doing {2} from {1} years i {0} it", "like", 20, "programming"); // Adiciona variáveis por parâmetro de posição
    //     println!("{language} is a system programming languagem which is cool to {activity}", language = "Rust", activity = "learn"); // Adiciona variáveis por parâmetro de nome
    //     println!("The summation of 5 and 10 is {}", 5 + 10); // Adiciona expressão
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

    // println!("The maximum number in i8 is {}", std::i8::MAX);
    // println!("The maximum number in u8 is {}", std::u8::MAX);
    // println!("The maximum number in f32 is {}", std::f32::MAX);

    // let c1: char = 'a';
    // let c2: char = 'b';
    // let c3: char = '\u{0063}'; // Unicode

    // println!("{:?}", (c1, c2, c3));

    // let mut x: i8 = 127;
    // println!("{}", x);
    // x = 126;
    // let y: f64 = 5.0;
    // let z: i128 = 10000000000000000;
    // let a = 18 != 18;
    // println!("{}", y);
    // println!("{}", z);
    // println!("{}", a);
    // println!("{:?}", (x, y, z, a) ); // Imprime todos os valores em uma tupla. O ponto de interrogacao indica que queremos imprimir o valor composto por vários tipos

    // Forma de declarar multiplas variaveis
    // underscore melhora a legibilidade dos pontos flutuantes e o rust também entende com ponto flutuante
    // let (first_number, second_number): (i32, f64) = (10, 1_000_000.0);

    // println!("{}", second_number);
    // println!(
    //     "The value of the variable in octal is {:o}, and in hex is {:X} and in binary is {:b}",
    //     first_number, first_number, first_number
    // );

    // let n1 = 10;
    // let n2 = 20.7;
    // let n3: f64 = n1 as f64 + n2; //Mudamos temporariamente o tipo do n1 com as
    // println!("{}", n3);

    // // Shadowing é criar uma variavel nova com o mesmo nome, sobrescrevendo o valor anterior
    // let s:i32 = 5; // fica escondida
    // let s:i32 = s + 5;
    // println!("{}", s);

    // let mut p = 5;
    // let p: f64 = 10.5; // mudei o tipo tbm
    // println!("{}", p);
    // // p = 60; // erro variavel anterior subst ituida por versao imutavel

    // let r: f64 = 5.0;
    // {
    //     let r:i32 = 10; // escopo interno
    //     println!("inside the scope {}", r);
    // }

    // println!("outside scope {:.1}", r); // {:.N} → imprime com N casas decimais fixas, sempre mostrando o .0 se precisar.

    // // Constantes sao variaveis imutaveis
    // // declaracao em maiusculo e underscore
    // const MAX_POINTS: u32 = 100_000;
    // println!("The maximum points are {}", MAX_POINTS);

    /*
     * Strings
     *  - São sequências de caracteres
     *  - Podem ser mutáveis ou imutáveis
     *  - Podem ser criadas com aspas duplas ou simples
     *  - &str -> String imutável
     *  - String -> String mutável
     */

    let some_string: &str = "Hello, world!"; // &str é uma fatia de string, uma string imutável
    let mut another_string: String = String::from(" Hello, world! "); // String é uma string mutável
    another_string.push_str(" How are you?"); // Adiciona uma string ao final da outra

    another_string.push('!'); // Adiciona um caractere ao final da string
    another_string.pop(); // Remove o último caractere da string
    println!("{}", some_string);
    println!("{}", another_string);

    // Algumas funcoes uteis de strings

    println!(
        "Basics function on Strings, is empty: {}",
        another_string.is_empty()
    ); // Verifica se a string está vazia
    println!(
        "Basics function on Strings, length: {}",
        another_string.len()
    ); // Retorna o tamanho da string
    println!(
        "Basics function on Strings, contains 'world': {}",
        another_string.contains("world")
    ); // Verifica se a string contém uma substring
    println!(
        "Basics function on Strings, bytes: {}",
        another_string.capacity()
    ); // Retorna a quantidade de bytes alocados na memória

    println!("{}", another_string.trim()); // Remove espaços em branco do início e do fim da string

    let number = 6;
    let num_str: String = number.to_string(); // Converte um número para string

    let empty_string = String::new(); // Cria uma string vazia

    let s_1: String = String::from("Hello");
    let s_2: String = String::from("World");
    let s_3: String = format!("{} {}", s_1, s_2); // Concatena duas strings
    println!("{}", s_3);

    println!("Is the string really empty: {}", empty_string.is_empty());
    println!("Is the number really a string: {}", number.to_string() == "6");   
}
