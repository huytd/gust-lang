# 04.29.2022 - A Working Lexer

Finally got some time to actually initialize some code for the compiler, I guess
I'll write up the syntax specs later, but for now, let's get the lexer up and running first.

I actually didn't plan to work on the lexer from the beginning but focus on the bytecode generator and the virtual machine part, but whatever, we need a working parser to better generate the bytecode, so here we are.

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

1. **Single-char:** are the tokens that only take 1 character, like brackets, mathematics operators,...
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

Keeping a source code's string slice in a token enum is very helpful to refer to that part of the source later on while avoiding allocating a new string on the heap every time we generate a new token. The downside is, that we have to pay close attention to the lifetime of the string slice.

Scanning tokens that have less than 2 characters are pretty easy, it can be done by iterating each character, and using [`Peekable<I>`](https://doc.rust-lang.org/stable/std/iter/struct.Peekable.html) to peek at the first character, if it matches with some patter, we consume it and continue looking at the next characters:

```rust
if let Some((_, c)) = self.chars.peek() {
    if let Some(token) = match c {
        '-' => Some(Token::Minus),
        '+' => Some(Token::Plus),
        '/' => Some(Token::Slash),
        '*' => Some(Token::Star),
        ':' => Some(Token::Colon),
        '\n' => Some(Token::EOL),
        ...
        _ => None,
    } {
        self.chars.next();
        return Some(token);
    }
}
```

For multiple character tokens, like keywords or identifiers, we will try to build the string slice from the first non-whitespace character to the last non-whitespace character, and compare this slice with the set of the expected keywords. Keep consuming as we build the string slice to move the iterator forward.

For example, here's how we scan for a number token:

```rust
if let Some((start, c)) = self.chars.next() {
    if let Some((_, c_next)) = self.chars.peek() {
        if let Some(token) = match c {
            _ => {
                ...
                if c.is_numeric() {
                    let mut end = start;
                    while let Some((next_end, c_next)) = self.chars.peek() {
                        if c_next.is_digit(10) || c_next == &'_' || c_next == &'.' {
                            end = *next_end;
                            self.chars.next();
                        } else {
                            break;
                        }
                    }
                    return Some(Token::Number(&self.source[start..=end]));
                }
                ...
                None
            }
        } {
            self.chars.next();
            return Some(token);
        }
    }
}
```

A number is a list of characters that starts with a numeric character, and all of the subsequence characters should be numeric too. Since in Gust, a number could be separated by an under-dash `_`, and for decimal numbers, there could be a dot `.` too, so, when scanning for the next character, we check with this condition:

```rust
if c_next.is_digit(10) || c_next == &'_' || c_next == &'.' {
    ...
}
```

---

**Update (10:25 AM):**

Actually, there was an error with the above code and I only spot it after adding some tests.

For the expression that ends with an identifier, like:

```
a + b - c
```

The last character would be `'c'`, and there is no characters come after that, the `peek()` call return a `None` value, the scanning function would not go any further:

```rust
if let Some((start, c)) = self.chars.next() {
    if let Some((_, c_next)) = self.chars.peek() {
        // ^ it's None here, so the code stop
```

To fix this, we can set the default value for the `peek()` as a null byte, and there should be no changes in the scanning algorithm:

```rust
if let Some((start, c)) = self.chars.next() {
    let (_, c_next) = self.chars.peek().unwrap_or(&(0, '\0'));
```
