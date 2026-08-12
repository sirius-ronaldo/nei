# Mapa de comandos — referência NEI

Esta tabela consolida comandos observados na ajuda e nos testes do Norton Editor 1.3 usados como referência para o NEI.

## Cursor control

| Tecla | Função | Estado |
|---|---|---|
| `←` | Move cursor left | CONFIRMED pela ajuda |
| `→` | Move cursor right | CONFIRMED pela ajuda |
| `↑` | Move cursor up | CONFIRMED pela ajuda |
| `↓` | Move cursor down | CONFIRMED pela ajuda |
| `Ctrl+←` | Move cursor word left | CONFIRMED pela ajuda |
| `Ctrl+→` | Move cursor word right | CONFIRMED pela ajuda |
| `Home` | Move to line beginning | CONFIRMED pela ajuda |
| `End` | Move to line end | CONFIRMED pela ajuda |
| `PgUp` | Move up a page | CONFIRMED pela ajuda |
| `PgDn` | Move down a page | CONFIRMED pela ajuda |
| `Ctrl+Home` | Move to file beginning | CONFIRMED pela ajuda |
| `Ctrl+End` | Move to file end | CONFIRMED pela ajuda |

## Delete commands

| Tecla | Função | Estado |
|---|---|---|
| `Backspace` | Delete character left | CONFIRMED pela ajuda |
| `Del` | Delete character right | CONFIRMED pela ajuda |
| `Ctrl+W` | Delete word left | CONFIRMED pela ajuda |
| `Alt+W` | Delete word right | CONFIRMED pela ajuda |
| `Ctrl+L` | Delete to line beginning | CONFIRMED pela ajuda |
| `Alt+L` | Delete to line end | CONFIRMED pela ajuda |
| `Alt+K` | Kill all characters on line | CONFIRMED pela ajuda |
| `F4 D` | Delete a block | CONFIRMED |
| `Ctrl+U` | Undelete text | CONFIRMED: somente a última deleção |

## F3 — FILE

| Sequência | Função | Estado |
|---|---|---|
| `F3 E` | Save and exit / Exit-with-save | CONFIRMED |
| `F3 S` | Save and don't exit | CONFIRMED |
| `F3 Q` | Quit and don't save | CONFIRMED |
| `F3 N` | Edit a new file | CONFIRMED pela ajuda; detalhes finos TBD |
| `F3 X` | Exchange windows | CONFIRMED |
| `F3 W` | Write text to cursor | Nome CONFIRMED; semântica detalhada TBD |
| `F3 L` | Load more of the file | Nome CONFIRMED; semântica detalhada TBD |
| `F3 A` | Append a file | CONFIRMED: solicita arquivo e anexa conteúdo ao final |
| `F3 C` | Close the output file | Nome CONFIRMED; semântica detalhada TBD |

Barra contextual observada:

```text
F3 FILE: Exit-with-save   Quit   Save   eXchange-windows   New   Append   L   W   C
```

## F4 — BLOCK

| Sequência | Função | Estado |
|---|---|---|
| `F4 S` | Set a block marker | CONFIRMED |
| `F4 R` | Remove the markers | CONFIRMED |
| `F4 D` | Delete a block | CONFIRMED pela ajuda |
| `F4 C` | Copy a block para a posição atual; mantém marcadores e posiciona cursor no fim da cópia | CONFIRMED |
| `F4 W` | Copy block from window | CONFIRMED |
| `F4 M` | Move a block para a posição atual; remove marcadores e posiciona cursor no fim | CONFIRMED |
| `F4 L` | Marca a linha inteira, incluindo quebra; na última linha marca até o fim | CONFIRMED |
| `F4 E` | Mark to line end (no CR) | CONFIRMED pela ajuda |
| `F4 F` | Localiza o marcador posterior mais próximo do cursor | CONFIRMED |

Barra contextual observada:

```text
F4 BLOCK: Set-marker   Copy   Move   Delete-block   Remove-marker   W   L   E   F
```

## Regras de implementação

1. Não associar funções por intuição moderna. Ex.: `F4 S` **não** é Save.
2. Comandos marcados como TBD não devem ter comportamento inventado.
3. O mapa clássico é parte da identidade funcional do NEI.

### Semântica confirmada dos blocos

Os marcadores delimitam um intervalo semiaberto: a seleção começa na posição
do marcador inicial e termina imediatamente antes da posição do marcador final.
A ordem dos marcadores pode ser invertida. Assim, um marcador final no início da
linha seguinte inclui a quebra de linha anterior; um marcador no fim da linha não
inclui quebra adicional.

`F4 C` copia para o cursor, mantém os marcadores e leva o cursor ao fim da cópia.
`F4 M` move para o cursor, remove os marcadores e leva o cursor ao fim. `F4 D`
remove o bloco e os marcadores. Uma segunda marcação na mesma posição é ignorada.
`F4 L` marca do início da linha ao início da próxima, ou até o fim na última linha.
`F4 F` procura o marcador posterior mais próximo da posição atual.
