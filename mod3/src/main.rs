/*
 * Desreferência
    - O operador de desreferência (*) é usado para acessar o valor ao qual um ponteiro está apontando.
    - Ele "desreferencia" o ponteiro, permitindo que você trabalhe com o valor real.
    - Isso é útil quando você tem um ponteiro e deseja acessar ou modificar o valor que ele aponta.
    - A desreferência basicamente é o ato de acessar o valor que está sendo apontado por um ponteiro.
*/

use std::vec;

fn main() {
    let mut x = 5;
    let y = &mut x; // y é um ponteiro mutável para x
    *y += 1; // Desreferencia y e incrementa o valor de x
    println!("O valor de x é: {}", x);

    // Exemplo 1: Tentando modificar um valor através de uma referência imutável
    let mut v = vec![1, 2, 3];
    let v1 = &v[0]; // v1 é uma referência imutável
    // v.push(4); // Erro: não é possível modificar v enquanto v1 está em uso
    println!("O primeiro elemento é: {}", v1);

    // Exemplo 2: Modificando um valor através de uma referência mutável
    let mut v2 = vec![4, 5, 6];
    let v2_ref = &mut v2[0]; // v2_ref é uma referência mutável
    *v2_ref += 1; // Desreferencia v2_ref e incrementa o valor
    println!("O primeiro elemento de v2 é: {}", v2[0]);

    // Exemplo 3: Usando clone para copiar valores em vez de referenciar
    let v3 = vec![7, 8, 9];
    let v3_clone = v3.clone(); // Cria uma cópia de v3
    println!("O primeiro elemento de v3_clone é: {}", v3_clone[0]);

    /* Explicação do Exemplo 3:
     * O clone cria uma cópia independente do vetor, permitindo que você trabalhe com ambos os vetores sem conflitos.
     * Aloca uma nova memória para o novo vetor, então as alterações em v3_clone não afetam v3.
     */

    // Exemplo 4: Tentando modificar um valor através de uma referência imutável (isso causará um erro de compilação)
    let mut x = 10;
    let y = &x; // referência imutável (&i32)
    *y = 20; // ERRO: não pode atribuir via referência imutável
}
