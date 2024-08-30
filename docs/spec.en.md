# INTJ Specification Version 0

## -1. Preliminary

### Stack Notation

Since the language is stack-oriented,
we need to denote the stack state in the document.

- `[]` denotes an empty stack.
- `[1, 2, 3]` denotes a stack with elements `1`, `2`, and `3`.
    The top of the stack is the leftmost element.
- Ellipsis stands for some elements, like `[1, 2, ..., 5, ...]`.

---

## 0. What is INTJ?

**Intelligent JSON (INTJ)** is a simple language that accepts JSON.

### Objective

- INTJ is designed to be a simple and small language for general purpose with JSON, Regex, HTTP Fetch and etc.
  - More friendly for computer, not for human...
- Accept JSON as itself.
- Light binary with rust.
- Can run on web (with JS or wasm)

### Features

- INTJ is **stack-oriented**.
  - The program uses a main stack to store temporary values, arguments, and results.
  - The stack is shared among all functions and operations.
  - Most keywords and functions manipulate the stack, primarily using the push operation.

- INTJ is dynamically typed.
  - Each cell in the stack can hold any type of data.

- INTJ accepts JSON.
  - Grammar of INTJ is a superset of JSON and JSON5.
    You can directly copy-paste your JSON to INTJ.
    This will push the JSON object onto the stack.
    - However, the meaning of each symbol is quite different from JSON.

- Functions are first-class.
  - Functions are treated like any other value and can be pushed onto the stack.
  - They are essentially lists of commands.

- All bindings are immutable.
  - INTJ does not allow changing variable values, which makes programs more predictable and easier to debug.

---

## 1. Language Basics

### Comments

INTJ's comments is almost same as C/C++/Java/JavaScript.

```intj
// This is a line comment
/* This is a block comment */
/// This is a document comment.
```

Also, as other script languages, INTJ also allow shebang.

```intj
#!/usr/bin/env intj
```

### Evaluation Order

In INTJ, every code executed from top to down,
and **right to left**.
For example,

```intj
1 2 3
4 5 6
```

In the above code, the first line (`1 2 3`) is executed first, and then the second line (`4 5 6`) is executed.
Also, in each line, the rightmost element is executed first.
Thus, the execution order is `3 2 1 6 5 4`.

Someone might think that this is not intuitive,
but this is a deisgn to make the language looks like other common languages.

Most stack-oriented languages invoke functions after the arguments are pushed.
For example, in Forth, add two numbers and call 'print' can be written as

```forth
1 2 + print
```

This is weird compared to other languages, for example

```javascript
print(+(1, 2))
```

However, by reversing the order of the arguments, the code looks more like other languages.

```intj
print + 1 2
```

