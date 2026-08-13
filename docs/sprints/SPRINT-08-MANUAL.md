# Roteiro manual — Sprint 08

Este roteiro deve ser executado por uma pessoa que não participou do desenvolvimento.
Não é necessário conhecer Rust, o código-fonte ou os detalhes internos do NEI.

## Preparação

Use um terminal Linux com pelo menos 80 colunas por 24 linhas. Tenha o comando `nei`
disponível no terminal.

Crie dois arquivos de teste, em uma pasta temporária, com estes conteúdos:

`nei-s08-a.txt`

```text
Primeira linha com acentuação: ação, órgão e informação.
Segunda linha para testar a janela principal.
Terceira linha muito longa: abcdefghijklmnopqrstuvwxyz 0123456789.
```

`nei-s08-b.txt`

```text
Arquivo da segunda janela.
Outra linha com UTF-8: maçã, café e coração.
```

Durante cada teste, anote o resultado observado. Se houver falha, registre a tecla
pressionada, o texto exibido e, se possível, faça uma captura de tela.

## Teste 1 — inicialização e arquivo vazio

1. Abra um arquivo vazio com `nei arquivo-vazio.txt`.
2. Confirme que o editor abre sem erro e apresenta uma área de edição utilizável.
3. Digite `ação` e pressione Enter.
4. Use `F3 S` para salvar e depois `F3 Q` para sair.
5. Abra novamente o arquivo.

Resultado esperado: o arquivo inexistente abre como documento vazio, aceita texto
UTF-8, salva e reabre sem perder o conteúdo nem apresentar caracteres corrompidos.

## Teste 2 — linhas longas e UTF-8

1. Abra `nei-s08-a.txt`.
2. Vá até a linha longa usando as setas, `End`, `Home`, `Ctrl+End` e `Ctrl+Home`.
3. Insira e apague caracteres acentuados em diferentes posições.
4. Redimensione o terminal horizontalmente e volte ao tamanho original.

Resultado esperado: nenhuma operação encerra o editor, o cursor permanece em posição
válida, a linha não perde caracteres e o texto UTF-8 continua correto.

## Teste 3 — resize frequente e terminal pequeno

1. Abra `nei-s08-a.txt`.
2. Redimensione repetidamente a janela do terminal, alternando entre tamanhos grandes
   e pequenos.
3. Repita usando um terminal com aproximadamente 40 colunas por 10 linhas.
4. Navegue, insira um caractere e remova-o.

Resultado esperado: a tela é redesenhada sem travar ou encerrar inesperadamente. Em um
terminal pequeno, parte do conteúdo pode ficar oculta, mas não deve haver panic, erro
visual permanente ou perda do documento.

## Teste 4 — duas janelas e estado independente

1. Abra `nei-s08-a.txt`.
2. Pressione `F3 X` e abra `nei-s08-b.txt` quando o prompt solicitar o arquivo.
3. Em cada janela, mova o cursor para uma posição diferente.
4. Coloque dois marcadores com `F4 S` em `nei-s08-a.txt`.
5. Pressione `F3 X` para alternar entre as janelas várias vezes.
6. Edite somente a janela ativa.

Resultado esperado: cada janela mantém seu próprio arquivo, cursor, viewport e
marcadores. Editar uma janela não altera o conteúdo da outra.

## Teste 5 — marcadores fora do viewport

1. Abra `nei-s08-a.txt`.
2. Coloque os dois marcadores em linhas diferentes com `F4 S`.
3. Navegue várias páginas para longe dos marcadores com `PgDn` ou pelas setas.
4. Volte até os marcadores.
5. Execute `F4 R`.

Resultado esperado: os marcadores continuam válidos mesmo fora da área visível,
voltam a aparecer quando a linha entra no viewport e são removidos por `F4 R`.

## Teste 6 — deleção e restauração

