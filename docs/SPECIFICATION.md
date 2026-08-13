# Especificação Funcional e Visual — NEI

## 1. Propósito

NEI — Norton Editor Inspired — é um editor full-screen para terminal Linux inspirado no Norton Editor 1.x, tendo como referência funcional observada a versão 1.3.

A prioridade até a primeira versão estável é reproduzir a filosofia, a interação por teclado e as principais características visuais clássicas sem importar convenções de editores modernos que alterem esse comportamento.

## 2. Terminologia de evidência

- **CONFIRMED**: comportamento testado diretamente no editor de referência e descrito pelo levantamento do projeto.
- **OBSERVED**: característica visível nas capturas de tela disponíveis.
- **TBD**: detalhe ainda não suficientemente conhecido.

## 3. Inicialização

### 3.1 Sem nome de arquivo — CONFIRMED

Execução:

```bash
nei
```

Deve iniciar no fluxo equivalente à tela de abertura, com prompt:

```text
Enter file name:
```

A identidade visual do NEI deve ser apresentada em composição retrô inspirada na disposição da tela histórica, sem reproduzir identificação autoral/copyright do software original.

### 3.2 Com nome de arquivo — CONFIRMED

Execução:

```bash
nei arquivo.txt
```

Deve abrir diretamente a tela de edição do arquivo informado, sem passar pela tela inicial de solicitação de nome.

## 4. Tela de edição

### 4.1 Layout — OBSERVED

- fundo predominantemente preto;
- texto de edição claro;
- área de edição ocupa praticamente toda a tela;
- não existe barra superior permanente;
- barra de status clara na linha de separação/rodapé correspondente à janela ativa;
- cursor em bloco retangular;
- interface baseada em células do terminal, sem widgets gráficos modernos.

### 4.2 Barra de status — OBSERVED

Formato conceitual:

```text
Line=5    Col=5                  /caminho/arquivo.txt             Insert    WW=Off
```

Campos observados:

- linha atual;
- coluna atual;
- arquivo/caminho;
- modo de edição (`Insert`; comportamento alternativo será detalhado em Sprint própria);
- estado de Word Wrap (`WW=Off` observado).

### 4.3 Ajuda — `F1`

`F1` abre uma única tela de ajuda, sem rolagem, adaptada aos comandos realmente
implementados no editor. Comandos equivalentes são exibidos lado a lado, como
`Ctrl+F`/`F5 F` e `Ctrl+H`/`F5 R`. Qualquer tecla fecha a tela e retorna à edição.
A barra de status normal informa `F1-Help` junto dos demais campos.

## 5. Cursor — CONFIRMED/OBSERVED

O cursor é um retângulo/bloco ocupando a célula atual. O NEI deve solicitar cursor em bloco ao terminal e possuir fallback visual adequado quando necessário.

## 6. Comandos prefixados

### 6.1 F3 — FILE — CONFIRMED

Ao pressionar `F3`, a barra de status é substituída temporariamente pela barra contextual de comandos de arquivo. O editor aguarda a segunda tecla.

Barra observada:

```text
F3 FILE: Exit-with-save   Quit   Save   eXchange-windows   New   Append   L   W   C
```

Após a execução/cancelamento do comando, a barra normal deve ser restaurada.

### 6.2 F4 — BLOCK — CONFIRMED

Ao pressionar `F4`, a barra de status é substituída temporariamente pela barra contextual de blocos.

Barra observada:

```text
F4 BLOCK: Set-marker   Copy   Move   Delete-block   Remove-marker   W   L   E   F
```

## 7. Block markers

### 7.1 Definição — CONFIRMED

`F4 S` coloca um marcador na posição atual.

Primeiro `F4 S`:

1. registra primeiro marcador;
2. marcador permanece ancorado;
3. cursor pode navegar livremente.

Segundo `F4 S`:

1. registra segundo marcador;
2. os dois marcadores delimitam o bloco.

### 7.2 Aparência — CONFIRMED/OBSERVED

- marcadores são visíveis na margem de edição;
- texto entre os marcadores recebe realce por intensidade/brilho;
- não usar seleção moderna com fundo colorido como representação principal.

### 7.3 Remoção — CONFIRMED

`F4 R` remove os marcadores.

### 7.4 Operações de bloco — CONFIRMED

O intervalo começa na posição do marcador inicial e termina imediatamente antes
da posição do marcador final. A ordem dos marcadores pode ser invertida. Um
marcador no início da linha seguinte inclui a quebra de linha anterior; um
marcador no fim da linha não inclui quebra adicional.

`F4 C` copia o bloco para o cursor, mantém os marcadores e posiciona o cursor no
fim da cópia. `F4 M` move o bloco para o cursor, remove os marcadores e posiciona
o cursor no fim. `F4 D` apaga o bloco e remove os marcadores. `F4 L` marca a linha
do início até o início da próxima, ou até o fim na última linha. `F4 F` localiza
o marcador posterior mais próximo do cursor. Uma segunda marcação na mesma posição
é ignorada.

## 8. Duas janelas

### 8.1 Criação/troca — CONFIRMED

Com um arquivo em edição, `F3 X` disponibiliza a segunda janela. Quando ela ainda não possui arquivo, apresenta prompt de nome de arquivo equivalente ao fluxo inicial. Se o nome informado não existir, a segunda janela abre como documento vazio e mantém o nome para gravação posterior.

Com as duas janelas abertas, `F3 X` alterna a janela ativa.

### 8.2 Layout — CONFIRMED/OBSERVED

