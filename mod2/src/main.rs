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

    // let some_string: &str = "Hello, world!"; // &str é uma fatia de string, uma string imutável
    // let mut another_string: String = String::from(" Hello, world! "); // String é uma string mutável
    // another_string.push_str(" How are you?"); // Adiciona uma string ao final da outra

    // another_string.push('!'); // Adiciona um caractere ao final da string
    // another_string.pop(); // Remove o último caractere da string
    // println!("{}", some_string);
    // println!("{}", another_string);

    // // Algumas funcoes uteis de strings

    // println!(
    //     "Basics function on Strings, is empty: {}",
    //     another_string.is_empty()
    // ); // Verifica se a string está vazia
    // println!(
    //     "Basics function on Strings, length: {}",
    //     another_string.len()
    // ); // Retorna o tamanho da string
    // println!(
    //     "Basics function on Strings, contains 'world': {}",
    //     another_string.contains("world")
    // ); // Verifica se a string contém uma substring
    // println!(
    //     "Basics function on Strings, bytes: {}",
    //     another_string.capacity()
    // ); // Retorna a quantidade de bytes alocados na memória

    // println!("{}", another_string.trim()); // Remove espaços em branco do início e do fim da string

    // let number = 6;
    // let num_str: String = number.to_string(); // Converte um número para string

    // let empty_string = String::new(); // Cria uma string vazia

    // let s_1: String = String::from("Hello");
    // let s_2: String = String::from("World");
    // let s_3: String = format!("{} {}", s_1, s_2); // Concatena duas strings
    // println!("{}", s_3);

    // println!("Is the string really empty: {}", empty_string.is_empty());
    // println!("Is the number really a string: {}", number.to_string() == "6");

    /*
     * Tuplas são coleções de valores de diferentes tipos
     *  - Acessadas por índice
     *  - Desestruturadas em variáveis
     *  - Podem conter diferentes tipos de dados
     *  - Tem tamanhos variados
     */
    // let my_info: (&str, f64) = ("Salary", 5_000.0);
    // println!("My {} is {}", my_info.0, my_info.1);
    // println!("Another way of printing the whole tuple is {:?}", my_info);

    // let (salary, amount) = my_info;
    // println!("My {} is {}", salary, amount);

    // // Indexacao por tuplas aninhadas
    // let nested_tuple: ((&str, f64), &str) = ((salary, amount), "USD");
    // println!(
    //     "My {} is {} {}",
    //     nested_tuple.0.0, nested_tuple.0.1, nested_tuple.1
    // );

    // /*
    //  * Matriz é uma coleção de elementos de mesmo tipo
    //  *  - Deve conter:
    //  *    - Um tipo de dado
    //  *    - Um tamanho fixo
    //  *    - Em []
    //  */
    // let mut array: [i32; 2] = [0, 5];

    // // Display = como mostrar um valor de forma bonita para o usuário.
    // // Ex: uma String pode ser mostrada direto como texto.

    // // Debug = como mostrar um valor de forma útil para o programador (pra depuração).
    // // Ex: vec![1, 2, 3] vira [1, 2, 3] com {:?}.

    // // Mudamos os valores do array pelo índice
    // array[0] = 10;
    // array[1] = 20;

    // println!("Array Value: {}", array[0]);
    // println!("Array: {:?}", array);

    // // Inicializando um array com os mesmos valores
    // let another_array: [i32; 2] = [0; 2];
    // println!("Another Array: {:?}", another_array);

    // // Imprimindo  um array de char
    // let char_array: [char; 5] = ['a', 'p', 'p', 'l', 'e'];
    // println!("Char Array: {:?}", char_array);

    // // Atribuindo uma referencia de um array

    // /*
    //  * Uma referencia é um ponteiro para um valor em outra parte da memória.
    //  * Permite acessar e manipular dados sem fazer cópias.
    //  */
    // let numbers: [i32; 4] = [1, 2, 3, 4];
    // // let numbers_ref: &[i32] = &numbers[0..2]; // Criando uma referência do indice 0 ao 1
    // let numbers_ref: &[i32] = &numbers[0..=2]; // Criando uma referência do indice 0 ao 2
    // println!("Numbers Ref: {:?}", numbers_ref);

    // /*
    //  * Algumas functions de arrays
    // */
    // println!("Length: {}", numbers.len()); // Retorna o tamanho do array
    // println!("First Element: {:?}", numbers.first()); // Retorna o primeiro elemento do array na forma de um Option. Options são tipos que podem ou não ter um valor.
    // println!("First Element: {:?}", numbers.first().unwrap()); // Retorna o primeiro elemento do array na forma de um unwrap. Unwrap é uma forma de acessar o valor dentro de um Option, assumindo que ele sempre estará presente.
    // println!("Last Element: {:?}", numbers.last()); // Retorna o último elemento do array na forma de um Option.
    // println!("Last Element: {:?}", numbers.last().unwrap()); // Retorna o último elemento do array na forma de um unwrap.
    // println!("Is Empty: {}", numbers.is_empty()); // Verifica se o array está vazio
    // println!("Size: {}", std::mem::size_of_val(&numbers)); // Retorna o tamanho em bytes do array na memória. É 16 porque contém 4 elementos de 4 bytes cada.
    // // std::mem é um módulo do rust que fornece funções para trabalhar com a memória.

    // // Podemos verificar a existência de um elemento em um array
    // let check: Option<&i32> = numbers.get(100);
    // println!("Get Element: {:?}", check); // Retorna o elemento do array na posição 100 na forma de um Option. Como não existe, retorna None.

    /*
     * Vetores são coleções de elementos de mesmo tipo, mas com tamanho dinâmico.
     * Declarados com a macro `vec![]`.
     * A tipagem envolve a utilização de `Vec<T>`, onde `T` é o tipo dos elementos.
     */
    let mut vetor: Vec<i32> = vec![1, 2, 3, 4];
    vetor[1] = 10;
    println!("Vetor: {}", vetor[1]);

    // Inicializando um vetor com os mesmos valores
    let vetor_2: Vec<i32> = vec![0; 10];
    println!("Vetor 2: {:?}", vetor_2);

    // Criando uma referência do indice 0 ao 1
    let subset: &[i32] = &vetor[0..2];
    println!("Subset: {:?}", subset);

    // Pesquisando a existencia de um elemento de forma segura usando a referencia em vez de pegar ou copiar
    let option: Option<&i32> = vetor.get(2);
    println!("Get Element: {:?}", option);

    vetor.push(12); // Adiciona um elemento ao final do vetor
    println!("Vetor: {:?}", vetor);

    vetor.pop(); // Remove o último elemento do vetor
    println!("Vetor: {:?}", vetor);

    vetor.remove(1); // Remove o elemento na posição 1
    println!("Remove Element from Index: {:?}", vetor);

    println!("Contains Element: {:?}", vetor.contains(&10)); // Preciso passar o & pq é uma medida do Rust para evitar cópias desnecessárias

    /*
     * Vec guarda os valores dentro dele. Se você passasse o valor direto, o Rust teria que mover o valor para dentro da função, ou copiar.
     * Ao exigir &T, a função não precisa copiar ou mover nada, só compara a referência com os elementos do vetor.
     * Isso funciona mesmo para tipos grandes, que não são Copy, sem gastar memória extra.
     */
}
