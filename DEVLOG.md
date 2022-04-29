#04.29.2022 - Add String support to the Lexer

With the unit tests up and running, it's time to start expanding the Lexer: Adding String support.

A string is a sequence of any characters that stay between a pair of quotation characters. Just like in JavaScript, either a double quote or a single quote can be used to define a string.

```javascript
"this is a string"
'this is also a string'
```

So, the first thing to do is to add a couple of tests, both the valid and invalid strings:

```rust
#[test]
fn lexer_string_test(){
    let lexer = new Lexer::new(r#""hello world""#);
    let actual = lexer.collect::<Vec<Token>>();
    assert!(actual == vec![
        Token::String(r#""hello world""#)
    ])
}
```

An invalid string should return a `Token::Invalid` token:

```rust
#[test]
fn lexer_invalid_string_test(){
    let lexer = new Lexer::new("'hello);
    let actual = lexer.collect::<Vec<Token>>();
    assert!(actual == vec![Token::Invalid])
}
```

The algorithm to scan for a string is pretty much like what we did for all the other multiple-char tokens:

1. Start with a quotation mark
2. If the next character is not a matching quotation mark, consume that character
3. If we found the matching quotation mark, record the position, return a `Token::String` token with the string slice from the start to the current position

Initially, I implemented the scanner like this:

```rust
let token = match c {
    ...
    quote @ ('"' | '\'') => {
        let mut end = start;
        while let Some((next_end, next_char)) = self.chars.next() {
            if quote == next_char {
                end = next_end; break;
            }
        }
        let content = &self.source[start..=end];
        return Some(Token::String(content));
    }
}
```

This implementation worked fine until I decided to test with some Unicode strings. The Lexer crashed with the **"byte index is not a char boundary"** message.

The reason is, that we're getting the string content slice using the byte indices `&self.source[start..=end]`, but both the `start` and `end` indices are the character index that came from `self.chars` iterator.

So, what we need to do here is access the character list of `self.source` and return a slice from the range of `start..=end`. To do this, we can use `char_indices()` iterator. There is an `utf8_slice` crate that does this, and the implementation is available at: [crate `utf8_slice`: src/utf8_slice/lib.rs.html#52-70]()https://docs.rs/utf8_slice/latest/src/utf8_slice/lib.rs.html#52-70).

All we have to do when fetching the string content is:

```rust
let content = fetch_string_slice(self.source, start, end);
return Some(Token::String(content));
```

Actually, with this implementation, I can make Gust supports Unicode identifiers (variable names, function names,...) as well!

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

The last character would be `'c'`, and there is no characters come after that, the `peek()` call return a `None` value, the scanning
function would not go anyfurther:

```rust
if let Some((start, c)) = self.chars.next() {
    if let Some((_, c_next)) = self.chars.peek() {
        // ^ it's None here, so the code stop
```

To fix this, we can set the default value for the `peek()` as a null byte, and there should be no
changes in the scanning algorithm:

```rust
if let Some((start, c)) = self.chars.next() {
    let (_, c_next) = self.chars.peek().unwrap_or(&(0, '\0'));
```
