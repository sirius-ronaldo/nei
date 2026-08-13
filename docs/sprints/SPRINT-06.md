# Sprint 06 — Duas janelas

## Objetivo

Reproduzir o fluxo clássico de duas janelas horizontais.

## Requisitos

- `F3 X` com somente uma janela disponibiliza a segunda área;
- se a segunda janela não possui arquivo, mostrar `Enter file name:` nela; nome inexistente abre documento vazio para gravação posterior;
- com duas janelas abertas, `F3 X` alterna a janela ativa;
- cada janela preserva documento, cursor, viewport, marcadores, modo de edição e Word Wrap;
- barra de status/separação deve refletir a janela ativa e ficar na posição apropriada ao layout;
- `F4 W` copia o bloco da outra janela para a posição atual da janela ativa;
- não implementar abas nem número arbitrário de janelas.

## Critérios de aceite

Alternar repetidamente entre as janelas não pode perder cursor, scroll, conteúdo ou marcadores.
