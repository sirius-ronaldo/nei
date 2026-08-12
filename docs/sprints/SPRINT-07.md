# Sprint 07 — Busca, substituição e operações de arquivo

## Objetivo

Implementar a lista fechada de funcionalidades especificadas neste documento.

## Regra principal

Não fazem parte desta Sprint: tratamento de arquivos grandes, comportamento
de resize durante operações e demais atalhos ainda não especificados.

## Escopo fechado

1. Implementar busca com `Ctrl+F`.
2. Implementar substituição com `Ctrl+H`.
3. Implementar `F3 L` (Load more of the file).
4. Implementar `F3 W` (Write text to cursor).
5. Corrigir `F4 E`, marcado como confirmado, com a semântica “marcar até o fim
   da linha sem CR”.

## Busca — `Ctrl+F`

- Abre o prompt na barra de status e preenche o último texto pesquisado, quando houver.
- Após `Enter`, inicia a busca para frente a partir do cursor.
- O modo de busca permanece ativo e informa na barra de status:
  `↑`/`←` para buscar para trás, `↓`/`→` para buscar para frente e `ESC` para sair.
- A busca começa em `Ignore Case`, indicado por `I`; `C` alterna para diferenciação
  de maiúsculas e minúsculas.
- Pesquisa trechos, aceita UTF-8, não é circular e encerra simplesmente quando não
  encontra ocorrência.
- Ao encontrar, posiciona o cursor no primeiro caractere, atualiza o viewport e
  preserva os marcadores.
- `ESC` durante a digitação apenas fecha o prompt e retorna à edição.

## Substituição — `Ctrl+H`

- Abre prompt análogo ao da busca para informar o texto procurado e o texto substituto.
- Usa as mesmas regras de UTF-8, diferenciação de maiúsculas/minúsculas e busca não
  circular da busca.
- Ao iniciar, posiciona o cursor na primeira ocorrência e desativa os marcadores.
- No modo de substituição, a barra de status informa: `Enter` substitui a ocorrência
  atual e vai para a próxima, `S` pula, `A` substitui todas as ocorrências restantes
  e `ESC` encerra o modo.
- O cursor fica no início da ocorrência atual; o viewport acompanha a ocorrência.
- Substituição por texto vazio é válida. O documento fica modificado quando houver
  alteração; marcadores não são alterados.
- Deve informar continuamente a contagem no formato `3 occurrence(s) replaced`.
- Se não houver ocorrência ao iniciar, retorna à edição sem mensagem. Quando as
  ocorrências acabam durante o processo, exibe somente `No more occurrences`, mantém
  o modo ativo até `ESC` e inibe `Enter`, `S` e `A`.
- O comportamento para texto substituto contendo quebras de linha e o texto exato
  das mensagens ainda precisam ser definidos antes da implementação.

## `F3 L` — Load more of the file

- Usa o prompt e o cancelamento de `F3 A`.
- Insere o conteúdo completo do arquivo escolhido na posição do cursor, como `F4 C`,
  preservando o conteúdo atual e sem alterar o nome do arquivo da janela ativa.
- Arquivo inexistente não produz alteração; aviso é opcional.
- Cursor, viewport, marcadores e estado de modificação seguem o comportamento de `F4 C`.
- `ESC` no prompt cancela e retorna à edição.

## `F3 W` — Write text to cursor

- Só atua quando existem dois marcadores válidos; caso contrário, é ignorado.
- Solicita o nome do arquivo. Se ele existir, solicita confirmação antes de substituí-lo.
- Grava somente o bloco marcado e mantém o conteúdo no documento.
- Cursor, viewport, marcadores e estado de modificação seguem o comportamento de `F4 C`.
- A operação atua somente na janela ativa; cancelamento por `ESC` retorna à edição.

## `F4 E`

Implementar a correção confirmada: o marcador vai da posição atual até o fim da
linha, sem incluir a quebra de linha (`CR`).

## Word Wrap — `F5 W`

Ao ativar, solicita a largura e insere fisicamente as quebras de linha. Ao desativar,
preserva o texto já alterado e deixa de aplicar novas quebras nas edições posteriores.
O estado pertence à janela e não é persistido. `Ctrl+W` continua sendo deleção de
palavra à esquerda. A quebra ocorre antes da palavra que não couber; palavras não são
divididas, mesmo quando excedem a largura configurada.

## Testes mínimos

### Unidade

- busca para frente e para trás a partir do cursor, sem circularidade;
- alternância entre `Ignore Case` e diferenciação de maiúsculas/minúsculas;
- busca UTF-8 e ausência de ocorrência;
- substituição única, salto, substituição de todas e substituição vazia;
- substituição que altera o tamanho do documento, preservando o cursor na próxima ocorrência;
- `F3 L` inserindo na posição do cursor sem alterar o nome do arquivo;
- `F3 W` sem marcadores, com bloco válido, arquivo novo e confirmação de sobrescrita;
- `F4 E` sem incluir a quebra de linha;
- preservação dos marcadores e isolamento da janela ativa.

### Cenários manuais de terminal

- cancelar os prompts de busca, substituição, `F3 L` e `F3 W` com `ESC`;
- repetir a última busca e navegar pelas ocorrências com as quatro setas;
- executar substituição passo a passo e com `A`, conferindo mensagens e contagem;
- executar `F3 L` e `F3 W` em cada uma de duas janelas, verificando que a outra não muda;
- tentar `F3 W` sem bloco e sobrescrever arquivo existente, confirmando ambos os fluxos;
- executar `F4 E` em linha intermediária e na última linha.

## Critério de aceite

A Sprint só é considerada fechada quando os itens implementáveis acima estiverem
testados.
