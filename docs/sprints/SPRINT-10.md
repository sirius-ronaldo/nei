# Sprint 10 — Empacotamento Debian

## Objetivo

Disponibilizar um pacote `.deb` para instalação do NEI em sistemas Debian e
derivados.

## Requisitos

- gerar o binário de release com `cargo build --release`;
- gerar um pacote `.deb` com `dpkg-deb`;
- instalar o executável em `/usr/bin/nei`;
- incluir licença, README e página de manual;
- documentar a geração e a instalação do pacote.

## Fora de escopo

Repositório APT, assinatura do pacote e pacotes para arquiteturas diferentes
da arquitetura do ambiente de build.