This syntax highly inspired by [Uiua](https://uiua.org).

### Line Separator

The newline character is a special symbol
to *cut* the evaluation chain.
However, someone wants to change execution order without
line changes.
In this case, you can also use `,` as a line separator.

```intj
1 2 3, 4 5 6
// This is equivalent to
1 2 3
4 5 6
```

Also, multiple newline & ',' mixing is allowed.

```intj
1 2,,,3,4 5 6,,

7
// The above code is equivalent to
1 2, 3, 4 5 6, 7
```

There are some cautions:

- Line comments does not consume the new line. In other words, because of the newline character after the comments, lines before the comments executed first, and then the next line.
- If block comments are placed over multiple lines, the parser determines separator after ignoring the block comments.
  For example,

  ```intj
  a b c   /* some long comments
  ends here */ d e f
  ```

  is equivalent to

  ```intj
  a b c d e f
  ```

  even the comments containing newlines.
- In the same way, even block of words (by parenthes, object, array) contains newlines in the middle, the parser considers them as a single line.

  For example,

  ```intj
  a b (
    c
  ) d
  ```

  is equivalent to `a b (c) d`, not `a b (c), d`.

### Literals

In a nutshell, all JSON literals are available in INTJ.
In addtion, INTJ allows more literal formats.

- Special values, `null`, `true`, `false`
  - Note that they are not reserved names,
    but just in standard library.
    You can use them as `std.null`, `std.true`, `std.false`.
- Numbers
  - `-12.345e-6`: The only number format JSON supports.
  - `0x1234`, `0o7624`: Hexadecimal and octal numbers.
- Strings
  - `"Hello, world!\n"`: The only string format JSON supports.
  - `'single quote\a\f\uAFFF'`: Single quotes, C escape sequences are allowed.

In code, all literals is a kind of command to push the value onto the stack.

```intj
42        // Push 42
"Hello!"  // Push "Hello, world!"
```

### Symbols

Symbols are a special type of literal looks like a string.
The easiest way to define a symbol is to use `:` after a string.

```intj
"hello":   // => Symbol 'hello'
'world':   // => Symbol 'world'
"good"  :  // => Spaces between the string and colon are allowed.

"good!"
   :       // Newline is also allowed, but don't do this!
```

Internally, a symbol is *just a kind of integer*,
and the only thing guaranteed is that
the integer value is equal if and only if the name is equal.
The symbol may be useful when you compare two short strings such as keys.

You can also convert an identifier to a symbol by adding `:`.

```intj
my-good-value:  // => Symbol 'my-good-value'
wow!ItWorks:    // => Symbol 'wow!ItWorks'
```

To convert a symbol to an integer, or vice versa, use standard functions.

### Arrays and Objects

To create arrays and objects, use `[]` and `{}`.
You can use them just like in JSON.
For example:

```intj
[1, 3, 5] // => [1, 3, 5]
{"a": 1, "b": 2} // => {a: 1, b: 2}
```

The important thing is that the only special syntax is about brackets.
*Commas, colons, and other characters are follows INTJ's behaviours.*
For example, on the above code, each literals just push literals onto the stack, and comma is just a separator.
In that sense, the following code is also valid:

```intj
[1
 3
 {,
      header: "test",,, body: "test"
 }]
```

and equivalent to `[1, 3, {header: "test", body: "test"}]`.

Because of the flexibility of INTJ, it also accepts JSON5 syntax.

I'll explain how this works after functions.

### Identifiers

Most words does not contains reserved characters
are valid identifiers, if they cannot be interpreted as a literal.
Unicode characters are also allowed.

The reserved characters are:

- Whitespaces (space, tab, newline, etc. From U+0000 to U+0020 in ASCII)
- Separator character `,`
- Symbol character `:`
- Brackets `()[]{}`
- Hash `#`
- Quotes `'`, `"`, ``` ` ```

Also, some special characters are considered as 'operator character'.

- `+`, `-`, `*`, `/`, `%`, `&`, `|`, `^`, `~`, `!`, `=`, `<`, `>`, `?`, `@`, `$`, `;`

They can be used as an identifier, but `=` is dealt in some special way.
I'll explain this later.

In addition, INTJ's identifier is case-insensitive.
Don't be confused with the strings and symbols, which are case-sensitive.

For example, the following are valid identifiers:

```intj
hello
WOrLd        // Same as 'world'
hello_world
good+morning // This is a single identifier!
3<->4        // This is also a single identifier!
안녕!        // Unicode characters are also allowed.
```

You may want to use some special characters in the identifier.
In this case, you can use backticks.
Between backticks, most characters are allowed except newline and backticks.
Also, unlike string literal, escape sequence is not allowed in backticks.

```intj
`hello, world!` // => Identifier 'hello, world!'
`안녕!`         // => Identifier '안녕!'
```

While literals are just pusing values onto the stack,
identifiers' behaviour will be differed by
what value is bound to the identifier.

- If the identifier is bound to a function,
  the function will be invoked.
- If the identifier is bound to a non-function value
  the value will be pushed onto the stack.

For example,

```intj
a = 42
b = [3, 2, 1]
f = (print "Hello")  // This is a function, not explained yet.

a  // It just pushes 42 onto the stack.
b  // Similar case.
f  // It invokes the function, and the function pushes "Hello" onto the stack.
```

### Bindings

You can bind a value to an identifier using `=`.

```intj
my_val = 42 // Bind 42 to 'my_val'
a= 3        // You can omit the space after the identifier, but not recommended.

// Bad cases:
// x
// = 42  // Newline is not allowed.
// y /* comment */ = boom
```

Note that `<ID> =` form is a kind of command.
The behaviour is to pop a value from the stack and bind it to the identifier.
Therefore, the RHS does not required.

```intj
"hello" 42  // Then, the stack will be ["hello", 42, ...]
a= b=
// First pop "hello" and bind to b
// Then pop 42 and bind to a
// Resulting in a=42, b="hello"
```

Because of the above assign form,
some operators looks like binding, for example `<=`, `>=`,
because they has suffix `=`.
Using backticks for operators with suffix `=` is too verbose,
thus INTJ considered identifiers finished with more than two operator characters not as binding.

```intj
a=        // This is a binding
a<=       // Since '<=' is a two-character operator, this is and identifier `a<=`.
**=       // This is also an identifier.
good<-to= // Since the last operator character is '=', this is a binding.
```

### Functions

Functions are just a group of codes.
You can define a function using `()`.

```intj
pop_and_double = (* 2 pop)
swap = (
    a = b = // The first line executes first, popping values to b and a
    a b     // Then pushes b and a
)

3 4 5 // => [3, 4, 5, ...]
swap  // => [4, 3, 5, ...]
pop_and_double // => [6, 5, ...]
```

Since each command invokes function,
you should wrap them with `()` to push the function without invoking.
Don't worry about the performance, since the compiler will optimize this.

```intj
t = pop_and_double  // This is a function invocation
t = (pop_and_double) // This is a function push
t = (42)  // It's almost same as t = 42, but a function.
```

Notes on functions:

- Unlike other common languages,
  INTJ does not have explicit function parameters.
  Instead, use bindings to takes arguments:

  ```intj
  f = (
    arg1=, arg2=, arg3=
    // Your code here
  )
  ```

- Unlike other common languages,
  INTJ does not provide return for every function.
  Instead, in standard library, user can create
  'break'.

### How Arrays and Object works?

Let's talk about how arrays and objects work.

In INTJ, arrays and objects brackets are
just a syntatic sugar.
It can replace with a function and some standard command:

```intj
std.array (1, 2, 3) // => Equivalent to [1, 2, 3]
```

As you can see,
the above code push a function,
which push values `[3, 2, 1]`,
then invoke the `std.array` function.

The magic is in the `std.array` function.
It takes a function that pushes values,
and create array with the value pushed by the function.

The `std.array` function can be written as follows
(not exactly, but similar):

```intj
array = (
  push_values= // This is an argument

  // Set the start marker into the stack
  // std.mark push a unique mark into the stack.
  std.mark

  // Keep the duplication in the local binding
  mark = dup

  // Push values.
  push_values

  arr = std.empty_array

  // Reduce the stack until the start marker
  std.until mark (v=
    std.push_array_front v arr
  )

  arr
)
```

Even that the construction is optimized by runtime, the internal logic is similar to the above code.
In other words, `array` just packs values of stack
in reverse order.

It implies, you can create an array with arbitrary commands,
such as arithmetic operations, string manipulation, etc.

```intj
[
    + 32 10 // This effectively pushes 42
    trim "  hello world " // This effectively pushes "hello world"
]
```

Objects works similarly.
If your function pushes key-value correctly,
`object` function just pack them into an object.
For example,

```intj
{
    "number": 42,
    value: 5
}
```

works as follows:

- First add a start marker.
- Evaluate the function.
  - The stack will be `[value:, 5, number:, 42, MARKER, ...]`
- Finally, convert the stack to an object, by pop 2 values until the marker.
  - The first key-value is `value:` and `5`.
  - The second key-value is `number:` and `42`.

If you're more comfortable with stack manipulation,
you can construct arrays and objects in more tricky
(but less practical) ways.
For example, using `swap` to reverse two values:

```intj
[ 1, 2, swap ] // => [2, 1]
{
    [ key1:, key2:, my-val: ]
    [ 42, true, null ]
    push_zip // This interleaves the two arrays on the stack.
} // => {key1: 42, key2: true, my-val: null}
```

---

## 2. Binding, Scope and Module

As mentioned before,
the only ways to keep or take values are

- Via stack.
- Via bindings.

Stack looks like a global single one shared among whole program.
And bindings are exists... where?
Let's talk about the scope and module system.

### 2-1. Scope and Stack

In INTJ runtime,
there are not only a single stack.
Futher **more than one kinds of** stack exists.

- **Value stack**:
  This is the stack we talked about.
  It's used for a single routine of execution.

- **Frame stack**:
  This is a kind of call stack.
  When you invoke a function,
  a new frame is created and pushed into the frame stack.
  And when you exit from the function,
  the frame will be popped.

Values not in the value stack, e.g. bindings, and some special values, e.g. instruction pointer, are stored in each frames in the frame stack.
In other words,
when you define a binding `a = 42`,
the runtime will put `42` into a cell named `a` in the current (i.e. top) frame.

Then, you may have one more question,
"Which frame global/top-level bindings are stored in?"

```intj
f = (
  a = 42 // This will be stored in the frame of f
)
g = "boom" // Where this will be stored?
```

### Module Scope

In INTJ, each file is considered as a 'module'.
And the whole code of the 'module'
is *a function*
which construct a bindings to be exported *in frame*.

For example, let's see the following intj file:

```intj
a = 42
str = "Hello, world!"
```

The above is just a module, with the module constructor `(a = 42, str = "Hello, world!")`.
When you invoke this *function*,
the frame will have bindings `a = 42` and `str = "Hello, world!"` at the end.

When you import the module, it'll prepare **empty value stack** and invoke the module constructor.
And when the constructor is done,
the frame will be popped and kept.
Whenever you access to one of the bindings in the module,
the runtime will search from the frame.

You may consider the module scope as a global scope.

### Import Module

To import a module, use `#` with parentheses.
In the parentheses, you can list the modules you want to import, with some alias if you want.

```intj
#(
    "my_module.intj" // Import my_module.intj, bind to 'my_module'
    mod2 = "my_module2.intj" // Import my_module2.intj, bind to 'mod2'
    "also" // You can skip the extension '.intj'
    sub = "subdir/sub_module.intj" // Import subdir/sub_module.intj, bind to 'sub'
    sub = "subdir/file2.intj" // You can overwrite the module by importing again.
    full_dir = "my_dir/" // Import all .intj files in my_dir/
    _ = "my_dir/overwirte.intj" // Can use without prefix.
)
```

After the import, you can access the module binding with a prefix. The default prefix is filename + `.`. If you put a name in the parentheses, you can use that name as a prefix.

For example, with the above import:

```intj
my_module.a // Access 'a' in my_module.intj
mod2.b // Access 'b' in my_module2.intj
sub.print // In this case, print in one of 'subdir/sub_module.intj' or 'subdir/file2.intj'
full_dir.some_fn // Access 'some_fn' in some .intj file in my_dir/
```

You can see there may be some ambiguity in the module name. `.intj` does not have a strict module system, so you need to be careful with the module name.

Import can be used in the middle of the code.
In this case, the module name is only available in the scope of function.

Note that the standard library is also a module, and its path is `""` (empty string). By default, the standard library is imported as follows for every code:

```intj
#(
    std = ""
    _ = ""
)
```

In other words, you can access the standard library with the prefix `std.`, or without a prefix.

### Notes about Module

- Modules are loaded only once.

  If you import the same module multiple times,
  the module code will be executed only once
  and reuse the same module instance.

- In most case, the imported module is executed before the importing module.

  Also, module code is executed exactly at
  the import statement `#(...)`.

### Cyclic Import and Unloaded Module

In my opinion,
most common cases cause import problems are cyclic import and unloaded module.

For example, consider the following code:

```intj
// A.intj
#( "B" )
f = B.f
g = ( B.g 42 )

// B.intj
#( "A" )
f = A.f
g = ( A.g 42 )
```

Suppose that you executed above codes with
`intj run A.intj`.
Then, the module `A`
will starts to load,
and the module `B` will be triggered to import.
However, the module `B` also tries to import `A`.
As mentioned before,
each module should be loaded only once.

To handle this situation corretly,
INTJ will mark the module as 'not loaded'
when it starts to be imported.
And mark it as 'loaded' when the module code is executed completely.
If the module is re-imported while it's in 'not loaded' state,
the import will be ignored.

In the above example,
in the module `B`,
the module `A` is in 'not loaded' state,
and ignoring the import.

If you try to access the 'not loaded' module's binding,
the runtime will raise a panic.
Fo example, in the above example,
module `B` access to `A.f`, and it will raise a panic.

This can prevent the cyclic import earlier,
but the rule is too strict for some cases.
So, INTJ loosens the rule a bit,
by allowing accessing to the 'not loaded' module
in the function which is not invoked during the loading time.

For example, in the above example, `B.g` is `(A.g 42)`, and `A.g` is accessed in the function.
Since `A.g` is not invoked during the loading time,
the runtime will not raise a panic.

### Local Scope and Closure

Each bindings in function will be stored in the
frame of the function.
However, in the function, not only the bindings
but also the enclosing bindings are also available.
This is because when you create a function in function,
the inner function captures required bindings.

For example:

```intj
f = (
  a = 42
  b = true
  g = (
    + a 3 // g uses a. Thus a should be captured.
  )
  g // this g is a function with captured a.
)
```

You may take it for granted,
because every global bindings (module bindings) are available in the function.
They are also captured by the function.
(Note: The real behaviour slightly differs
because of optimization.)

### Documentation Comments

Before the assignment, you can put a documentation comment, beginning with `///`. You can access the documentation not only in many tools but also at runtime. For example, the function `help` will show the documentation of some binding:

```intj
help "f" // Show the documentation of 'f'
help ["a", "b", "c"] // Show the documentation of binding "c" which in "b" which in "a"
```

This is implemented with some 'magic', using `std`'s reflect and runtime features.

Note that the documentation is bound to the last assignment of the next line. For example:

```intj
/// Hello!
1 2 3 a= b= 42 "boom"
```

In this case, the documentation is bound to `a`, not `b`.

---

## 3. Types

### Built-in Types

The built-in types in INTJ are:

- Literals
  - **null**: can be constructed by `std.null`.
  - **boolean**: `std.true` and `std.false`.
  - **number**: 64-bit floating point number.
  - **string**: UTF-8 string.
  - **symbol**: a kind of string mapped to an integer.
- Containers
  - **array**: a list of values.
  - **object**: a map of key-value pairs.
- Functions
  - **closure**: a function with captured bindings.
- Special
  - **marker**: a unique identifiable value for stack manipulation.

## A. Standard Library

## B. Standard IO Library, and Async

## C. System-Dependent Features

## X. Example Programs
