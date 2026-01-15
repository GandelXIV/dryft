# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## About Dryft

Dryft is an experimental stack-based concatenative programming language featuring:
- Simple, extensible syntax
- Pure (fun) and impure (act) function distinction
- Linear types and stack-based resource management
- Optional type inference
- Minimal control flow primitives (cycle, then, elect)

## Build and Development Commands

### Building and Running

```bash
# Build the compiler
cargo build --release

# Compile a .dry file with default target (cc)
cargo run -- examples/fizzbuzz.dry

# Run compilation and execute immediately
cargo run -- examples/fizzbuzz.dry --run

# Specify a target backend
cargo run -- examples/fizzbuzz.dry --target cc
cargo run -- examples/fizzbuzz.dry --target elf
cargo run -- examples/fizzbuzz.dry --target ccopt

# Use a custom target descriptor
cargo run -- examples/fizzbuzz.dry --custom-target path/to/target.toml

# Output only assembly (no external compilation)
cargo run -- examples/fizzbuzz.dry --assembly-only

# Output only object file (no linking)
cargo run -- examples/fizzbuzz.dry --object-only

# Specify assembly output file
cargo run -- examples/fizzbuzz.dry --assembly-out build/output.c
```

### REPL

```bash
# Start REPL (no input file)
cargo run

# REPL commands
.help      # Display help
.exit      # Exit REPL
```

Note: NASM64 backend is currently unsupported for REPL mode.

### Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test simple_parse
```

## Architecture Overview

### Compilation Pipeline

The compiler follows a multi-stage pipeline:

1. **Frontend (src/frontend.rs)**: Tokenizes and compiles .dry source into intermediate representation
2. **Backend (src/backends.rs)**: Transforms IR into target-specific code (C99 or NASM64)
3. **External Tooling**: Uses system tools (gcc, nasm, ld) to produce final executable

The process is orchestrated by main.rs using target descriptors (src/targets/*.toml).

### Key Components

**Frontend (src/frontend.rs)**
- `compile()`: Main compilation function that tokenizes source and builds IR
- `CompileState`: Maintains compilation state including definition stack, body stack, variable scopes, and type stack
- Token handling uses a stack-based approach where definitions (fun, act, then, elect, cycle) push onto `defnstack`
- Variables are tracked per-scope in `varscopes` vector
- Actions cannot be called from inside functions (enforced by `before_action()`)

**Backend Trait (src/backends.rs)**
- Defines interface that all code generation backends must implement
- Key methods: `create_function()`, `push_integer()`, `push_string()`, `complete()`
- Backends: `C99Backend` (fully implemented), `Nasm64Backend` (partially implemented)
- Each backend provides its own implementations for stack operations, control flow, and variable management

**Target System (src/targets/)**
- TOML descriptors define compilation pipeline for each target
- Specify backend, intermediate file paths, and shell commands for stdlib/assemble/link/interpret steps
- Platform-specific configurations (unix/windows)

**Main (src/main.rs)**
- Parses CLI arguments using clap
- Loads target descriptors
- Coordinates build steps: build_file() -> stdlib() -> assemble() -> link() -> interpret()
- REPL mode wraps user input in `act: main { input } ;` and executes

### Language Semantics

**Functions vs Actions**
- `fun:` defines pure functions (cannot call actions, can be called from anywhere)
- `act:` defines impure actions (can call other actions and functions, cannot be called from functions)
- Main entry point must be `act: main`

**Control Flow**
- `then:` / `:then` - conditional blocks
- `elect:` / `:elect` - exclusive multi-branch conditionals (like switch/match)
- `cycle:` / `:cycle` - infinite loops (use `break` to exit)
- Semicolon `;` can terminate most blocks alternatively

**Variables**
- Declared with `var name` (pops value from stack)
- Read with `$name` (pushes to stack)
- Write with `name!` (pops value from stack)
- Scoped to containing block

**Include System**
- `include path/to/file` includes .dry files (automatically appends .dry extension)
- Standard library in lib/std/ (e.g., `include lib/std/io`)

### Backend Implementation Details

**C99 Backend**
- Generates C code with stack operations implemented as function calls
- User functions prefixed with `fun_` to avoid naming conflicts
- Stack managed by runtime (base.c)
- Uses GNU label-as-values for elect blocks
- Variables become C local variables with `var_` prefix

**NASM64 Backend**
- Generates x86-64 assembly
- Many features incomplete (marked with `todo!()`)
- Stack operations use NASM macros defined in base.asm
- Last working commit: 009e79bc974da89b12e591e17a35e9e99c8fe759

## Important Development Notes

- The frontend uses `prepend` mechanism to inject included files into the token stream
- Line numbers track source location for error reporting but are affected by includes
- Definition stack (`defnstack`) determines parsing context for tokens
- Body stack (`bodystack`) accumulates generated code at different nesting levels
- Type stack (`typestack`) tracks types for potential future type checking
- Main function must be an action, not a function (enforced at compile time)
- NASM64 backend is significantly incomplete compared to C99
