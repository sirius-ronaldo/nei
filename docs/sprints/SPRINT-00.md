# Sprint 00 — Fundação do projeto

## Objetivo

Criar uma base pública, documentada, compilável e preparada para implementação incremental do NEI.

## Entregáveis

- projeto Rust válido;
- `LICENSE` 0BSD;
- `README.md` bilíngue nas homenagens;
- `AGENTS.md`;
- especificação funcional/visual;
- mapa de comandos;
- arquitetura proposta;
- roadmap;
- especificações Sprints 01–09;
- `.gitignore`.

## Fora de escopo

- `crossterm`;
- raw mode;
- editor full-screen;
- edição real;
- templates de contribuição/comunidade.

## Critérios de aceite

```bash
cargo fmt --check
cargo check
cargo test
cargo run -- --version
```

Todos devem concluir com sucesso.
