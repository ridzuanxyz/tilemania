# Contributing to TileMania

Thank you for your interest in contributing to TileMania! This document provides guidelines and instructions for contributing to the project.

---

## 📋 Table of Contents

1. [Code of Conduct](#code-of-conduct)
2. [Getting Started](#getting-started)
3. [Development Setup](#development-setup)
4. [Project Structure](#project-structure)
5. [Coding Standards](#coding-standards)
6. [Making Changes](#making-changes)
7. [Testing](#testing)
8. [Pull Request Process](#pull-request-process)
9. [Asset Contributions](#asset-contributions)

---

## Code of Conduct

### Our Standards

- **Be respectful** and inclusive
- **Be constructive** in feedback
- **Focus on** what's best for the project and community
- **Show empathy** towards other contributors

This is an educational project for children ages 7-12. Keep all content age-appropriate.

---

## Getting Started

### Areas Where We Need Help

**High Priority:**
- 🎵 **Audio assets** (music tracks, sound effects)
- 🎨 **Visual assets** (sprites, backgrounds, fonts)
- 🧪 **Testing** on different platforms
- 🐛 **Bug reports** and fixes
- 📚 **Documentation** improvements

**Medium Priority:**
- ✨ **Feature enhancements**
- 🎮 **Gameplay balance** suggestions
- 🌐 **Translations** (future)
- ♿ **Accessibility** improvements

**Nice to Have:**
- 🎓 **Educational content** suggestions
- 🎪 **Marketing** materials
- 📹 **Demo videos** or GIFs

---

## Development Setup

### Prerequisites

1. **Install Rust** (1.70+)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Install system dependencies** (Linux only)
   ```bash
   sudo apt-get install libasound2-dev libudev-dev
   ```

3. **Clone the repository**
   ```bash
   git clone https://github.com/ridzuanxyz/tilemania.git
   cd tilemania
   ```

4. **Build and run**
   ```bash
   cargo run --release
   ```

See [BUILD_GUIDE.md](BUILD_GUIDE.md) for detailed instructions.

---

## Project Structure

```
tilemania/
├── src/
│   ├── main.rs              # Entry point
│   ├── lexicon/             # Word validation (CSW24)
│   ├── scoring/             # Scrabble scoring engine
│   ├── plugins/             # Core plugins
│   ├── ui/                  # UI plugin
│   ├── stage1/              # Falling Letters stage
│   ├── stage2/              # Tile Matching stage
│   ├── stage3/              # Classic Board stage
│   ├── stage4/              # Speed Challenge stage
│   └── stage5/              # AI Tournaments stage
│
├── assets/                  # Game assets (fonts, audio, sprites)
├── docs/                    # Documentation
├── tests/                   # Integration tests
└── benches/                 # Performance benchmarks
```

### Stage Module Pattern

Each stage follows the same 8-module structure:

```
stageN/
├── mod.rs           # Plugin, resources, state
├── components.rs    # ECS components
├── systems.rs       # Core gameplay logic
├── difficulty.rs    # 5 difficulty levels
├── visuals.rs       # Visual effects
├── ui.rs            # Screens and HUD
├── pause.rs         # Pause menu
└── audio.rs         # Audio events
```

**When adding to a stage:**
- Follow this pattern
- Keep modules focused and single-purpose
- Maintain consistency across stages

---

## Coding Standards

### Rust Style Guide

We follow the official [Rust Style Guide](https://doc.rust-lang.org/nightly/style-guide/).

**Key points:**
- Use `rustfmt` for formatting
- Use `clippy` for linting
- Maximum line length: 100 characters
- Use meaningful variable names
- Document public APIs

### Running Code Quality Tools

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check

# Run linter
cargo clippy

# Run linter with all features
cargo clippy --all-features -- -D warnings

# Fix common issues automatically
cargo clippy --fix
```

### Naming Conventions

**Rust Conventions:**
- `snake_case` for functions, variables, modules
- `PascalCase` for types, structs, enums
- `SCREAMING_SNAKE_CASE` for constants
- `snake_case` for file names

**Examples:**
```rust
// Good
const MAX_COMBO: u32 = 3;
struct GameState { ... }
fn calculate_score() { ... }

// Bad
const maxCombo: u32 = 3;
struct gameState { ... }
fn CalculateScore() { ... }
```

### Comments and Documentation

```rust
/// Public API documentation (uses ///)
///
/// # Arguments
/// * `word` - The word to validate
///
/// # Returns
/// * `true` if word is in CSW24 lexicon
pub fn is_valid_word(word: &str) -> bool {
    // Implementation comments use //
    self.lexicon.contains(word)
}
```

---

## Making Changes

### Workflow

1. **Create a branch**
   ```bash
   git checkout -b feature/your-feature-name
   # OR
   git checkout -b fix/your-bug-fix
   ```

2. **Make your changes**
   - Follow coding standards
   - Add tests if applicable
   - Update documentation

3. **Test your changes**
   ```bash
   cargo test
   cargo clippy
   cargo fmt -- --check
   ```

4. **Commit your changes**
   ```bash
   git add .
   git commit -m "feat: add new power-up system"
   ```

5. **Push to your fork**
   ```bash
   git push origin feature/your-feature-name
   ```

6. **Create a Pull Request**

### Commit Message Format

We use [Conventional Commits](https://www.conventionalcommits.org/):

**Format:**
```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

**Types:**
- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation changes
- `style:` Code style changes (formatting, etc.)
- `refactor:` Code refactoring
- `test:` Adding or updating tests
- `chore:` Maintenance tasks

**Examples:**
```
feat(stage1): add rainbow tile power-up
fix(lexicon): handle uppercase words correctly
docs(readme): update installation instructions
test(scoring): add tests for combo multiplier
```

### Branch Naming

**Format:** `<type>/<description>`

**Examples:**
- `feature/stage6-implementation`
- `fix/audio-crackling-issue`
- `docs/api-documentation`
- `refactor/scoring-system`

---

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests for specific module
cargo test lexicon::

# Run with output
cargo test -- --nocapture

# Run ignored tests
cargo test -- --ignored
```

### Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_validation() {
        let lexicon = Lexicon::new();
        assert!(lexicon.is_valid("AA"));
        assert!(!lexicon.is_valid("ZZZ"));
    }

    #[test]
    #[should_panic(expected = "Empty word")]
    fn test_empty_word_panics() {
        validate_word("");
    }
}
```

### Test Coverage

**Aim for:**
- Core logic: 80%+ coverage
- Game mechanics: 60%+ coverage
- UI code: 40%+ coverage

**Check coverage:**
```bash
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

---

## Pull Request Process

### Before Submitting

**Checklist:**
- [ ] Code follows Rust style guide
- [ ] All tests pass (`cargo test`)
- [ ] Clippy shows no warnings (`cargo clippy`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] Documentation is updated
- [ ] Commit messages follow convention
- [ ] Branch is up to date with main

### PR Description Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] Tested locally
- [ ] Added new tests
- [ ] All tests pass

## Screenshots (if applicable)
[Add screenshots or GIFs]

## Checklist
- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Comments added for complex code
- [ ] Documentation updated
- [ ] No new warnings
```

### Review Process

1. **Automated checks** run on PR
2. **Code review** by maintainer
3. **Feedback** addressed
4. **Approval** and merge

**Review criteria:**
- Code quality and style
- Test coverage
- Documentation
- Performance impact
- Breaking changes

---

## Asset Contributions

### Audio Assets

See [ASSET_SPECIFICATIONS.md](ASSET_SPECIFICATIONS.md) for detailed specs.

**Requirements:**
- Format: OGG Vorbis
- Sample Rate: 44.1 kHz
- Bit Rate: 96-192 kbps
- No copyright issues
- Age-appropriate content

**Submission:**
```bash
# Add audio file
cp your-sound.ogg assets/audio/sfx/stage1/

# Test in game
cargo run --release

# Submit PR with asset
```

### Visual Assets

**Requirements:**
- Format: PNG (transparency supported)
- High resolution (vector preferred)
- Consistent style
- No copyright issues
- Child-friendly design

**Style Guide:**
- Clean, sans-serif fonts
- High contrast for readability
- Friendly, educational theme
- Not too juvenile (ages 7-12)

---

## Code Review Guidelines

### As a Reviewer

**Do:**
- ✅ Be constructive and helpful
- ✅ Explain the "why" behind suggestions
- ✅ Acknowledge good work
- ✅ Test the changes locally
- ✅ Focus on substance over style

**Don't:**
- ❌ Nitpick minor style issues
- ❌ Block PRs over preferences
- ❌ Leave vague comments
- ❌ Ignore good practices

### As a Contributor

**Do:**
- ✅ Respond to all feedback
- ✅ Ask questions if unclear
- ✅ Be open to suggestions
- ✅ Update PR based on feedback
- ✅ Thank reviewers

**Don't:**
- ❌ Take feedback personally
- ❌ Ignore review comments
- ❌ Force push without warning
- ❌ Argue about style preferences

---

## Performance Considerations

### Profiling Changes

```bash
# Build with release optimizations
cargo build --release

# Run with profiling (Linux)
perf record --call-graph dwarf ./target/release/tilemania

# View results
perf report
```

### Performance Guidelines

- Avoid allocations in hot loops
- Use `&str` over `String` where possible
- Prefer iterators over indexing
- Cache expensive computations
- Profile before optimizing

---

## Documentation

### Code Documentation

```rust
/// Brief one-line summary
///
/// Longer description explaining:
/// - What the function does
/// - When to use it
/// - Important notes
///
/// # Arguments
/// * `param` - Description of parameter
///
/// # Returns
/// * Description of return value
///
/// # Examples
/// ```
/// let result = function_name(param);
/// assert_eq!(result, expected);
/// ```
pub fn function_name(param: Type) -> ReturnType {
    // Implementation
}
```

### Updating Documentation

When making changes, update:
- Code comments
- API documentation (`///`)
- README.md (if user-facing)
- CHANGELOG.md (for releases)
- Architecture docs (if structural)

---

## Questions?

**Where to ask:**
- GitHub Issues: Bug reports, feature requests
- GitHub Discussions: General questions
- Pull Requests: Code-specific questions

**Response time:**
- Issues: 1-3 days
- PRs: 2-5 days
- Urgent bugs: Same day

---

## License

By contributing to TileMania, you agree that your contributions will be licensed under the same license as the project (see LICENSE file).

---

## Recognition

Contributors will be acknowledged in:
- README.md (Contributors section)
- CHANGELOG.md (for significant contributions)
- In-game credits (for asset contributions)

---

**Thank you for contributing to TileMania!** 🎉

Every contribution, no matter how small, helps make this educational game better for children learning Scrabble.

---

**Last Updated:** November 19, 2025
**Questions?** Open an issue or discussion on GitHub
