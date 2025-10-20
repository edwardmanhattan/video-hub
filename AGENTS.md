# Agent Guidelines for Video Hub

## Build/Lint/Test Commands

- **Build frontend**: `vite build`
- **Build backend**: `cargo build`
- **Dev server**: `vite dev`
- **Lint**: `prettier --check .`
- **Format**: `prettier --write .`
- **Type check**: `svelte-kit sync && svelte-check --tsconfig ./tsconfig.json`
- **No test framework configured** - run manual testing

## Code Style Guidelines

### TypeScript/JavaScript

- **Strict TypeScript**: Always use strict mode, explicit types
- **Imports**: ES6 imports, use `$lib` alias for internal modules
- **Naming**: camelCase for variables/functions, PascalCase for components
- **Error handling**: Try-catch blocks, descriptive error messages
- **State**: Use Svelte 5 `$state()` for reactive variables

### Svelte

- **Syntax**: Svelte 5 with `$state`, `$props`, `{@render children?.()}`
- **Components**: Single responsibility, props destructuring
- **Styling**: Tailwind CSS with DaisyUI components
- **Events**: Use `on:` directives, handle errors in event handlers

### Rust (Tauri Backend)

- **Error handling**: Use `Result<T, String>` for Tauri commands
- **Naming**: snake_case for functions/variables, PascalCase for types
- **File operations**: Use `std::fs`, handle errors with `?` operator
- **Platform-specific code**: Use `#[cfg(target_os = "windows")]` for OS differences

### Formatting

- **Prettier config**: Tabs, single quotes, no trailing commas, 100 char width
- **Plugins**: prettier-plugin-svelte, prettier-plugin-tailwindcss
- **Auto-format**: Run `npm run format` before commits

### General

- **No comments**: Avoid adding comments unless absolutely necessary
- **Security**: Never expose secrets, validate all inputs
- **Dependencies**: Check package.json/Cargo.toml before adding new deps