1. Abra `nei-s08-a.txt`.
2. Posicione o cursor no meio de uma palavra e use Backspace, Delete, `Ctrl+W`,
   `Ctrl+L` ou `Alt+L`.
3. Pressione `Ctrl+U` uma vez.
4. Repita uma segunda deleção e use `Ctrl+U` novamente.

Resultado esperado: `Ctrl+U` restaura somente a última deleção, exatamente uma vez.
Não deve restaurar uma pilha de deleções antigas.

## Teste 7 — Word Wrap físico

1. Abra `nei-s08-a.txt`.
2. Pressione `F5 W` e informe uma largura pequena, como `20`.
3. Verifique as quebras inseridas e observe a barra de status.
4. Confirme que palavras não são divididas; uma palavra longa pode exceder a largura.
5. Pressione `F5 W` novamente para desativar.
6. Edite o texto e observe a barra de status.

Resultado esperado: a ativação insere quebras físicas antes de palavras que não
couberem. A barra continua exibindo linha, coluna, arquivo, modo e `WW=On/Off`.
Ao desativar, as quebras existentes permanecem e novas edições não criam novas
quebras automaticamente.

## Teste 8 — busca e substituição

1. Abra `nei-s08-a.txt`.
2. Pressione `Ctrl+F`, pesquise `linha` e navegue com as quatro setas.
3. Pressione `ESC` e repita usando `F5 F`.
4. Pressione `Ctrl+H`, pesquise `linha` e substitua por `linha nova`.
5. Use Enter para substituir uma ocorrência, `S` para saltar outra e `A` para
   substituir as restantes.
6. Teste uma busca que não existe.
7. Teste uma substituição iniciada sem ocorrência.

Resultado esperado: busca e substituição não são circulares. Busca sem ocorrência e
substituição sem ocorrência inicial retornam silenciosamente à edição. A contagem da
substituição aparece continuamente no formato `3 occurrence(s) replaced`.

## Teste 9 — fim das ocorrências na substituição

1. Abra um arquivo com pelo menos duas ocorrências da mesma palavra.
2. Inicie `Ctrl+H` e substitua uma ocorrência com Enter.
3. Continue até não haver mais ocorrências.
4. Pressione Enter, `S` e `A` depois de aparecer `No more occurrences`.
5. Pressione `ESC`.

Resultado esperado: a barra exibe somente `No more occurrences   ESC Edit`; o modo permanece
ativo, Enter/S/A não produzem efeito e somente `ESC` retorna à edição.

## Teste 10 — prompts, cancelamento e saída

1. Pressione `F3 L` e cancele o prompt com `ESC`.
2. Pressione `F3 W` sem bloco válido e confirme que nada é gravado.
3. Crie um bloco, pressione `F3 W`, informe um arquivo novo e confirme a gravação.
4. Repita usando um arquivo existente e teste a confirmação de sobrescrita.
5. Abra cada prompt de busca, substituição e Word Wrap e cancele com `ESC`.
6. Pressione `ESC` no modo normal de edição.

Resultado esperado: os prompts cancelados retornam à edição sem corromper a tela.
`ESC` no modo normal não fecha o editor. A saída deve ocorrer por `F3 E` ou `F3 Q`.

## Teste 11 — restauração do terminal

1. Abra o editor e confirme que o cursor e o terminal estão em modo de edição normal.
2. Saia usando `F3 E` ou `F3 Q`.
3. Execute um comando comum no mesmo terminal, como `echo terminal restaurado`.
4. Repita o teste e interrompa o editor com `Ctrl+C`, se o ambiente permitir.

Resultado esperado: após sair, o prompt do shell aparece normalmente, o cursor está
visível e os caracteres digitados não ficam ocultos ou deslocados.

## Registro final

Para cada teste, registre `OK` ou `FALHOU`, acrescente observações e anexe capturas
de tela quando houver diferença visual. Uma falha deve informar o cenário, o arquivo
usado, o tamanho aproximado do terminal e a sequência de teclas que levou ao problema.
