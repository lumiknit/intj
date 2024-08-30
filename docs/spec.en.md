# INTJ Specification Version 0
## -1. Preliminary

### Stack Notation

Since the language is stack-oriented, it is important to denote the stack state in the document.

- `[]` denotes an empty stack.
- `[1, 2, 3]` denotes a stack with elements `1`, `2`, and `3`, where the top of the stack is the leftmost element.
- Ellipsis (...) represents some elements, for example, `[1, 2, ..., 5, ...]`.

---

## 0. What is INTJ?

**Intelligent JSON (INTJ)** is a simple language that accepts JSON.

### Objective

- INTJ is designed to be a lightweight and efficient language for general-purpose programming with JSON, Regex, HTTP Fetch, and more.
  - It prioritizes computer-friendliness over human-friendliness.
- INTJ natively supports JSON as a data format.
- It is implemented in Rust, resulting in a compact binary.
- INTJ can be run on the web using JavaScript or WebAssembly (wasm).

### Features

- INTJ is a **stack-oriented** language.
  - The main stack is used to store temporary values, arguments, and results.
  - The stack is shared among all functions and operations.
  - Most keywords and functions manipulate the stack by pushing and popping values.

- INTJ is dynamically typed.
  - Each cell in the stack can hold any type of data.

- INTJ accepts JSON syntax.
  - The grammar of INTJ is a superset of JSON and JSON5.
  - You can directly copy and paste JSON code into INTJ, which will push the JSON object onto the stack.
    - However, the meaning of each symbol may differ from JSON.

- Functions are first-class citizens.
  - Functions can be treated as values and pushed onto the stack.
  - They are essentially lists of commands.

- All bindings are immutable.
  - INTJ does not allow changing the values of variables, which enhances program predictability and simplifies debugging.

---

## 1. Language Basics

### Comments

INTJ's comments are similar to those in C/C++/Java/JavaScript.

```intj
// This is a line comment
/* This is a block comment */
/// This is a documentation comment.
```

Additionally, like other scripting languages, INTJ also allows the use of a shebang.

```intj
#!/usr/bin/env intj
```

### Evaluation Order

In INTJ, code is executed from top to bottom and **from right to left**.
For example:

```intj
1 2 3
4 5 6
```

In the above code, the first line (`1 2 3`) is executed first, followed by the second line (`4 5 6`).
Within each line, the rightmost element is executed first.
Therefore, the execution order is `3 2 1 6 5 4`.

This execution order may seem unintuitive to some, but it is designed to make the language resemble other common programming languages.

In most stack-oriented languages, functions are invoked after the arguments are pushed onto the stack.
For example, in Forth, adding two numbers and calling the 'print' function would be written as:

```forth
1 2 + print
```

This syntax may seem unusual compared to other languages, such as:

```javascript
print(1 + 2)
```

However, by reversing the order of the arguments, the code in INTJ looks more similar to other programming languages:

```intj
print + 1 2
```

