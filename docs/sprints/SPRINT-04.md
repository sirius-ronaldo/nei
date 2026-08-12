# Sprint 04 — F3 FILE

## Objetivo

Implementar o estado de comando prefixado `F3 FILE` e operações de arquivo confirmadas.

## Requisitos

Ao pressionar `F3`, exibir:

```text
F3 FILE: Exit-with-save   Quit   Save   eXchange-windows   New   Append   L   W
```

Implementar nesta Sprint, se a semântica estiver suficientemente especificada:

- `F3 E` — Save and exit;
- `F3 Q` — Quit and don't save;
- `F3 S` — Save and don't exit;
- `F3 N` — Edit a new file, respeitando prompts e proteção contra perda de alterações conforme levantamento disponível;
- `F3 A` — prompt de arquivo e append de seu conteúdo ao final do documento atual.

## TBD explícito

`F3 L` e `F3 W` não devem receber comportamento inventado. Implementá-los somente se a especificação tiver sido atualizada com detalhes confirmados. `F3 C` está fora do escopo do projeto.

`F3 X` pertence à Sprint 06, embora apareça na barra.
