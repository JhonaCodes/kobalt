# Kobalt 🎨

> **Experimental Project** 🧪
> Just a hobby project to learn WGPU and explore UI framework development!

Hey there! This is my personal experiment building a cross-platform UI framework using **WGPU**. It's mainly for learning, having fun, and seeing what's possible. Don't expect production-ready code here... yet? Maybe never? Who knows! 😄

## What is this?

Kobalt is a **mobile-first UI framework** designed for mobile developers who want to use Rust. Think Jetpack Compose or Flutter, but powered by Rust and WGPU.

The goal is to make it **mobile developer friendly** by using familiar patterns and concepts:
- 🎯 **Declarative UI** (like Compose/SwiftUI)
- 🔄 **Modern state management** (LiveData, MVVM, reactive patterns)
- 📱 **Mobile-first approach** with desktop and web as bonuses
- ⚡ **Native performance** through WGPU

### Target Platforms
- **Mobile**: iOS, Android (the main focus!)
- **Desktop**: Windows, macOS, Linux
- **Web**: WebAssembly

## Why?

I've been developing mobile applications for several years, and I've been using Rust for about a year now. I'm convinced that Rust could bring a lot to cross-platform development, especially for mobile. So I thought, why not try to create a UI framework? It could be a good starting point, or in the best case scenario, make it into something truly usable.

**Why doesn't anyone think about us?** 😭 Mobile developers who love Rust and want familiar patterns like Compose or Flutter, but with all the benefits of Rust. We exist! There are dozens of us! DOZENS!

**If you dream of seeing Rust powering mobile UIs, welcome!** This is going to be fun... and painful. Probably more painful than fun at first, but hey, that's part of the journey! 😅

Let's see where this experiment takes us.

## Current Status

Very early stages. Mostly just setting up the project structure and learning as I go. This is more of a learning journal than a production framework.

**What's working (kinda):**
- ✅ Desktop windows (via winit)
- ✅ Basic WGPU rendering
- ✅ Text rendering (with fontdue, a bit rough around the edges 😅)
- ✅ Column layout with alignment and padding
- ✅ Declarative macro syntax (app!, column!, text!)
- ✅ Basic color and styling

**What's not working (yet):**
- ❌ Mobile (iOS/Android) - the whole point of this project! 😭
- ❌ Most widgets (buttons, inputs, images, etc.)
- ❌ State management
- ❌ Navigation
- ❌ Gestures/touch
- ❌ Pretty much everything else

## Quick Peek 👀

> ⚠️ **Current Reality Check**: Right now, this only works on **Desktop** (Windows, macOS, Linux) using `winit`. Mobile support is the dream, but we're starting with desktop to get the foundations right! Baby steps! 🚶‍♂️
>
> ⚠️ **Syntax Disclaimer**: This is a *rough*, early prototype of what the syntax might look like. It's super basic and will probably change 100 times before stabilizing (if it ever does!). But hey, it gives you an idea of the direction!

Here's how you'd build a simple **Desktop** UI with Kobalt's macro-based API (inspired by Flutter/Compose):

```rust
use kobalt::prelude::*;
use kobalt_macros::column;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    app! {
        title: text!("My First Kobalt App"),
        size: (800, 600),
        background: Color::from_rgb8(20, 20, 30),
        home: column! {
            main_axis_alignment: MainAxisAlignment::Center,
            cross_axis_alignment: CrossAxisAlignment::Center,
            padding: EdgeInsets::all(20.0),
            children: [
                text!("Hello, Kobalt! 👋", size: 36.0, color: Color::WHITE),
                text!("Built with Rust + WGPU", size: 24.0),
                text!("Mobile-first UI framework")
            ]
        }
    }
}
```

**What you're seeing:**
- 🎯 **Declarative macros** - `app!`, `column!`, `text!` for a clean, Flutter-like syntax
- 📐 **Layout properties** - `main_axis_alignment`, `cross_axis_alignment`, `padding` (just like Compose!)
- 🎨 **Inline styling** - Direct property setting like `size:` and `color:`

This is just a taste! The API is evolving, but the goal is to make it feel natural for developers coming from Compose or Flutter.

## Goals (Maybe?)

### Core Framework
- [ ] Declarative UI API (Compose/SwiftUI-like syntax)
- [ ] Reactive state management (LiveData-inspired)
- [ ] MVVM architecture support
- [ ] Layout system (Flexbox/Column/Row)
- [ ] Common widgets (Button, Text, Image, List, etc.)
- [ ] Touch/gesture handling

### Mobile-Specific
- [ ] Native iOS integration
- [ ] Native Android integration
- [ ] Platform-specific components
- [ ] Navigation patterns familiar to mobile devs
- [ ] Hot reload (the dream!)

### Nice to Have
- [ ] Desktop and web support
- [ ] Theming system
- [ ] Animation API
- [ ] Document everything for mobile developers coming from Compose/Flutter

## Can I use this?

You can look at it, learn from it, or laugh at my code! But seriously, this is super experimental and changes constantly. Not recommended for anything serious.

## License

MIT - Do whatever you want with it!

---

**Note**: This is a hobby project developed for educational purposes. Expect bugs, breaking changes, and long periods of inactivity when life gets busy!
