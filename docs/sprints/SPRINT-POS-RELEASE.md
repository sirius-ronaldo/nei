Você está trabalhando no repositório do projeto **NEI — Norton Editor Inspired**, já concluído na versão **1.0.0**.

O objetivo desta tarefa é preparar o projeto para distribuição em sistemas Debian/Ubuntu por meio de um pacote `.deb` e revisar pequenos pontos de documentação relacionados à versão estável.

As tarefas de automação de release no GitHub foram separadas para
`docs/sprints/SPRINT-GITHUB-RELEASE.md`.

## Contexto do projeto

O NEI é um editor de texto full-screen para terminal Linux, escrito em Rust e inspirado no fluxo de trabalho do Norton Editor 1.x.

Características relevantes:

* linguagem: Rust;
* interface de terminal baseada em `crossterm`;
* executável: `nei`;
* versão atual: `1.0.0`;
* licença: `0BSD`;
* repositório:
  `https://github.com/sirius-ronaldo/nei`
* projeto já funcional e estabilizado;
* não alterar o comportamento do editor nesta tarefa;
* não introduzir dependências de runtime desnecessárias.

Antes de alterar qualquer arquivo, leia obrigatoriamente:

* `AGENTS.md`
* `Cargo.toml`
* `README.md`
* arquivos existentes em `.github/workflows/`, caso existam.

Respeite integralmente as regras descritas em `AGENTS.md`.

# Objetivo principal

Adicionar suporte oficial à geração de pacote Debian utilizando `cargo-deb`.

O resultado esperado deve permitir executar:

```bash
cargo deb
```

e gerar um pacote semelhante a:

```text
target/debian/nei_1.0.0-1_amd64.deb
```

O nome exato pode variar conforme o comportamento do `cargo-deb`.

# 1. Configurar `cargo-deb`

Adicionar ao `Cargo.toml` a configuração necessária em:

```toml
[package.metadata.deb]
```

Use os metadados já existentes no projeto sempre que possível.

A configuração deve contemplar, no mínimo:

```toml
[package.metadata.deb]
maintainer = "Ronaldo F Morais <sirius.ronaldo@gmail.com>"
section = "editors"
priority = "optional"
license-file = ["LICENSE", "0"]
```

Adicionar uma descrição estendida equivalente a:

```text
NEI — Norton Editor Inspired is a small, fast and retro full-screen
text editor for the Linux terminal, written in Rust and inspired by
the keyboard-oriented workflow of Norton Editor 1.x.
```

Os assets do pacote devem incluir:

```text
target/release/nei
    -> /usr/bin/nei

README.md
    -> /usr/share/doc/nei/README.md

LICENSE
    -> /usr/share/doc/nei/LICENSE
```

Com permissões apropriadas:

```text
/usr/bin/nei                     755
/usr/share/doc/nei/README.md     644
/usr/share/doc/nei/LICENSE       644
```

A intenção é que, após a instalação:

```bash
sudo apt install ./nei_1.0.0-1_amd64.deb
```

o comando:

```bash
nei
```

esteja disponível globalmente.

# 2. Não alterar a licença do projeto

O projeto utiliza:

```text
0BSD — BSD Zero Clause License
```

Não substituir por MIT, GPL, BSD-2-Clause ou qualquer outra licença.

O arquivo `LICENSE` deve continuar existindo e deve ser incluído no pacote Debian em:

```text
/usr/share/doc/nei/LICENSE
```

# 3. Preservar o comportamento atual

Esta tarefa é exclusivamente de:

* empacotamento;
* release;
* documentação relacionada à distribuição.

Não alterar:

* atalhos;
* comandos F3/F4;
* comportamento de blocos;
* comportamento de duas janelas;
* renderização;
* movimentação;
* edição;
* UTF-8;
* lógica de documentos;
* arquitetura funcional do editor.

Evite qualquer refatoração não necessária.

# 4. Revisar o README para a versão estável

O projeto já está na versão:

```text
1.0.0
```

Revise o `README.md` procurando textos herdados da fase inicial de desenvolvimento, especialmente expressões equivalentes a:

```text
Quando o editor estiver implementado...
```

ou:

```text
Na Sprint 00...
```

ou qualquer outro trecho que dê a impressão de que o editor ainda não existe.

