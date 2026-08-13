# Sprint 09 — Release 1.0

## Objetivo

Preparar a primeira versão estável pública do NEI.

## Requisitos

- revisar CLI `nei`, `--help`, `--version`;
- documentação de instalação e build;
- `cargo build --release`;
- avaliar target `x86_64-unknown-linux-musl` e documentar limitações reais encontradas;
- gerar checksums dos artefatos de release;
- revisar licença 0BSD, acknowledgments e disclaimer de independência;
- fechar itens críticos de robustez;
- preparar changelog/release notes.

## Fora de escopo

Estrutura formal para aceitar contribuições. Ela será preparada após a primeira versão estável.

## Artefatos e limitações observadas

- versão da release: `1.0.0`;
- artefato Linux principal: `target/release/nei`;
- checksum gerado no ambiente desta Sprint: `13f3d72f57908838f7d1b63d2da3a255d1a5076c5dd92cde3ee5c0f23a94c223`;
- o target `x86_64-unknown-linux-musl` foi tentado, mas não está instalado no toolchain
  disponível (`can't find crate for core`). Não há linker/target musl configurado no
  ambiente; nenhum suporte musl é afirmado pela release.
