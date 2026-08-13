# Sprint pós-release — Automação de release no GitHub

## Objetivo

Automatizar a validação, o empacotamento e a publicação dos artefatos de
distribuição do NEI por meio do GitHub Actions.

Esta Sprint foi separada de `docs/sprints/SPRINT-POS-RELEASE.md`. A Sprint
principal permanece dedicada ao pacote Debian, à documentação estável e à
validação local.

## Pré-condições

Antes de criar ou alterar workflows:

- ler `AGENTS.md`;
- verificar se já existem arquivos em `.github/workflows/`;
- confirmar que a versão em `Cargo.toml` continua sendo a fonte de verdade;
- confirmar quais permissões o workflow terá para criar ou atualizar Releases.

## Escopo

Para uma tag como:

```text
v1.0.0
```

o workflow deve preferencialmente:

1. fazer checkout do repositório;
2. instalar Rust estável;
3. executar `cargo fmt --check`, `cargo check`, `cargo test` e
   `cargo build --release`;
4. instalar `cargo-deb`;
5. executar `cargo deb`;
6. gerar um arquivo compactado contendo o binário Linux, por exemplo:
   `nei-1.0.0-x86_64-linux.tar.gz`;
7. anexar à GitHub Release os artefatos:
   - `nei_1.0.0-1_amd64.deb`;
   - `nei-1.0.0-x86_64-linux.tar.gz`.

Se já existir um workflow de release, adaptá-lo minimamente e não duplicar
responsabilidades.

## Restrições

- priorizar somente `x86_64` / `amd64` na primeira versão;
- não adicionar cross-compilation para ARM64 nesta Sprint;
- não duplicar manualmente a versão em scripts quando ela puder ser obtida do
  `Cargo.toml`;
- não alterar o comportamento do editor;
- não introduzir dependências de runtime;
- não publicar releases automaticamente sem confirmar as permissões e o
  gatilho de tags adotados pelo repositório.

## Decisões pendentes

- executar o workflow apenas em tags `v*` ou também em pushes e pull requests;
- publicar automaticamente uma GitHub Release ou somente armazenar artefatos;
- gerar checksums dos artefatos;
- definir permissões mínimas para `GITHUB_TOKEN`;
- decidir se a publicação será de uma release existente ou de uma nova release.

## Critérios de aceite

- o workflow é acionado pelo evento definido para releases;
- as validações Rust são executadas antes da publicação;
- `cargo deb` gera um pacote Debian válido;
- o tarball contém o binário Linux esperado;
- a versão e os nomes dos artefatos são derivados do projeto ou da tag;
- os artefatos são anexados à release conforme a política escolhida;
- falhas de validação impedem a publicação;
- nenhuma funcionalidade do editor é alterada.
