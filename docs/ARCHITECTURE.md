# Arquitetura proposta — NEI

## 1. Objetivos arquiteturais

- manter o núcleo de edição independente do terminal;
- tornar comportamento de documento testável;
- controlar diretamente a renderização por célula;
- representar comandos prefixados por uma máquina de estados explícita;
- representar as duas janelas como estados independentes;
- manter dependências reduzidas.

## 2. Stack

- Rust, edition 2024;
- Linux como plataforma inicial;
- `crossterm` a partir da Sprint 01;
- sem Ratatui na arquitetura inicial;
- UTF-8 no modelo interno.

## 3. Organização alvo

A estrutura será criada incrementalmente; não é necessário criar módulos vazios antes da Sprint que os usar.

```text
src/
├── main.rs
├── app.rs
├── command.rs
├── input.rs
├── document.rs
├── editor_window.rs
├── block.rs
├── screen.rs
├── status_bar.rs
├── prompt.rs
└── file_io.rs
```

## 4. Estado da aplicação

Modelo conceitual:

```rust
struct App {
    primary: EditorWindow,
    secondary: Option<EditorWindow>,
    active_window: ActiveWindow,
    input_mode: InputMode,
}

enum ActiveWindow {
    Primary,
    Secondary,
}

enum InputMode {
    Editing,
    FileCommand,
    BlockCommand,
    Prompt,
}
```

Os nomes e detalhes podem evoluir se os testes indicarem solução melhor, preservando a semântica.

## 5. Janela

Cada `EditorWindow` deve possuir estado próprio:

```rust
struct EditorWindow {
    document: Document,
    cursor: Position,
    viewport: Viewport,
    block: BlockMarkers,
    insert_mode: InsertMode,
    word_wrap: bool,
}
```

## 6. Blocos

O modelo deve representar dois marcadores explicitamente, não apenas uma seleção visual moderna.

```rust
struct BlockMarkers {
    first: Option<Position>,
    second: Option<Position>,
}
```

O renderer decide se uma posição pertence ao intervalo do bloco para aplicar intensidade visual.

## 7. Entrada e comandos

Teclas devem ser convertidas em intenções/comandos antes de modificar documentos. O estado `FileCommand`/`BlockCommand` resolve a segunda tecla após `F3`/`F4`.

Evitar lógica de negócio espalhada dentro do event loop do terminal.

## 8. Renderização

A renderização deve:

- desenhar apenas o que está visível;
- renderizar margem de marcadores;
- aplicar intensidade ao bloco;
- posicionar cursor em bloco;
- renderizar barra normal ou contextual conforme `InputMode`;
- adaptar-se a resize sem perder estado do documento.

## 9. I/O

Operações de arquivo devem ser isoladas do modelo de documento. Erros devem voltar à aplicação como `Result`, evitando `panic!` para erros normais de I/O.

## 10. Testes

Priorizar testes de unidade para:

- cursor;
- inserção/overwrite;
- deleções;
- undelete único;
- marcação e intervalo de blocos;
- operações de copy/move/delete;
- independência de estado entre janelas.

Testes visuais/manuais permanecem necessários para fidelidade do terminal.
