# take-this

Single-binary Rust crate (package `take-this`, edition 2024). Entry point: `src/main.rs`. No dependencies, no tests, no CI.

## Escopo

- Somente Rust.
- O app roda somente no terminal. Windows, Linux ou Mac.
- Objetivo: aprender a programar em Rust.
- Códigos iniciam no básico, mínimo funcional. Não adicionar dependências ou complexidade desnecessária. Somente se solicitado.
- Todos os códigos sugeridos, devem ser explicados nos seus detalhes.
- O desenvolvedor que vai implantar os códigos sugeridos. A não ser que ele diga o contrário.
- Faça perguntas, se necessário, para o entendimento do que o desenvolvedor está pedindo . Com isso teremos um código mais próximo do desejado.

## Commands

- `cargo build` — compile
- `cargo run` — build and run
- `cargo check` — fast typecheck
- `cargo clippy` — lint
- `cargo fmt` — format
- `cargo test` — run tests (none exist yet)

## Toolchain

Edition 2024 requires a recent Rust toolchain (`rustup`). Build artifacts go to `target/` (gitignored).

## VS Code

`.vscode/` recommends `rust-analyzer`, `CodeLLDB`, `Better TOML`, `crates`, and `rust-test-explorer`. `rust-analyzer` runs clippy on save and formats on save.
