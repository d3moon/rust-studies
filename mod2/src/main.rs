/**
 * Funções são blocos de código que realizam uma tarefa específica.
 * Declaradas com a palavra-chave `fn`.
 * Podem receber parâmetros e retornar valores.
 * Utilizam snake_case para nomes.
 * Podem ter parâmetros opcionais com valores padrão.
 * Parâmetros são imutáveis por padrão.
 * No Rust não importa a ordem física da funcao no arquivo, porque ele resolve todas as declarações na hora da compilação.
 * Function call é o ato de invocar ou executar uma função previamente definida.
 * Function definition é a declaração ou criação de uma função, onde se especifica seu nome, parâmetros e corpo.
 *
 */

fn basic_fn() {
    println!("This is a basic function.");
}

// Recebe dois parâmetros inteiros e imprime a soma deles.
fn function_with_inputs(x: i32, y: i32) {
    println!("The sum is: {}", x + y);
}

// Função que retorna a soma de dois inteiros.
fn function_with_return(x: i32, y: i32) -> i32 {
    x + y // Retorna a soma de x e y -> inteiro
}

fn function_with_multiple_returns(x: i32, y: i32) -> (i32, i32) {
    (x + y, x * y) // Retorna uma tupla com a soma e o produto de x e y
}

fn main() {
    basic_fn();
    function_with_inputs(5, 10); // Chamada da função com argumentos 5 e 10
    let x = 3;
    let y = 7;
    function_with_inputs(x, y); // Chamada da função com variáveis x e y

    // Chamada da função que retorna um valor
    let result = function_with_return(2, 2);
    println!("The returned sum is: {}", result);

    // Chamada da função que retorna múltiplos valores
    let (sum, product) = function_with_multiple_returns(4, 5);
    println!("Sum: {}, Product: {}", sum, product);

    /*
     * Podemos  usar blocos de código para retornar valores em uma variável, como em funções.
       * Útil para criar escopos limitados e organizar o código.
     * Podemos usar o macro format! para formatar strings.
     * O macro format ele formata a string como o println!, mas ao invés de imprimir na tela, ele retorna a string formatada.
     * Ele roda em tempo de execução e não em tempo de compilação.
     * Ele é um atalho para criar strings expandindo internamente com o uso do std::fmt.
     */

    let full_name = {
        let first_name = "John";
        let last_name = "Doe";
        format!("{} {}", first_name, last_name) // Retorna a string formatada
    };

    println!("Full name: {}", full_name);

    /*
     * Entrada do usuario
     * Podemos usar a biblioteca padrão do Rust para ler a entrada do usuário.
     * Usamos o módulo std::io para isso.
     * A função read_line lê uma linha da entrada padrão (teclado) e armazena em uma variável.
     * A função read_line retorna um Result, que pode ser Ok ou Err.
     * Usamos o método expect para tratar erros, exibindo uma mensagem personalizada.
     * A entrada lida é uma string, então precisamos converter para o tipo desejado.
     * Usamos o método trim para remover espaços em branco e quebras de linha.
     * Usamos o método parse para converter a string para o tipo desejado.
     * O método parse retorna um Result, então usamos expect para tratar erros.
     * Note que o parse pode falhar se a entrada não for um número válido.  
     */

    let mut input = String::new(); // Cria uma string mutável para armazenar a entrada
    println!("Please enter a number: ");
    std::io::stdin() // Obtém a entrada padrão (teclado)
        .read_line(&mut input) // Lê uma linha e armazena na variável usando referência mutável que acessa o conteúdo da variável na memória
        .expect("Failed to read line"); // Trata erros

    let number: i32 = input.trim().parse().expect("Please type a number!"); // Converte a string para i32
    println!("You entered: {}", number);
}
