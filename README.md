# MiniLang Compiler

Um compilador para a linguagem de programação fictícia MiniLang, desenvolvido em Rust. Este projeto visa demonstrar habilidades em engenharia de software e computação, explorando a poderosa linguagem de programação Rust.

## Sobre

MiniLang é uma linguagem de programação minimalista projetada para fins educacionais. O compilador MiniLang, escrito em Rust, realiza a análise léxica e sintática do código-fonte, transformando-o em uma representação intermediária conhecida como Árvore de Sintaxe Abstrata (AST). A AST pode então ser utilizada para diversas finalidades, como otimização de código, geração de código de máquina, entre outros.

## Funcionalidades

- **Análise Léxica:** Conversão do código-fonte em tokens.
- **Análise Sintática:** Construção de uma Árvore de Sintaxe Abstrata (AST) a partir dos tokens.
- **Suporte para Operações Matemáticas Básicas:** Adição, subtração, multiplicação e divisão.
- **Expressões com Parênteses:** Agrupamento de expressões para controle de precedência das operações.
- **Identificadores:** Uso de nomes personalizados para variáveis ou funções.
- **Operadores de Comparação:** Igual (==), diferente (!=), menor que (<), maior que (>), menor ou igual (<=), maior ou igual (>=).
- **Operadores Lógicos:** E (&&) e OU (||).

## Como Executar

Para compilar e executar o compilador MiniLang, siga estas etapas:

1. Garanta que o Rust esteja instalado em seu sistema. Se não estiver, você pode instalar seguindo as instruções em [rust-lang.org](https://www.rust-lang.org/tools/install).
2. Clone este repositório para sua máquina local usando o comando:
git clone (https://github.com/guilhermesob/MiniLang-Compiler/tree/main)
3. Navegue até o diretório do projeto clonado.
4. Compile o projeto com o comando:
cargo build
5. Execute o binário gerado com:
./target/debug/Compilers
Substitua `nome_do_seu_programa` pelo nome real do binário gerado.

## Contribuições

Contribuições são bem-vindas e encorajadas. Se você deseja contribuir, por favor, siga estas diretrizes:

1. Faça fork do repositório.
2. Crie uma branch para sua nova feature ou correção de bug.
3. Implemente sua alteração.
4. Verifique se todas as verificações de CI estão passando.
5. Abra um Pull Request.

## Licença

Este projeto é distribuído sob a licença MIT. Veja o arquivo LICENSE para mais detalhes.