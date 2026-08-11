# Sprint 01 — Terminal e identidade visual

## Objetivo

Construir a camada básica de terminal full-screen e a primeira reprodução visual do fluxo de abertura do NEI.

## Requisitos

- adicionar `crossterm`;
- entrar em raw mode e alternate screen;
- esconder/mostrar cursor quando necessário e solicitar cursor em bloco para edição;
- restaurar terminal de forma confiável ao sair ou em erros controlados;
- tratar resize;
- `nei` sem arquivo apresenta `Enter file name:` e identidade visual própria do NEI;
- `Esc` deve permitir cancelar o fluxo inicial e sair de maneira limpa (decisão NEI para segurança operacional; documentar como extensão, caso não haja evidência do original);
- nenhuma edição de documento ainda.

## Fora de escopo

- modelo completo de documento;
- salvar arquivo;
- F3/F4;
- duas janelas.

## Critérios de aceite

- terminal volta ao estado normal após saída;
- resize não produz panic;
- `cargo fmt --check`, `cargo check` e `cargo test` passam;
- teste manual em terminal Linux real.
