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
| `F4 C` | Copy a block | CONFIRMED pela ajuda |
| `F4 W` | Copy block from window | CONFIRMED |
| `F4 M` | Move a block | CONFIRMED pela ajuda |
| `F4 L` | Mark line (w/CR) | CONFIRMED pela ajuda; detalhes de CR deverão ser testados |
| `F4 E` | Mark to line end (no CR) | CONFIRMED pela ajuda |
| `F4 F` | Find a block marker | CONFIRMED pela ajuda; navegação exata TBD |

Barra contextual observada:

```text
F4 BLOCK: Set-marker   Copy   Move   Delete-block   Remove-marker   W   L   E   F
```

## Regras de implementação

1. Não associar funções por intuição moderna. Ex.: `F4 S` **não** é Save.
2. Comandos marcados como TBD não devem ter comportamento inventado.
3. O mapa clássico é parte da identidade funcional do NEI.
