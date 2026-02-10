# Agent Instructions

This file defines mandatory rules for AI agents working in this repository. Follow it strictly.

## Hard Rules
- Never assume missing requirements.
- Ask clarifying questions if requirements are ambiguous.
- Do not introduce new dependencies without explicit approval.
- Always run all tests after making any change.
- Do not modify generated files.
- Do not commit secrets.

## Workflow
1. Read relevant specs / code.
2. If unclear: ask questions.
3. Write or update tests first when applicable.
4. Implement changes.
5. Run all tests.
6. Report results and commands executed.

Never skip steps.

## Correctness over convenience
- Model the full error space—no shortcuts or simplified error handling.
- Handle all edge cases, including race conditions, signal timing, and platform differences.
- Use the type system to encode correctness constraints.
- Prefer compile-time guarantees over runtime checks whenever possible.
- Maintain consistency across platforms even when underlying OS capabilities differ.
  Use OS-native logic rather than trying to emulate Unix on Windows (or vice versa).

## Documentation
- Use inline comments to explain "why," not just "what".
- No narrative comments in function bodies. Only add a comment if what you're doing is non-obvious or special in some
  way, or if something needs a deeper "why" explanation.
- Module-level documentation should explain purpose and responsibilities.
- Always use periods at the end of code comments.
- Never use title case in headings and titles. Always use sentence case.

## Rust
- Prefer Result over panic.
- Use type system extensively: newtypes, builder patterns, type states, lifetimes.

## RabbitMQ
- Verify message schema compatibility before changes
- Document new message types and routing keys

## Output
- Be concise
- Show diffs when possible
- Always list executed commands
- No emojis

## Testing and Quality
- Test after every change
- Run: `cargo llvm-cov --html` for coverage reports
- Aim for 100% test coverage, use reports to find gaps
- Test comprehensively, including edge cases, race conditions, and stress tests.

## Deployment
- Ensure graceful shutdown (stop consuming, finish processing, close connections)
- Keep services independently deployable
- Services must pass Docker health checks
