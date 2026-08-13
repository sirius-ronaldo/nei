# NEI — Norton Editor Inspired

> **Retro outside. Modern inside.**

**NEI (Norton Editor Inspired)** é um editor de textos open source para terminal Linux, escrito em Rust e inspirado na simplicidade, velocidade e fluxo de trabalho orientado por teclado do **Norton Editor 1.x**, especialmente a versão 1.3 estudada durante a concepção deste projeto.

O objetivo não é criar uma IDE moderna com aparência retrô. A meta é preservar a experiência de um editor full-screen simples, rápido e essencialmente controlado pelo teclado, implementado sobre uma base moderna, segura e adequada ao Linux atual.

> **Project status:** stable 1.0.0 release. The repository is public; contribution guidelines will be published after the first stable release cycle.

## Objetivos

- preservar o fluxo de trabalho clássico inspirado no Norton Editor 1.3;
- manter uma interface de terminal minimalista e retrô;
- priorizar teclado, velocidade de inicialização e baixa complexidade;
- executar nativamente em Linux;
- produzir um executável independente sempre que viável;
- trabalhar internamente com UTF-8;
- preservar conceitos clássicos como marcadores de bloco e duas janelas de edição;
- evitar dependências e abstrações de UI desnecessárias;
- evoluir de forma incremental, com cada Sprint gerando um estado compilável e testável.

## Interface clássica

O NEI seguirá o modelo de comandos por tecla prefixadora observado no Norton Editor 1.3.

```text
F3  FILE commands
F4  BLOCK commands
```

Ao pressionar `F3` ou `F4`, a barra de status é temporariamente substituída por uma barra contextual, e o editor aguarda a segunda tecla do comando.

Exemplos confirmados:

```text
F3 S    Save and don't exit
F3 E    Save and exit
F3 Q    Quit and don't save
F3 X    Exchange windows

F4 S    Set a block marker
F4 C    Copy a block
F4 M    Move a block
F4 D    Delete a block
F4 R    Remove the markers
```

A tabela completa levantada até o momento está em [`docs/COMMANDS.md`](docs/COMMANDS.md).

## Blocos

O NEI preservará o conceito clássico de **block markers**. `F4 S` coloca um marcador na posição atual. Após mover o cursor, um segundo `F4 S` coloca o segundo marcador e delimita o bloco.

Os marcadores são apresentados na margem e o conteúdo do bloco recebe realce por intensidade, em vez de uma seleção gráfica moderna por fundo colorido.

## Duas janelas

O fluxo clássico de duas janelas horizontais também faz parte da especificação. `F3 X` abre ou alterna entre as janelas. Cada janela mantém estado próprio de documento, cursor, viewport e blocos. `F4 W` permite copiar um bloco da outra janela para a janela ativa.

## Retro por fora, moderno por dentro

A interface e os atalhos devem preservar a experiência clássica; a implementação, porém, será moderna:

- Rust;
- Linux;
- `crossterm` como camada de terminal a partir da Sprint 01;
- UTF-8;
- gerenciamento seguro de memória;
- restauração confiável do terminal após saída ou erro;
- testes automatizados para regras de documento e edição.

Frameworks de widgets como Ratatui não fazem parte da arquitetura inicial. O NEI precisa de controle direto das células do terminal para preservar sua identidade visual.

## Instalação e build

Com Rust instalado, compile a versão de release:

```bash
cargo build --release
```

O executável estará em `target/release/nei`. Ele pode ser executado diretamente ou
copiado para um diretório presente no `PATH`:

```bash
./target/release/nei arquivo.txt
```

Para instalar o executável pelo Cargo:

```bash
cargo install --path .
```

Consulte `nei --help` para as opções da linha de comando e pressione `F1` dentro do
editor para consultar os comandos de teclado.

## Uso

Quando o editor estiver implementado:

```bash
nei arquivo.txt
```

abrirá diretamente o arquivo. A execução sem argumento:

```bash
nei
```

exibirá o fluxo clássico de solicitação:

```text
Enter file name:
```

Na Sprint 00, o binário é apenas um esqueleto compilável; a interface full-screen começa na Sprint 01.

## Desenvolvimento incremental

O projeto é dividido em Sprints. Cada Sprint deve terminar com código compilável e um resultado verificável.

Consulte [`docs/ROADMAP.md`](docs/ROADMAP.md) e [`docs/sprints/`](docs/sprints/).

## Acknowledgment — English

NEI is also a tribute to the simplicity and effectiveness of software tools from an earlier era of personal computing.

Special acknowledgment and sincere thanks go to **Peter Norton**, whose name became closely associated with a generation of remarkably practical software tools for personal computers.

Norton Editor is an important inspiration for this project. Its compact interface, keyboard-oriented workflow, and focus on staying out of the programmer's way are reminders that useful software does not need to be complicated.

NEI is an independent open-source project. It is not affiliated with, sponsored by, or endorsed by Peter Norton, Gen Digital, Symantec, or any current or former owner of Norton-related trademarks or software.

## Agradecimento — Português (Brasil)

O NEI é também uma homenagem à simplicidade e à eficiência das ferramentas de software de uma época anterior da computação pessoal.

Fica aqui uma **menção honrosa e um agradecimento especial a Peter Norton**, cujo nome ficou associado a uma geração de ferramentas extraordinariamente práticas para computadores pessoais.

O Norton Editor é uma importante inspiração para este projeto. Sua interface compacta, seu fluxo de trabalho orientado pelo teclado e a preocupação em não atrapalhar o programador demonstram que um software útil não precisa ser complicado.

O NEI é um projeto independente e de código aberto. Não possui afiliação, patrocínio ou endosso de Peter Norton, Gen Digital, Symantec ou de qualquer atual ou antigo proprietário de marcas ou softwares relacionados ao nome Norton.

## Licença

NEI é distribuído sob a **Zero-Clause BSD License (`0BSD`)**. Consulte [`LICENSE`](LICENSE).

A licença do NEI aplica-se ao código e aos materiais produzidos para este projeto; a inspiração histórica no Norton Editor não transfere ao NEI direitos sobre software, documentação ou marcas de terceiros.