- divisão horizontal;
- cada janela mantém conteúdo independente;
- a barra de status/separação reflete a janela em que está o cursor;
- o cursor em bloco identifica a janela ativa.

### 8.3 Estado independente — CONFIRMED por comportamento observado

Cada janela deve manter ao menos:

- documento;
- cursor;
- viewport;
- marcadores de bloco;
- modo de edição;
- Word Wrap.

### 8.4 Transferência de bloco — CONFIRMED

`F4 W` copia bloco da outra janela para a posição atual da janela ativa.

## 9. Undelete

### Ctrl+U — CONFIRMED

`Ctrl+U` restaura **somente a última deleção**. Não representa uma pilha moderna de undo encadeado.

Internamente a implementação pode usar uma estrutura apropriada, mas o comportamento clássico exposto ao usuário deve ser de uma única restauração da última operação de deleção elegível.

## 10. Append

### F3 A — CONFIRMED

`F3 A` abre prompt de arquivo no mesmo estilo do prompt inicial. O conteúdo do arquivo escolhido é inserido no **final do arquivo atual**.

## 11. Load more e Write — Sprint 07

### 11.1 `F3 L` — Load more of the file

`F3 L` usa o mesmo prompt, cancelamento e fluxo de `F3 A`, mas insere o conteúdo
completo do arquivo na posição do cursor, preservando o conteúdo atual e o nome do
arquivo da janela ativa. Arquivo inexistente não altera o documento; uma mensagem de
aviso é opcional.

Cursor, viewport, marcadores e estado de modificação seguem o comportamento de `F4 C`.
A operação afeta somente a janela ativa.

### 11.2 `F3 W` — Write text to cursor

`F3 W` só atua quando os dois marcadores delimitam um bloco válido; sem marcadores
válidos a ação é ignorada. O comando solicita o nome do arquivo e pede confirmação
antes de substituir um arquivo existente. Grava somente o bloco e mantém o conteúdo
no documento.

Cursor, viewport, marcadores e estado de modificação seguem o comportamento de `F4 C`.
A operação afeta somente a janela ativa.

### 11.3 `F3 C` — fora do escopo

O comando existia na referência clássica, mas não será implementado no NEI.

## 12. Busca — `Ctrl+F`

`Ctrl+F` abre o prompt na barra de status e recupera o último texto pesquisado. Após
`Enter`, a busca começa para frente a partir do cursor e entra em modo de busca.
Nesse modo, `↑`/`←` buscam para trás, `↓`/`→` buscam para frente e `ESC` encerra
a busca e retorna à edição.

A busca começa em `Ignore Case`, indicado por `I` na barra de status. `C` alterna
para diferenciação de maiúsculas e minúsculas. Pesquisa trechos, aceita UTF-8, não
é circular e encerra simplesmente quando não encontra ocorrência. Ao encontrar,
posiciona o cursor no primeiro caractere, atualiza o viewport e não altera os
marcadores. `ESC` durante o prompt apenas o fecha e retorna à edição.

`F5 F` executa a mesma operação de `Ctrl+F`; os dois atalhos permanecem disponíveis.

## 13. Substituição — `Ctrl+H`

`Ctrl+H` abre prompt análogo ao da busca para solicitar o texto procurado e o texto
substituto. A busca usa as mesmas regras de UTF-8, diferenciação de maiúsculas e
minúsculas e ausência de circularidade.

Ao iniciar o modo de substituição, o cursor vai para o início da primeira ocorrência
e os marcadores são desativados. `Enter` substitui a ocorrência atual e posiciona o
cursor no início da próxima; `S` pula a ocorrência; `A` substitui todas as ocorrências
restantes; `ESC` encerra o modo. A barra de status informa esses comandos.

Se a substituição começar sem ocorrência, retorna imediatamente à edição, sem
mensagem. Se as ocorrências acabarem durante o processo, a barra exibe somente
`No more occurrences   ESC Edit`; o modo permanece ativo até `ESC`, e `Enter`, `S` e `A` ficam
sem efeito.

Substituição por texto vazio é válida. O viewport acompanha a ocorrência atual, os
marcadores não são alterados durante o modo e a operação informa a contagem de
ocorrências substituídas.

`F5 R` executa a mesma operação de `Ctrl+H`; os dois atalhos permanecem disponíveis.

O comportamento para texto substituto contendo quebras de linha e o texto exato das
mensagens ainda precisam de definição antes da implementação.

## 14. Word Wrap

Word Wrap é controlado por `F5 W` e pertence ao estado de cada janela. Ao ativar, o
editor solicita a largura e insere fisicamente as quebras de linha necessárias. Ao
desativar, o texto já alterado é preservado; apenas as edições posteriores deixam de
usar o Word Wrap. O estado não é persistido. `Ctrl+W` continua reservado ao comando
confirmado de deleção de palavra à esquerda em `COMMANDS.md`.
Uma palavra que não couber no espaço restante provoca a quebra imediatamente antes
dela; palavras não são divididas, mesmo quando excedem a largura configurada.
O estado `WW=On` ou `WW=Off` é exibido junto das demais informações da barra normal;
ativar ou desativar o modo não substitui a barra por uma mensagem isolada.

## 15. Princípios de compatibilidade

Até a primeira versão estável:

- comportamento clássico confirmado tem prioridade sobre conveniência moderna;
- não introduzir atalhos modernos substitutos por padrão;
- não substituir block markers por seleção convencional;
- não substituir as duas janelas por abas;
- `ESC` no modo de edição não fecha o editor; o encerramento ocorre pelos comandos
  de arquivo confirmados.
- não inventar comportamento para comandos TBD.
