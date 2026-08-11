# Referência visual — levantamento do Norton Editor 1.3

Este documento descreve as capturas usadas durante a especificação. As imagens históricas não são redistribuídas neste repositório nesta fase; o objetivo é registrar características funcionais/visuais observadas, não copiar ativos do software original.

## Tela de abertura

Quando executado sem arquivo, o editor de referência apresenta `Enter file name:` e uma caixa de identificação aproximadamente centralizada no terço superior. O restante da tela é praticamente vazio.

Para o NEI, usar composição própria, com identidade `NEI — Norton Editor Inspired`, sem copiar avisos de copyright do produto histórico.

## Tela de edição

- fundo preto;
- texto claro;
- cursor em bloco;
- ausência de barra superior permanente;
- status na última linha quando uma única janela está ativa;
- formato observado: `Line=... Col=... <arquivo> Insert WW=Off`.

## Primeiro F4 S

Após o primeiro `F4 S`, um marcador permanece na margem na posição definida enquanto o cursor pode ser deslocado para outra parte do documento.

## Segundo F4 S

Após o segundo marcador:

- ambos aparecem na margem;
- conteúdo entre eles recebe maior intensidade/brilho;
- cursor permanece independente dos marcadores.

## Duas janelas

`F3 X` resulta em divisão horizontal. Quando a segunda janela ainda não possui arquivo, ela mostra `Enter file name:`. Após abertura, as duas áreas exibem documentos distintos.

A barra de status ocupa a linha divisória/rodapé da janela ativa e reflete seu cursor/arquivo.

## F3

A barra contextual observada é:

```text
F3 FILE: Exit-with-save   Quit   Save   eXchange-windows   New   Append   L   W   C
```

## F4

A barra contextual observada é:

```text
F4 BLOCK: Set-marker   Copy   Move   Delete-block   Remove-marker   W   L   E   F
```

## Princípio de reprodução

A meta é reproduzir linguagem visual, proporções e comportamento geral, não realizar cópia pixel a pixel de material proprietário ou usar branding que sugira produto oficial.
