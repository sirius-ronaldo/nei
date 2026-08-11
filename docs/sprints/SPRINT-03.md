# Sprint 03 — Edição básica e deleção

## Objetivo

Transformar o visualizador em editor de texto em memória.

## Requisitos

- inserir caracteres;
- Enter cria nova linha;
- Backspace e Delete;
- alternar Insert/Overwrite conforme tecla/comportamento de terminal apropriado;
- `Ctrl+W`: delete word left;
- `Alt+W`: delete word right;
- `Ctrl+L`: delete to line beginning;
- `Alt+L`: delete to line end;
- `Alt+K`: kill all characters on line;
- `Ctrl+U`: restaurar somente a última deleção elegível;
- modificar flag de documento alterado.

## Regra crítica

Não implementar uma interface de undo/redo moderno. `Ctrl+U` expõe somente a restauração da última deleção.

## Fora de escopo

- persistência F3;
- blocos;
- segunda janela.
