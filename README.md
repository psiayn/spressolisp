# spressolisp
lisp in rust


## TODO
- [x] functions ( lambdas )
- [x] debug env? ( .debug )
- [X] conditionals
- [x] relational ops
- [x] logical ops
- [x] strings
- [x] revamp tokenizer and parser
    - [x] strings with spaces in them
    - [x] track line and col num for each token
    - [x] keep track of token(s) for each expr
- [x] show context for errors - mark the exact token which caused an error
- [x] loops
- [x] lists
  - [x] map
  - [x] append
- [ ] cleanup code
  - [ ] make env more functional
- [x] tests for floats
- [x] tests for conditional
- [x] tests for lambdas
- [x] tests for strings
- [x] unit type -> return this instead of `false` when there's no return value. Like `()` in Rust.
- [ ] recursion
- [ ] macros
- [x] refactor tests to reduce if else ladder
- [ ] functional programming stuff
  - [ ] map
  - [ ] reduce
  - [ ] filters
  - etc...?
