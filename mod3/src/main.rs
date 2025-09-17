fn main() {
    /*
     * Nesse exemplo, veremos como alterar o valor de uma variável na pilha (stack) dentro de uma função.
     * Passos:
     *  - Criar uma variável mutável na pilha. Para permitir a alteração, a variável deve ser mutável.
     *  - Criar uma função que recebe uma referência mutável para essa variável. Isso permite que a função acesse e modifique o valor original. Novo dono.
     *  - Alterar o valor da variável dentro da função.
     */

    // Variável na pilha (stack)-> Significa que o valor é armazenado diretamente na memória stack.
    let mut x = 5; // Mutável
    println!("main x: {}", x);

    // Função que altera o valor da variável na pilha
    fn stack_fn(number: &mut i32) {
        // Recebe uma referência mutável, que é um ponteiro para o valor na stack
        *number = 42; // Desreferencia o ponteiro e altera o valor na stack
        println!("stack_fn x: {}", number); // Imprime o valor alterado
    }

    // Chama a função passando uma referência mutável para x
    stack_fn(&mut x);
    // O Ownership de x permanece com main, mas o valor foi alterado pela função stack_fn, nova atribuição
    println!("main x after stack_fn: {}", x);

    /*
     * Nesse exemplo, veremos como alterar o valor de uma variável sem modificar o ownership.
     * Passos:
     * - Criar uma variável imutável na pilha.
     * - Criar uma função que recebe uma cópia do valor dessa variável.
     * - Alterar o valor dentro da função, mas isso não afetará a variável original
     */
    let y = 5; // Imutável

    fn stack_fn2(mut x: i32) {
        if x == 5 {
            x = 42; // Altera só localmente
        }
        println!("stack_fn2 x: {}", x); // Imprime o valor local
    }

    stack_fn2(y); // Passa uma cópia do valor de y
    println!("main y: {}", y); // y continua 5


    // O Ownership desse exemplo é movido para o another_vec
    // É útil quando queremos transferir a posse de um valor para outra variável
    let some_vec = vec![1, 2, 3]; // some_vec é dono do vetor na heap
    let another_vec = some_vec; // Ownership de some_vec é movido para another_vec, isso é possível pois tipos compostos movem ownership por padrão, enquanto tipos primitivos copiam o valor (por serem valores pequenos)
    let yet_another_vec = &another_vec; // Referência para another_vec


    // Podemos usar referências mutáveis para alterar valores na heap
    // É útil quando queremos modificar o valor sem transferir o ownership
    let mut vec_1 = vec![1, 2, 3]; // vec_1 é dono do vetor na heap
    let mut ref1 = &mut vec_1; // ref1 é uma referência mutável para vec_1 (o valor de ref1 é um ponteiro para o valor na heap que pode ser alterado (o valor de vec_1 pode ser alterado através de ref1))
    ref1.push(4); // Altera o valor na heap através da referência mutável

    let large_string = String::from("Hello, world!"); // large_string é dono da string na heap
    let large_string_2: String = String::from("Hello, Rust!"); // large_string_2 é dono da string na heap

    // É útil quando queremos criar uma coleção de referências para valores que já possuem um dono, sem assumir a posse desses valores
    let huge_data: Vec<&String> = vec![&large_string, &large_string_2]; // huge_data é dono do vetor que contém as strings na heap
}
