# Web Snake Game

## Description

Classic game of snake created to run with rust as a game process and rendered out using javascript with html canvas.

## Instructions

1. Compile Rust with web assembly pack in the root directory. This will create the pkg package.

```bash
$ wasm-pack build --target web
```

2. Go to the www/ directory and start the server.

```bash
$ cd www/
```

```bash
$ npm run dev
```

## Rust

Rust is a multi-paradigm, general-purpose programming language designed for performance and safety. Rust is a popular low level language that is gaining a lot of popularity and now is able to be integrated into the web using webassembly.
<br></br>
All the logic program is in the src/ directory

## Javascript

All the HTML/CSS and javascript source code is under the www/ directory.
