# Sprint 05 — F4 BLOCK

## Objetivo

Implementar o sistema clássico de blocos por marcadores.

## Requisitos

Ao pressionar `F4`, exibir:

```text
F4 BLOCK: Set-marker   Copy   Move   Delete-block   Remove-marker   W   L   E   F
```

Implementar:

- `F4 S` — primeiro/segundo marcador;
- marcadores visíveis na margem;
- realce por intensidade do intervalo selecionado;
- `F4 R` — remover marcadores;
- `F4 C` — copiar bloco conforme semântica clássica documentada;
- `F4 M` — mover bloco;
- `F4 D` — apagar bloco;
- `F4 L` — marcar linha incluindo quebra de linha, conforme especificação confirmada;
- `F4 E` — marcar até o fim da linha sem quebra;
- `F4 F` — localizar marcador conforme comportamento confirmado disponível.

## TBD

`F4 W` é reservado à Sprint 06.

Se a semântica exata de `C`, `M`, `L` ou `F` ainda estiver insuficiente no momento da implementação, não inventar detalhes; solicitar atualização da especificação.
