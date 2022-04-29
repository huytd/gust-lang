# 04.29.2022 - A Working Lexer

Finally got sometime to actually initialize some code for the compiler, I guess
I'll write up the syntax specs later, but for now, let's get the lexer up and running
first.

A `gust` program will look like this:

```rust
let x = 10
let y = x
let z = x + 5
if y >= x && z != 10 {
    let hello_world = 100
    print(hello_world)
}
```

For now, a few tokens has been defined, it's pretty much the same as the tokens
that come from the book [Crafting Interpreters](https://www.craftinginterpreters.com/scanning.html).

They can be divided into four groups:

1. **Single-char:** are the tokens that only takes 1 character, like brackets, mathematics operators,...
2. **Comparison:** are the tokens for the comparison operators like `<`, `<=`, `>`, `>=`,...
3. **Keyword:** are the tokens for the built-in keywords like `let`, `for`, `if`,...
4. **Multiple-char:** are the tokens for the identifiers and numbers, strings,...

To use the multiple-char tokens, we need to keep a reference to the original source code, this is why they're defined with a string slice:

```rust
enum Token<'a> {
    ...
    Identifier(&'a str),
    String(&'a str),
    Number(&'a str),
    ...
}
```

Scanning tokens that less than 2 characters are pretty easy, it can be done by iterating each character, and use [`Peekable<I>`](https://doc.rust-lang.org/stable/std/iter/struct.Peekable.html) to look ahead one more character without consuming the iterator.

For multiple characters tokens, like keywords or identifiers, we will try to build the string slice from the first non-whitespace character, to the last non-whitespace character, and compare this slice with the set of the expected keywords. Keep consuming as we build the string slice to move the iterator forward.
