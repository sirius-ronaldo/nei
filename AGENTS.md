# AGENTS.md — Regras para agentes de implementação

Este arquivo define regras obrigatórias para agentes de código (incluindo Codex) que trabalharem no NEI.

## 1. Fonte de verdade

A ordem de precedência é:

1. especificação da Sprint atual em `docs/sprints/`;
2. `docs/SPECIFICATION.md`;
3. `docs/COMMANDS.md`;
4. imagens locais de referência em `docs/reference-images/`, quando disponíveis;
5. `docs/ARCHITECTURE.md`;
6. `README.md`.

Se houver contradição, NÃO invente uma resolução silenciosa. Registre a inconsistência e peça decisão humana.

## 2. Imagens de referência do Norton Editor 1.3

Durante o levantamento funcional do NEI foram produzidas capturas do Norton Editor 1.3 original. Elas servem como referência visual e comportamental para a implementação.

Essas imagens são **material local de referência** e NÃO fazem parte do código-fonte do NEI. Por esse motivo, a pasta `docs/reference-images/` é ignorada pelo Git e pode não existir ou estar vazia em clones do repositório.

Quando disponíveis, as imagens devem ser consultadas para conferir layout, posicionamento, intensidade de texto, cursor, marcadores e barras contextuais. Elas complementam a documentação escrita, mas sua ausência não autoriza o agente a inventar comportamentos não documentados.

Pasta sugerida:

```text
docs/reference-images/
```

Arquivos de referência atualmente previstos:

| Arquivo | Referência |
|---|---|
| `ne_help.png` | Tela de ajuda do NE 1.3; referência para comandos de cursor, deleção, arquivo e blocos. |
| `tela_principal.png` | Tela inicial quando o editor é executado sem nome de arquivo; mostra `Enter file name:` e o quadro de identificação. |
| `tela_de_edicao.png` | Tela normal de edição; referência para área de texto, cursor em bloco e barra de status. |
| `set_block_marker_1.png` | Estado após o primeiro `F4 S`; mostra o primeiro marcador de bloco e o cursor já deslocado. |
| `set_block_marker_2.png` | Estado após o segundo `F4 S`; mostra os dois marcadores e o realce por intensidade do bloco delimitado. |
| `new_window_F3_X.png` | Estado ao acionar `F3 X` sem a segunda janela possuir arquivo; mostra a divisão da tela e o prompt para nome do arquivo. |
| `two_windows.png` | Duas janelas com arquivos abertos; referência para divisão horizontal, posição da barra de status e janela ativa. |
| `status_f3.png` | Barra contextual apresentada após pressionar `F3` (`F3 FILE`). |
| `status_f4.png` | Barra contextual apresentada após pressionar `F4` (`F4 BLOCK`). |

Ao implementar uma funcionalidade associada a uma dessas telas, compare a implementação com a imagem correspondente sempre que ela estiver disponível no ambiente local.

Não copie para o projeto código, executáveis, fontes, logos ou outros recursos do Norton Editor original. As imagens são exclusivamente referências para a reimplementação independente do comportamento e da interface observados.

## 3. Fidelidade clássica

O NEI é inspirado no Norton Editor 1.3. Comportamentos marcados como **CONFIRMED** ou **OBSERVED** devem ser preservados.

Não modernize atalhos ou UX por iniciativa própria. Exemplos proibidos sem decisão explícita:

- substituir `F3 S` por `Ctrl+S`;
- transformar marcadores de bloco em seleção moderna;
- substituir duas janelas por abas;
- introduzir menus gráficos permanentes;
- adicionar mouse como requisito;
- trocar a barra contextual de `F3`/`F4` por menus de widgets.

Recursos modernos poderão ser discutidos depois da primeira versão estável, desde que não quebrem o modo clássico.

## 4. Comportamentos desconhecidos

Itens marcados como **TBD** ou **UNKNOWN** não devem ser implementados com comportamento inventado.

Se uma Sprint depender de um desses itens, implemente somente a infraestrutura segura e deixe o ponto explicitamente pendente, ou peça esclarecimento.

## 5. Arquitetura

- linguagem: Rust;
- alvo inicial: Linux;
- terminal: `crossterm` a partir da Sprint 01;
- não usar Ratatui sem decisão explícita;
- separar modelo de documento, entrada, comandos e renderização;
- lógica de documento deve ser testável sem terminal;
- estado de cada janela deve ser independente;
- UTF-8 é requisito interno;
- nenhuma dependência deve ser adicionada sem justificativa na mudança.

## 6. Qualidade

Antes de concluir uma Sprint:

```bash
cargo fmt --check
cargo check
cargo test
```

Quando Clippy passar a fazer parte da Sprint:

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

Não considerar a Sprint concluída se uma validação exigida falhar.

## 7. Escopo

Implemente apenas a Sprint solicitada. Não antecipe funcionalidades de Sprints futuras, salvo infraestrutura mínima indispensável e claramente documentada.

## 8. Segurança do terminal

A partir da Sprint 01, qualquer entrada em raw mode/alternate screen deve possuir caminho confiável de restauração do terminal, inclusive em erros previsíveis.

## 9. Regras de Codificação
Código claro, objetivo e de boa qualidade. Acrescente comentários relevantes, objetivos,
concisos e necessários, sempre escritos em português do Brasil (PT-BR). Caso haja uma
decisão de código entre alternativas cuja escolha seja difícil, registre a decisão no
trecho de código.

## 10. Licença

O projeto usa SPDX `0BSD`. Não substituir a licença nem inserir cabeçalhos de licença obrigatórios em cada arquivo sem decisão explícita do mantenedor.

## 11. Changelog de Desenvolvimento

O arquivo `docs/sprints/dev_changelog.md` deve ser usado como referência das Sprints implementadas.
Ao iniciar uma nova Sprint, considere que todas as Sprints anteriores já foram implementadas.
Adicione ao arquivo, caso ainda não conste, uma linha `SprintNN` para a Sprint atual, sem `ok`.
Ao iniciar a Sprint seguinte, altere a linha da Sprint anterior para `SprintNN ok`.
Sprints anteriores que ainda não constarem no arquivo devem ser adicionadas já no formato
`SprintNN ok`. Nunca remova registros anteriores.
