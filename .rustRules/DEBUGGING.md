## Core Approach for Debugging

### Handling Unused Variables - Best Practices, follow this systematic approach:

1. **Source Tracing**
  - Identify the source module/crate of the unused variable
  - Review the source's documentation and intended purpose

2. **Import Analysis**
  - Determine why the variable was originally imported
  - Review surrounding code context and related functionality

3. **Connection Establishment**
  - Implement missing functionality that should use this variable
  - Add necessary code to create proper variable relationships
  - Document the established connections

4. **Connection Types**
  - Direct functional relationships
  - Indirect dependencies
  - Future implementation placeholders
  - Testing requirements

## Implementation Steps

### 1. Variable Investigation
- Track the variable's origin
- Review commit history if available
- Check related documentation

### 2. Relationship Building
- Implement proper variable usage
- Create necessary function calls
- Establish event handlers if needed
- Add required state management

### 3. Documentation
- Comment the established connection
- Explain the relationship logic
- Document any assumptions made

## Exception Case

### If after thorough investigation:
- No logical connection can be established
- No valid use case is found
- No future implementation is planned
- No testing requirement exists

### Then remove the variable following proper refactoring practices, ensuring to:
- Document the removal reason
- Verify no side effects
- Update related documentation
- Follow Rust's idiomatic practices for refactoring

## Error Handling
- Use thiserror for library error types
- Use anyhow for application error types
- Implement Error trait for custom error types
- Use ? operator for error propagation
- Provide context with error messages
- Handle all Result and Option cases explicitly

## Secondary Error Resolution Approach

### Instead of targeting critical errors first, adopt this strategy:
1. **Start with the simplest errors**
  - Begin with the most straightforward fixes
  - Typically, it's unused variables or warnings
  - Address low-complexity issues first
  - Build momentum through quick wins

2. **Batch Processing**
  - Group similar easy fixes together
  - Resolve multiple simple issues per commit
  - Maintain clear documentation of changes

3. **Progressive Complexity**
  - Gradually move to more complex issues
  - Use insights gained from simpler fixes
  - Build understanding through progression

## Standard Operation Procedures

### Documentation References
- Validate generated code against official documentation
- Use `cargo doc --open` for local documentation
- Check specific crates with `cargo doc --open --package <crate_name>`
- Reference standard library with `rustup doc std`
- Access online docs via `cargo docs-rs <crate_name>`

### Debugging Protocol
- Start with `cargo check` for basic validation
- Use `cargo run --debug` for development builds
- Run specific tests with `cargo test <test_name> -- --nocapture`
- Verify documentation tests using `cargo test --doc`
- Employ `cargo clippy -- -W clippy::all` for linting
- Debug macros with `cargo expand <module_name>`

### Analysis Tools
- Review dependencies: `cargo outdated && cargo tree`
- Security checks: `cargo audit`
- Identify unused dependencies: `cargo udeps`
- Performance profiling: `cargo bench && cargo flamegraph`
- Size optimization: `cargo bloat`
- Compile-time analysis: `cargo build --timings`