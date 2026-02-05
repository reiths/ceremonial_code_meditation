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

## Rust
- Prefer Result over panic

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

## Deployment
- Ensure graceful shutdown (stop consuming, finish processing, close connections)
- Keep services independently deployable
- Services must pass Docker health checks
