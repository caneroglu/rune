# Rune Project Restructure Plan

## Current Issues
1. ❌ `datamodel.rs` - Too generic name, contains multiple responsibilities
2. ❌ `parser_commands.rs` - Nearly empty, unclear purpose  
3. ❌ `indexing.rs` - Not being used effectively
4. ❌ Mixed responsibilities in single files
5. ❌ No proper error handling structure
6. ❌ Missing tests organization

## Proposed New Structure

```
src/
├── lib.rs                 # Public API exports
├── main.rs               # CLI entry point
├── core/                 # Database engine core
│   ├── mod.rs
│   ├── storage.rs        # Data models and persistence
│   ├── index.rs          # Patricia tree indexing logic
│   ├── engine.rs         # Main database engine
│   └── error.rs          # Custom error types
├── query/                # Query processing
│   ├── mod.rs
│   ├── parser.rs         # RQL grammar and parsing
│   ├── executor.rs       # Query execution logic
│   ├── commands.rs       # Command implementations
│   └── rql.pest          # Grammar file
├── cli/                  # Command line interface
│   ├── mod.rs
│   └── interface.rs      # CLI argument parsing and handling
├── config/               # Configuration management
│   ├── mod.rs
│   └── settings.rs       # Application settings
└── utils/                # Utility functions
    ├── mod.rs
    └── crypto.rs         # Hashing and crypto utilities

tests/
├── integration/          # Integration tests
│   ├── mod.rs
│   ├── db_operations.rs
│   ├── query_parsing.rs
│   └── cli_tests.rs
├── unit/                 # Unit tests (alongside modules)
└── fixtures/             # Test data
    ├── sample_db.bin
    └── test_queries.rql

docs/                     # Documentation
├── ARCHITECTURE.md
├── QUERY_LANGUAGE.md
└── API.md

examples/                 # Usage examples
├── basic_usage.rs
├── advanced_queries.rs
└── custom_indexing.rs
```

## Migration Steps

### Step 1: Create new directory structure
- [ ] Create `core/`, `query/`, `cli/`, `config/`, `utils/` directories
- [ ] Create proper `mod.rs` files

### Step 2: Move and refactor existing code
- [ ] Move `datamodel.rs` content to `core/storage.rs`
- [ ] Move `parser.rs` to `query/parser.rs`
- [ ] Move `terminal.rs` to `cli/interface.rs`
- [ ] Create `core/error.rs` with proper error types
- [ ] Move `Sha256Algorithm` to `utils/crypto.rs`

### Step 3: Improve separation of concerns
- [ ] Split `storage.rs` into models and persistence
- [ ] Create `query/executor.rs` for command execution
- [ ] Implement `core/engine.rs` as main orchestrator

### Step 4: Add missing components
- [ ] Implement `config/settings.rs`
- [ ] Add comprehensive error handling
- [ ] Create proper test structure

### Step 5: Update dependencies and exports
- [ ] Update `lib.rs` with new module structure
- [ ] Fix all imports across the codebase
- [ ] Update `Cargo.toml` if needed

## Benefits of New Structure

1. **Clear Separation of Concerns**: Each module has a specific responsibility
2. **Better Testability**: Easier to write unit and integration tests
3. **Maintainability**: Easier to find and modify specific functionality
4. **Scalability**: Easy to add new features without cluttering
5. **Professional Standards**: Follows Rust ecosystem conventions

## Implementation Priority

1. **High Priority**: Core restructuring (Steps 1-3)
2. **Medium Priority**: Error handling and testing (Step 4)
3. **Low Priority**: Documentation and examples (Step 5)