This syntax is highly inspired by [Uiua](https://uiua.org).

### Line Separator

The newline character is a special symbol used to change evaluation order. However, there may be cases where someone wants to change the execution order without adding a new line. In such cases, you can use the comma (`,`) as a line separator.

For example, instead of writing:

```intj
1 2 3
4 5 6
```

You can write:

```intj
1 2 3, 4 5 6
```

This is equivalent to the previous code, but without the new line.

You can also mix multiple newlines and commas as separators. For example:

```intj
1 2,,,3,4 5 6,,

7
```

The above code is equivalent to:

```intj
1 2, 3, 4 5 6, 7
```

Here are some important points to keep in mind:

- Line comments do not consume the new line. This means that the line comment is treated as a separator too.
- If block comments span multiple lines, the parser will ignore the comments when determining the line separators.
  For example:

  ```intj
  a b c   /* some long comments
  ends here */ d e f
  ```

  is equivalent to:

  ```intj
  a b c d e f
  ```

  even if the comments contain newlines.

- Similarly, if a block of code (enclosed in parentheses, objects, or arrays) contains newlines, the parser will treat it as a single line.

  For example:

  ```intj
  a b (
    c
  ) d
  ```

  is equivalent to `a b (c) d`, not `a b (c), d`.

### Literals

In INTJ, all JSON literals are available, and INTJ also allows additional literal formats.

- Special values: `null`, `true`, `false`
  - Note that these values are not reserved keywords, but rather part of the standard library. You can access them as `std.null`, `std.true`, and `std.false`.

- Numbers
  - INTJ supports the same number format as JSON, such as `-12.345e-6`.
  - Additionally, INTJ allows hexadecimal and octal numbers, such as `0x1234` and `0o7624`.

- Strings
  - INTJ supports the same string format as JSON, such as `"Hello, world!\n"`.
  - INTJ also allows the use of single quotes and C escape sequences in strings, such as `'single quote\a\f\uAFFF'`.

In INTJ code, all literals are commands that push their corresponding values onto the stack.

Examples:

```intj
42        // Pushes the number 42 onto the stack
"Hello!"  // Pushes the string "Hello, world!" onto the stack
```

### Symbols

Symbols are a special type of literal that looks like a string. The easiest way to define a symbol is to use a colon (`:`) after a string.

```intj
"hello":   // => Symbol 'hello'
'world':   // => Symbol 'world'
"good"  :  // => Spaces between the string and colon are allowed.

"good!"
  :       // Newline is also allowed, but it's not recommended.
```

Internally, a symbol is just a kind of integer, and the only guarantee is that the integer value is equal if and only if the name is equal. Symbols can be useful when comparing two short strings, such as keys.

You can also convert an identifier to a symbol by adding a colon (`:`).

```intj
my-good-value:  // => Symbol 'my-good-value'
wow!ItWorks:    // => Symbol 'wow!ItWorks'
```

To convert a symbol to an integer, or vice versa, you can use standard functions.

### Arrays and Objects

To create arrays and objects, use `[]` and `{}`. You can use them just like in JSON. For example:

```intj
[1, 3, 5] // => [1, 3, 5]
{"a": 1, "b": 2} // => {a: 1, b: 2}
```

The important thing to note is that the only special syntax is the use of brackets. Commas, colons, and other characters follow INTJ's behavior. For example, in the code above, each literal simply pushes the literal value onto the stack, and the comma is just a separator. Therefore, the following code is also valid:

```intj
[1
 3
 {,
  header: "test",,, body: "test"
 }]
```

This code is equivalent to `[1, 3, {header: "test", body: "test"}]`.

Due to the flexibility of INTJ, it also accepts JSON5 syntax.

I will explain how this works after covering functions.

### Identifiers

Most words that do not contain reserved characters are valid identifiers, as long as they cannot be interpreted as a literal. Unicode characters are also allowed.

The reserved characters are:

- Whitespaces (space, tab, newline, etc. from U+0000 to U+0020 in ASCII)
- Separator character `,`
- Symbol character `:`
- Brackets `()[]{}`
- Hash `#`
- Quotes `'`, `"`, ``` ` ```

Additionally, some special characters are considered 'operator characters'.

- `+`, `-`, `*`, `/`, `%`, `&`, `|`, `^`, `~`, `!`, `=`, `<`, `>`, `?`, `@`, `$`, `;`

These characters can be used as identifiers, but the `=` character is treated in a special way. I will explain this later.

Furthermore, identifiers in INTJ are case-insensitive. However, please note that strings and symbols are case-sensitive.

For example, the following are valid identifiers:

```intj
hello
WOrLd        // Same as 'world'
hello_world
good+morning // This is a single identifier!
3<->4        // This is also a single identifier!
안녕!        // Unicode characters are also allowed.
```

If you want to use special characters in an identifier, you can enclose the identifier in backticks. Backticks allow most characters except newline and backticks themselves. Unlike string literals, escape sequences are not allowed in backticks.

```intj
`hello, world!` // => Identifier 'hello, world!'
`안녕!`         // => Identifier '안녕!'
```

While literals simply push values onto the stack, the behavior of identifiers depends on what value is bound to them.

- If an identifier is bound to a function, the function will be invoked.
- If an identifier is bound to a non-function value, the value will be pushed onto the stack.

For example:

```intj
a = 42
b = [3, 2, 1]
f = (print "Hello")  // This is a function, which will be explained later.

a  // It simply pushes 42 onto the stack.
b  // Similar case.
f  // It invokes the function, which pushes "Hello" onto the stack.
```

### Bindings

You can bind a value to an identifier using the `=` operator.

```intj
my_val = 42 // Bind 42 to 'my_val'
a= 3       // You can omit the space after the identifier, but it is not recommended.

// Bad cases:
// x
// = 42  // Newline is not allowed.
// y /* comment */ = boom
```

Note that the `<ID> =` form is a command that pops a value from the stack and binds it to the identifier. Therefore, the right-hand side (RHS) is not required.

```intj
"hello" 42  // The stack will be ["hello", 42, ...]
a= b=       // First, "hello" is popped and bound to b. Then, 42 is popped and bound to a. Resulting in a=42, b="hello"
```

Because of the assignment form, some operators may look like bindings, for example, `<=`, `>=`, because they have a suffix `=`. Using backticks for operators with a suffix `=` is too verbose, so INTJ considers identifiers that end with more than two operator characters as not bindings.

```intj
a=           // This is a binding
a<=          // Since '<=' is a two-character operator, this is an identifier `a<=`.
**=          // This is also an identifier.
good<-to=    // Since the last operator character is '=', this is a binding.
```

### Functions

Functions in INTJ are simply a group of code. You can define a function using `()`.

```intj
pop_and_double = (* 2 pop)
swap = (
  a= b=   // The first line executes first, popping values to b and a
  a b     // Then pushes b and a
)

3 4 5 // => [3, 4, 5, ...]
swap  // => [4, 3, 5, ...]
pop_and_double // => [6, 5, ...]
```

Since each command invokes a function, you should wrap them with `()` to push the function without invoking it. Don't worry about performance, as the compiler will optimize this.

```intj
t = pop_and_double   // This is a function invocation
t = (pop_and_double) // This is a function push
t = (42)  // It's almost the same as t = 42, but as a function.
```

Notes on functions:

- Unlike other common languages, INTJ does not have explicit function parameters. Instead, use bindings to take arguments:

  ```intj
  f = (
  arg1 =, arg2 =, arg3 =
  // Your code here
  )
  ```

- Unlike other common languages, INTJ does not provide a return statement for every function. Instead, in the standard library, users can create a 'break' to exit a function.

### How Arrays and Objects Work

Let's talk about how arrays and objects work in INTJ.

In INTJ, the brackets used for arrays and objects are just syntactic sugar. They can be replaced with a function and some standard commands:

```intj
std.array (1, 2, 3) // Equivalent to [1, 2, 3]
```

As you can see, the above code pushes a function that pushes the values `[3, 2, 1]`, and then invokes the `std.array` function.

The magic happens in the `std.array` function. It takes a function that pushes values and creates an array with the values pushed by the function.

The `std.array` function can be implemented as follows (not exactly, but similar):

```intj
array = (
  push_values = // This is an argument

  // Set the start marker on the stack
  // std.mark pushes a unique mark onto the stack.
  std.mark

  // Keep the duplication in the local binding
  mark = dup

  // Push values
  push_values

  arr = std.empty_array

  // Reduce the stack until the start marker
  std.until mark (v =
  std.push_array_front v arr
  )

  arr
)
```

Even though the construction is optimized by the runtime, the internal logic is similar to the above code. In other words, `array` simply packs the values on the stack in reverse order.

This implies that you can create an array with arbitrary commands, such as arithmetic operations, string manipulation, etc.

```intj
[
  + 32 10 // This effectively pushes 42
  trim "  hello world " // This effectively pushes "hello world"
]
```

Objects work similarly. If your function pushes key-value pairs correctly, the `object` function will pack them into an object. For example:

```intj
{
  "number": 42,
  value: 5
}
```

works as follows:

- First, add a start marker to the stack.
- Evaluate the function.
  - The stack will be `[value:, 5, number:, 42, MARKER, ...]`
- Finally, convert the stack to an object by popping 2 values until the marker.
  - The first key-value pair is `value:` and `5`.
  - The second key-value pair is `number:` and `42`.

If you are more comfortable with stack manipulation, you can construct arrays and objects in more complex (but less practical) ways. For example, using `swap` to reverse two values:

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

As mentioned before, there are two ways to store or retrieve values in INTJ:

1. Stack: The stack is a global, shared data structure used to store temporary values, arguments, and results. It is accessible throughout the entire program.

2. Bindings: Bindings are stored in the scope of a function or module. Each function or module has its own set of bindings, which can be accessed within that scope.

Let's discuss the scope and module system in more detail.

### Scope and Stack

In the INTJ runtime, there are multiple kinds of stacks, not just a single stack.

- **Value stack**: This is the stack we discussed earlier. It is used for a single routine of execution.

- **Frame stack**: This is a type of call stack. When you invoke a function, a new frame is created and pushed onto the frame stack. When you exit from the function, the frame is popped.

Values that are not on the value stack, such as bindings and some special values like the instruction pointer, are stored in each frame on the frame stack. In other words, when you define a binding like `a = 42`, the runtime will store `42` in a cell named `a` in the current (i.e., top) frame.

Now, you may have another question: "In which frame are global/top-level bindings stored?"

```intj
f = (
  a = 42 // This will be stored in the frame of f
)
g = "boom" // Where will this be stored?
```

### Module Scope

In INTJ, each file is considered a 'module'.
The entire code within a module is treated as a *function* that constructs the bindings to be exported *within the module's frame*.

For example, let's consider the following INTJ file:

```intj
a = 42
str = "Hello, world!"
```

The code above represents a module with a module constructor `(a = 42, str = "Hello, world!")`.
When you invoke this *function*,
the module's frame will contain the bindings `a = 42` and `str = "Hello, world!"` upon completion.

When you import the module, an **empty value stack** is prepared, and the module constructor is invoked.
Once the constructor finishes executing,
the frame is popped and retained.
Whenever you access any of the bindings within the module,
the runtime will search for them within the module's frame.

You can think of the module scope as a global scope.

### Import Module

To import a module, use `#` followed by parentheses. Inside the parentheses, you can list the modules you want to import, and optionally assign them an alias.

```intj
#(
  "my_module.intj" // Import my_module.intj and bind it to 'my_module'
  mod2 = "my_module2.intj" // Import my_module2.intj and bind it to 'mod2'
  "also" // You can skip the file extension '.intj'
  sub = "subdir/sub_module.intj" // Import subdir/sub_module.intj and bind it to 'sub'
  sub = "subdir/file2.intj" // You can overwrite the module by importing it again
  full_dir = "my_dir/" // Import all .intj files in my_dir/
  _ = "my_dir/overwirte.intj" // You can import a module without assigning it a prefix
)
```

After importing a module, you can access its bindings using a prefix. By default, the prefix is the filename followed by a dot (`.`). If you specify a name in the parentheses during import, you can use that name as the prefix.

For example, with the above import:

```intj
my_module.a // Access the binding 'a' in my_module.intj
mod2.b // Access the binding 'b' in my_module2.intj
sub.print // Access the 'print' binding in either 'subdir/sub_module.intj' or 'subdir/file2.intj'
full_dir.some_fn // Access the 'some_fn' binding in any .intj file in my_dir/
```

Please note that there may be ambiguity in module names. Since `.intj` does not have a strict module system, it is important to be careful with module naming.

Imports can be used anywhere in the code. In such cases, the module name is only available within the scope of the function.

It is worth mentioning that the standard library is also a module, and its path is an empty string (`""`). By default, the standard library is imported as follows in every code:

```intj
#(
  std = ""
  _ = ""
)
```

This means you can access the standard library with the prefix `std.`, or without a prefix.

### Modules are loaded only once

When you import the same module multiple times, the module code will be executed only once, and the same module instance will be reused.

In most cases, the imported module is executed before the importing module. The module code is executed exactly at the import statement `#(...)`.

### Cyclic Import and Unloaded Module

In my opinion, the most common causes of import problems are cyclic imports and unloaded modules.

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

Suppose you execute the above codes with `intj run A.intj`. In this case, the module `A` will start to load, triggering the import of module `B`. However, module `B` also tries to import module `A`. As mentioned before, each module should only be executed once.

To handle this situation correctly, INTJ marks the module as 'not loaded' when it starts to be imported. It then marks it as 'loaded' when the module code is executed completely. If the module is re-imported while it is in the 'not loaded' state, the import will be ignored.

In the example provided, when module `B` tries to import module `A`, it is in a 'not loaded' state and the import is ignored. If you attempt to access a binding from a 'not loaded' module, the runtime will raise a panic. For instance, in the given example, module `B` tries to access `A.f`, which would result in a panic.

To prevent cyclic imports, INTJ has a strict rule that prohibits accessing bindings from 'not loaded' modules. However, this rule is relaxed in certain cases. You are allowed to access bindings from a 'not loaded' module within a function that is not invoked during the loading process.

In the example, `B.g` is defined as `(A.g 42)`, and `A.g` is accessed within the function. Since `A.g` is not invoked during the loading process, the runtime will not raise a panic in this case.

### Local Scope and Closure

Each binding in a function is stored in the frame of that function. However, in a function, not only the local bindings but also the enclosing bindings are available. This is because when you create a function within a function, the inner function captures the required bindings.

For example:

```intj
f = (
  a = 42
  b = true
  g = (
    + a 3 // g uses a, so a should be captured.
  )
  g // This g is a function with the captured value of a.
)
```

You may take this for granted because every global binding (module binding) is available in the function. They are also captured by the function. (Note: The actual behavior may slightly differ due to optimization.)

### Documentation Comments

Before the assignment, you can add a documentation comment using `///`. This comment can be accessed not only in various tools but also at runtime. For example, the `help` function can be used to display the documentation of a specific binding:

```intj
help "f" // Show the documentation of 'f'
help ["a", "b", "c"] // Show the documentation of the binding "c" within "b" within "a"
```

This functionality is implemented using `std`'s reflect and runtime features.

It's important to note that the documentation comment is associated with the last assignment on the following line. For example:

```intj
/// Hello!
1 2 3 a= b= 42 "boom"
```

In this case, the documentation comment is associated with the binding `a`, not `b`.

---

## 3. Types

### Built-in Types

The built-in types in INTJ are:

- Literals
  - **null**: can be constructed using `std.null`.
  - **boolean**: represented by `std.true` and `std.false`.
  - **number**: a 64-bit floating point number.
  - **string**: a UTF-8 string.
  - **symbol**: a special kind of string that is mapped to an integer.
- Containers
  - **array**: a collection of values.
  - **object**: a collection of key-value pairs.
- Functions
  - **closure**: a function that captures bindings from its enclosing scope.
- Special
  - **marker**: a unique and identifiable value used for stack manipulation.

---

## 4. Coroutine and Asyncronous Execution

WIP

## A. Standard Library

WIP

## B. System-Dependent Features

WIP

## X. Example Programs

WIP
