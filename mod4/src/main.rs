    /**
     * Match Statement
     * ========================
     * match é como um switch superpoderoso que obrigatoriamente cobre todos os casos (exaustivo)
     * e sempre executa algo.
     *
     * Padrão,Significado
        1,valor exato
        2 | 3,OU lógico (2 ou 3)
        4..=100,intervalo inclusivo (4 até 100)
        _,qualquer outro valor  (coringa)

    * Propriedades OBRIGATÓRIAS do match
        * Exaustivo → deve cobrir todos os casos possíveis.
        * Sempre executa um braço → nunca "cai fora".
        Erro se faltar _ e não cobrir tudo
        → Compilador reclama: "non-exhaustive patterns"

        Erro se braço inalcançável
        Ex: colocar 2 depois de 2 | 3 → "unreachable pattern"



    */

    fn main() {
        let valor = 3;

        // Padrão básico
        match valor {
            1 => println!("Um"),
            2 => println!("Dois"),
            3 => println!("Três"),
            _ => println!("Outro valor"), // Curinga para qualquer outro valor
        }

        // Padrão lógico
        match valor {
            1 | 3 | 5 | 7 | 9 => println!("Ímpar"),
            2 | 4 | 6 | 8 | 10 => println!("Par"),
            _ => println!("Fora do intervalo"),
        }

        // Intervalo inclusivo
        match valor {
            1..=5 => println!("Entre 1 e 5"),
            6..=10 => println!("Entre 6 e 10"),
            _ => println!("Fora do intervalo"),
        }

        // Atribuindo match a uma variável

        //Regras do let = match:
            
        // Última expressão de cada braço = valor retornado
        // Todos os braços devem retornar o mesmo tipo
        // Se tiver várias linhas → use {} e sem ; na última
        // Erro: braços com tipos diferentes
        let mensagem: &str = match valor {
            1 => "Um",
            2 => "Dois",
            3 => "Três",
            _ => "Outro valor",
        };
        println!("Mensagem: {}", mensagem);

        // Quando PODE omitir _?
        let is_active = true;
        match is_active {
            true => println!("Ativo"),
            false => println!("Inativo"),
        };
        

    }
