# Roadmap — NEI

## Sprint 00 — Fundação

Repositório Rust compilável, licença, documentação de produto, arquitetura e especificações das Sprints.

## Sprint 01 — Terminal e identidade visual

`crossterm`, raw mode, alternate screen, restauração segura, cursor em bloco, resize e tela inicial do NEI.

## Sprint 02 — Documento, arquivo e navegação

Abrir arquivo, `Document`, viewport, cursor, navegação clássica e barra de status.

## Sprint 03 — Edição básica

Inserção, overwrite, Enter, Backspace, Delete, comandos de deleção e `Ctrl+U` de uma única restauração.

## Sprint 04 — F3 FILE

Máquina de prefix command e operações de arquivo cujo comportamento esteja confirmado.

## Sprint 05 — F4 BLOCK

Marcadores, realce, copy/move/delete, mark line/end e navegação de marcadores conforme especificado.

## Sprint 06 — Duas janelas

`F3 X`, estado independente e `F4 W` para cópia de bloco entre janelas.

## Sprint 07 — Busca, substituição e operações de arquivo

Busca (`Ctrl+F`/`F5 F`), substituição (`Ctrl+H`/`F5 R`), `F3 L`, `F3 W`,
`F4 E` e Word Wrap (`F5 W`). `F3 C` permanece fora do escopo.

## Sprint 08 — Robustez e fidelidade

Resize, UTF-8, arquivos extremos, erros de I/O, terminais distintos e comparação funcional/visual.

## Sprint 09 — Release 1.0

CLI final, documentação, build release, avaliação de musl, checksums e preparação de publicação.

## Após 1.0

Abertura formal para contribuições, documentação de contribuição, templates, política comunitária e discussão de extensões modernas opcionais sem quebrar o modo clássico.
