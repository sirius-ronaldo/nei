# NEI 1.0.0

Primeira versão estável do NEI — Norton Editor Inspired.

## Destaques

- edição de texto UTF-8 no terminal Linux;
- busca e substituição com `Ctrl+F`/`F5 F` e `Ctrl+H`/`F5 R`;
- operações de arquivo `F3 L` e `F3 W`;
- marcadores e operações de bloco `F4`;
- duas janelas com `F3 X`;
- Word Wrap físico com `F5 W`;
- tela de ajuda pela tecla `F1`;
- restauração do terminal ao sair;
- CLI com `--help` e `--version`.

## Build

```bash
cargo build --release
./target/release/nei --help
```

O binário de release é `target/release/nei`. Para gerar o checksum:

```bash
sha256sum target/release/nei
```

Checksum gerado nesta preparação para `x86_64-unknown-linux-gnu`:

```text
13f3d72f57908838f7d1b63d2da3a255d1a5076c5dd92cde3ee5c0f23a94c223  target/release/nei
```

O build `x86_64-unknown-linux-musl` não foi produzido nesta preparação porque o target
musl não está instalado no toolchain disponível. Ele também exige um linker musl.