Atualize apenas esses pontos para refletir que:

```text
NEI 1.0.0 é uma versão estável.
```

Não reescreva desnecessariamente o README.

Preserve:

* a identidade retro do projeto;
* a referência histórica ao Norton Editor;
* a homenagem a Peter Norton;
* as versões em Português e Inglês, caso estejam presentes;
* a declaração de independência do projeto;
* a licença 0BSD.

# 5. Documentar instalação via `.deb`

Adicionar ao README, em local apropriado, instruções simples de instalação do pacote Debian.

Exemplo:

```bash
sudo apt install ./nei_1.0.0-1_amd64.deb
```

Após a instalação:

```bash
nei
```

ou:

```bash
nei arquivo.txt
```

Também pode ser documentada a remoção:

```bash
sudo apt remove nei
```

Não assumir que o nome final do arquivo sempre terá exatamente `-1_amd64`; se necessário, explique que o nome pode variar conforme arquitetura e empacotamento.

# 6. Validação local do pacote

Se o ambiente permitir, execute:

```bash
cargo fmt --check
cargo check
cargo test
cargo build --release
```

Instale `cargo-deb` somente se o ambiente permitir:

```bash
cargo install cargo-deb
```

Depois execute:

```bash
cargo deb
```

Verifique o pacote gerado com:

```bash
dpkg-deb --info target/debian/*.deb
```

e:

```bash
dpkg-deb --contents target/debian/*.deb
```

Confirme que o conteúdo inclui algo equivalente a:

```text
/usr/bin/nei
/usr/share/doc/nei/README.md
/usr/share/doc/nei/LICENSE
```

Se estiver em um ambiente apropriado para teste de instalação, valide:

```bash
sudo apt install ./target/debian/*.deb
```

e:

```bash
which nei
nei --version
```

Não execute comandos privilegiados caso o ambiente não seja seguro ou não permita.

Nesse caso, apenas registre que o teste de instalação precisa ser feito manualmente.

# 7. Arquiteturas

Para esta primeira versão de distribuição Debian, priorizar:

```text
x86_64 / amd64
```

Não adicionar cross-compilation para ARM64 nesta tarefa, salvo se a infraestrutura do repositório já estiver preparada para isso.

A arquitetura ARM64 pode ser tratada futuramente.

# 8. Versão

A fonte de verdade da versão deve continuar sendo:

```toml
[package]
version = "1.0.0"
```

no `Cargo.toml`.

Não duplicar manualmente a versão em scripts sempre que for possível obtê-la do próprio projeto.

Evite estruturas que exijam atualizar a versão em vários arquivos para cada release.

# 9. Gitignore

Verifique se artefatos de build estão ignorados.

O repositório não deve versionar:

```text
target/
```

nem arquivos `.deb` gerados localmente.

Não remover regras existentes relacionadas a:

```text
docs/reference-images/
```

Essa pasta contém imagens locais de referência histórica e deve continuar fora do Git.

# 10. Critérios de aceite

A tarefa estará concluída quando:

* `Cargo.toml` estiver configurado para `cargo-deb`;
* `cargo build --release` continuar funcionando;
* testes existentes continuarem passando;
* `cargo deb` gerar corretamente o pacote;
* o pacote instalar o binário como `/usr/bin/nei`;
* README e LICENSE forem instalados em `/usr/share/doc/nei/`;
* o README refletir corretamente que a versão 1.0.0 está estável;
* o README possuir instruções de instalação via `.deb`;
* nenhuma funcionalidade do editor tiver sido alterada;
* a licença continuar sendo 0BSD;
* `target/` e pacotes gerados não forem adicionados ao Git;
* a geração local seja reproduzível a partir da versão definida no projeto.

# 11. Entrega final

Ao terminar, apresente um resumo objetivo contendo:

* arquivos modificados;
* arquivos criados;
* configuração Debian adicionada;
* comando para gerar o pacote localmente;
* caminho/nome do `.deb` efetivamente gerado;
* resultado de `cargo fmt --check`;
* resultado de `cargo check`;
* resultado de `cargo test`;
* resultado de `cargo deb`;
* resultado da inspeção por `dpkg-deb`;
* qualquer validação que não pôde ser realizada e o motivo.

Não faça alterações extras fora do escopo sem explicar previamente a necessidade.
