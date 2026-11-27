# take-that

Fuzzer de parâmetros HTTP de linha de comando (CLI), escrito em Rust.

## O que é

A aplicação lê uma **wordlist** (arquivo de texto com uma palavra por linha) e envia várias requisições HTTP para uma URL alvo, substituindo o marcador `TAKE` no corpo da requisição por cada palavra da lista.

É útil para testes de segurança (pentest), como por exemplo:

- Testar nomes de usuário em formulários de login
- Descobrir valores válidos de parâmetros em APIs
- Brute-force leve de campos de entrada

Exemplo de chamada:

```bash
./take-that \
  -w ~/dev/learning/rust/take-that/wordlists/username.txt \
  -X POST \
  -u "https://0a92007a0455ab5c83720ac4009f009e.web-security-academy.net/login" \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "username=TAKE&password=123"
```

Neste exemplo, para cada palavra da wordlist, o programa envia um `POST` substituindo `TAKE` pelo nome de usuário candidato:

```
username=admin&password=123
username=root&password=123
username=test&password=123
...
```

## Como é um CLI?

Sim. `take-that` é uma aplicação de **linha de comando** (CLI — *Command Line Interface*). Não tem interface gráfica; tudo é controlado por flags e argumentos no terminal.

## Flags disponíveis

| Flag | Descrição | Obrigatório | Padrão |
|------|-----------|-------------|--------|
| `-X` | Método HTTP (`GET`, `POST`, `PUT`, `PATCH`, `DELETE`) | Não | `GET` |
| `-u` | URL alvo da requisição | Sim | — |
| `-w` | Caminho para o arquivo de wordlist | Sim | — |
| `-H` | Cabeçalho HTTP (ex: `Content-Type: application/json`) | Não | — |
| `-d` | Corpo da requisição. Use `TAKE` como marcador de substituição | Não | — |

Se executado sem argumentos, exibe a ajuda básica:

```bash
./take-that
```

## Como compilar e executar

Requer Rust toolchain recente (edição 2024).

```bash
# Compilar
cargo build

# Compilar e executar direto
cargo run -- -w wordlists/username.txt -X POST -u "https://exemplo.com/login" -d "user=TAKE&pass=123"

# Executar o binário já compilado
./target/debug/take-that -w wordlists/usuario.txt -X GET -u "https://exemplo.com/api?id=TAKE"
```

## Como funciona (visão geral do código)

O programa está em `src/main.rs` e segue este fluxo:

1. **Parsing de argumentos**: Lê os argumentos passados no terminal (`std::env::args`) e preenche variáveis para método, URL, wordlist, cabeçalho e corpo da requisição.
2. **Leitura da wordlist**: A função `read_wordlist` abre o arquivo informado em `-w` e lê linha por linha, armazenando cada palavra em um `Vec<String>`.
3. **Substituição e requisição**: Para cada palavra da lista:
   - Substorna a string `TAKE` no corpo (`-d`) pela palavra corrente.
   - Envia a requisição HTTP com o método escolhido (`ureq`).
   - Imprime o status code da resposta ou um erro, caso falhe.

### Exemplo simplificado do loop principal

```rust
let words = read_wordlist(path);
for word in words {
    let new_data = data.replace("TAKE", &word);

    let response = match method {
        "POST" => ureq::post(url).send_string(&new_data),
        "GET"  => ureq::get(url).call(),
        // ... PUT, PATCH, DELETE
        _ => { /* método não suportado */ }
    };

    match response {
        Ok(res) => println!("Status: {}", res.status()),
        Err(e)  => println!("Request error: {}", e),
    }
}
```

## Escopo do projeto

Este é um projeto de **aprendizado de Rust**. Por isso:

- Código mínimo e funcional, sem complexidade desnecessária.
- Sem dependências externas além do necessário (conforme `AGENTS.md`).
- Executa somente no terminal (Windows, Linux ou Mac).
- O desenvolvedor implementa as sugestões; a IA apenas explica e sugere.

## Observações atuais

- O cabeçalho informado em `-H` é aceito pelo parser, mas ainda não é aplicado nas requisições HTTP.
- As requisições são enviadas sequencialmente (uma por vez), sem concorrência.
- O marcador de substituição é fixo: `TAKE` (maiúsculo).

## Próximos passos sugeridos

Se quiser evoluir a aplicação, alguns caminhos naturais são:

- Aplicar o cabeçalho `-H` nas requisições.
- Permitir múltiplos marcadores ou um marcador personalizável.
- Adicionar opção de mostrar o corpo da resposta, não apenas o status.
- Adicionar concorrência com threads para envios mais rápidos.
- Adicionar opção de salvar resultados em arquivo.

---

*Projeto criado com fins educacionais. Use apenas em sistemas que você possui ou tem autorização explícita para testar.*