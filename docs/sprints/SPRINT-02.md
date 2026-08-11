# Sprint 02 — Documento, arquivo e navegação

## Objetivo

Abrir e exibir arquivos UTF-8 com navegação clássica e barra de status.

## Requisitos

- `nei arquivo.txt` abre diretamente o arquivo;
- fluxo de nome de arquivo da Sprint 01 abre o documento solicitado;
- criar `Document`, `Position`, `Viewport` e `EditorWindow` conforme necessário;
- implementar `←`, `→`, `↑`, `↓`, `Ctrl+←`, `Ctrl+→`, `Home`, `End`, `PgUp`, `PgDn`, `Ctrl+Home`, `Ctrl+End`;
- status com `Line`, `Col`, nome/caminho, `Insert`, `WW=Off`;
- cursor em bloco;
- scrolling vertical e horizontal suficiente para manter o cursor visível.

## Fora de escopo

- alterações no conteúdo;
- salvar;
- blocos;
- segunda janela.

## Critérios de aceite

- navegação não ultrapassa limites válidos;
- arquivos vazios funcionam;
- UTF-8 comum não causa panic;
- testes de cursor/documento sem terminal.
