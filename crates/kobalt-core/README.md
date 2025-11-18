# Kobalt Core

Core functionality for the Kobalt UI framework.

## Overview

`kobalt-core` is the heart of the Kobalt framework. It provides all the fundamental building blocks for creating declarative UIs without depending on any specific rendering backend or platform runtime.

## Features

- **Widget Tree**: Composable widget system for building UIs
- **Layout System**: Flexbox-inspired layout engine (Column, Row)
- **State Management**: Reactive state with LiveData, ValueNotifier, and ChangeNotifier
- **MVVM Infrastructure**: ViewModel patterns for separating business logic
- **Common Types**: Primitive types like Size, Rect, Color

## Architecture

This crate is:
- ✅ Platform-agnostic
- ✅ No WGPU dependency
- ✅ Testable with `cargo test`
- ✅ Portable to any platform

## Module Structure

```
kobalt-core/
├── types.rs          # Common types (Size, Rect, Color)
├── state/            # State management system
│   ├── live_data.rs
│   ├── value_notifier.rs
│   └── change_notifier.rs
├── widget.rs         # Widget tree and composition
├── layout/           # Layout system
│   ├── column.rs
│   ├── row.rs
│   └── flex.rs
└── mvvm/             # MVVM infrastructure
    └── view_model.rs
```

## Usage

```rust
use kobalt_core::prelude::*;

// Example usage will be added as implementation progresses
```

## Status

🚧 **Under Construction** - This crate is in early development.

See the main [Kobalt repository](../../README.md) for the full roadmap and architecture.
