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

## 8. Duas janelas

### 8.1 Criação/troca — CONFIRMED

Com um arquivo em edição, `F3 X` disponibiliza a segunda janela. Quando ela ainda não possui arquivo, apresenta prompt de nome de arquivo equivalente ao fluxo inicial.

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

## 11. Princípios de compatibilidade

Até a primeira versão estável:

- comportamento clássico confirmado tem prioridade sobre conveniência moderna;
- não introduzir atalhos modernos substitutos por padrão;
- não substituir block markers por seleção convencional;
- não substituir as duas janelas por abas;
- não inventar comportamento para comandos TBD.
