use std::io;

/**
 * Stack implementation
 *
 * Pilha = LIFO → Last In, First Out
 * Só mexe no topo:
 * push → coloca no topo
 * pop → tira do topo
 * É exatamente como uma pilha de pratos: você só pega e coloca no de cima.
*/

fn main() {
    let mut stack = Vec::new(); // minha pilha

    // push -> adiciona um elemento na pilha
    stack.push(10);
    stack.push(20);
    stack.push(30);

    // pop -> remove um elemento na pilha se tiver algum elemento

    /*
     *  Criamos a condicao;
     * Criamos uma variavel do tipo Some com um valor dentro inteiro;
     * Atribuimos a resposta do pop na variável que pode existir ou n se tiver elemento;
     */
    if let Some(top) = stack.pop() {
        println!("Elemento removido: {}", top);
    }

    // peek -> mostra o ultimo elemento da pilha se tiver algum elemento
    if let Some(top) = stack.last() {
        println!("Elemento: {}", top);
    } else {
        println!("Pilha vazia");
    }

    // exec 1
    let mut pilha = Vec::new(); // minha pilha

    /*
     * Cria uma caixinha vazia → pede pro usuário digitar → joga tudo que ele digitou nessa caixinha e guarda no op
     */

    loop {
        println!("Escolha: 1 empilhar | 2 desempilhar | 3 ver topo | 4 ver tamanho | 5 sair");
        let mut op = String::new();
        io::stdin().read_line(&mut op).unwrap();
        match op.trim() {
            "1" => {
                print!("Digite o número pra empilhar: ");
                let mut num = String::new();
                io::stdin().read_line(&mut num).unwrap();
                let num: i32 = num.trim().parse().unwrap_or(0);
                pilha.push(num);
                println!("✅ Empilhado!");
            }
            "2" => match pilha.pop() {
                Some(x) => println!("❌ Removeu: {}", x),
                None => println!("🚫 Pilha vazia!"),
            },
            "3" => match pilha.last() {
                Some(x) => println!("👀 Topo é: {}", x),
                None => println!("🚫 Pilha vazia!"),
            },
            "4" => {
                println!("Existem {} elementos", pilha.len());
            }
            "5" => {
                println!("Tchau!");
                break;
            }
            _ => println!("Opção errada, tenta de novo"),
        }
    }
}
