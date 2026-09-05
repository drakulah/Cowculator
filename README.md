# Cow

This is a runtime for Cow language.

!!! This project is incomplete !!!

## Example

An arithmetic calculation example in Cow.

```bash
$ ./cow "1 + 1 - 2 * 3 / 2 - 6 + 2 + 1 % 9 * PI - (12 + 4)"        
-20 # Result

$ ./cow "2 + 5 - 5 * PI - (12 + 4) - 6 + 2 + 1 % 9 + 7 * 8 % 6"
-25.707963267948966 # Result

$ ./cow "(2 + 5 - 5 * PI - (12 + 4) - 6 + 2 + 1 % 9 + 7 * 8 % 6).floor()"
-26 # Result

$ ./cow "143 * 642 # 32"
1:10: lexer err: unknown token `#`
    |
  1 | 143 * 642 # 32
    |           ~

$ ./cow "(2 + 5 - 5 * PI - (12 + 4) - 6 + 2 + 1 % 9 + 7 * 8 % 6).floor()) " 
1:63: lexer err: unexpected token `)`
    |
  1 | … - (12 + 4) - 6 + 2 + 1 % 9 + 7 * 8 % 6).floor())
    |                                                  ~
```

Now in `main.rs` add line:

```rs
eval.set_constant("DRAKULAH", 7.0);
```

then build the executable file.

```bash
$ ./cow "DRAKULAH + 3"
10 # Result
```
