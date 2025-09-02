Qual a diferença entre println! e print!?	println! adiciona quebra de linha ao final; print! não.
Como formatar saídas com placeholders posicionais?	Use println!("{2} from {1} years I {0} it", "like", 20, "programming").
Como usar parâmetros nomeados em println!?	Use println!("{language} é legal para {activity}", language = "Rust", activity = "learn").
Como imprimir o resultado de uma expressão?	Use println!("A soma de 5 e 10 é {}", 5 + 10).
Quais são as sequências de escape mais úteis?	\n nova linha; \t tab; \r retorno ao início; \" aspas; \\ barra.
O que \r faz em uma string?	Move o cursor para o início da linha e sobrescreve o que já foi impresso.
O que é uma macro em Rust?	Construto que gera código em compilação (ex.: println!).
Variáveis são mutáveis por padrão?	Não. Use mut para torná-las mutáveis.
Regras básicas para nomes de variáveis?	Começam com letra ou _, podem conter números, são case sensitive.
Quais são os inteiros assinados e não assinados?	Assinados: i8,i16,i32,i64,i128,isize. Não assinados: u8,u16,u32,u64,u128,usize.
Quais tipos de ponto flutuante existem?	f32 e f64.
Como declarar um char e usar Unicode?	let c: char = 'a'; let u: char = '\u{0063}';
Como ver limites máximos de tipos?	std::i8::MAX, std::u8::MAX, std::f32::MAX.
Como imprimir múltiplos valores juntos?	Use tupla com debug: println!("{:?}", (x, y, z, a)).
Qual a diferença entre Display e Debug?	Display usa {} para usuário; Debug usa {:?} para desenvolvedor.
Como declarar múltiplas variáveis e usar separadores em números?	let (a,b):(i32,f64)=(10,1_000_000.0);
Como imprimir em octal, hexadecimal e binário?	Use {:o}, {:X}, {:b}.
Como fazer casting entre tipos numéricos?	Use as: let s: f64 = (10_i32 as f64) + 20.7;
O que é shadowing?	Recriar a variável com mesmo nome: let s=5; let s=s+5;
Shadowing pode mudar o tipo?	Sim: let mut p=5; let p: f64=10.5;
Como funciona shadowing com escopo?	let interno esconde o externo no bloco; fora, o externo volta a valer.
Como formatar casas decimais fixas?	println!("{:.1}", r) imprime com 1 casa.
Como declarar uma constante?	const MAX_POINTS: u32 = 100_000; (sempre imutável).
Diferença entre &str e String?	&str é imutável (fatia); String é mutável e alocada no heap.
Como criar &str e String e concatenar?	let s: &str = "Hello"; let mut t=String::from(" Hello"); let u=format!("{}{}", s, t);
Métodos úteis de String?	is_empty, len, contains, capacity, trim, push, push_str, pop, to_string.
Como criar String vazia?	let empty = String::new();
Como converter número para String?	let s = 6.to_string();
O que são tuplas e como acessar valores?	Coleções heterogêneas; acesse por índice: t.0, t.1.
Como desestruturar uma tupla?	let (salary, amount) = my_info;
Como indexar tuplas aninhadas?	nested.0.1 acessa o segundo campo da interna.
Como declarar e alterar um array?	let mut a:[i32;2]=[0,5]; a[0]=10;
Como inicializar um array com o mesmo valor?	let a:[i32;2]=[0;2];
Como imprimir arrays?	println!("{:?}", a) usa Debug.
Como criar um slice de um array?	let r:&[i32]=&numbers[0..=2];
Quais funções úteis de array?	len, first, last, is_empty, get, std::mem::size_of_val.
O que get retorna e como tratar?	Option<&T>; use match/if let/unwrap() com cuidado.
Como criar e usar um Vec<T>?	let mut v:Vec<i32>=vec![1,2,3]; v[1]=10;
Como criar um vetor repetido e fazer slice?	let v=vec![0;10]; let s:&[i32]=&v[0..2];
Como buscar elemento com segurança em Vec?	v.get(2) retorna Option<&i32>.
Como modificar um Vec?	push, pop, remove.
Por que contains usa referência &T?	Evita cópias/movimentos; compara por referência: v.contains(&10).
Como comentar código em Rust?	// para linha; /* ... */ para bloco.