# oxide:

**Grammar in EBNF Notation**

Brackets denote non-terminals, all-caps denote tokens/terminals
```
Program     ::= INIT <StmtList> HALT
StmtList    ::= <Stmt>; { <Stmt>; }
Stmt        ::= <DeclStmt> | <CtrlStmt>
DeclStmt    ::= (INT | FLOAT | BOOL | STRING) IDENT { , IDENT }
CtrlStmt    ::= <AssignStmt> | <PrintStmt> | <IfStmt> | <LoopStmt>
AssignStmt  ::= IDENT = <OrExpr>
PrintStmt   ::= PRINT( <Expr> { , <Expr> } )
IfStmt      ::= IF ( <OrExpr> ) THEN <StmtList> [ ELSE <StmtList> ] ENDIF
LoopStmt    ::= WHILE ( <OrExpr> ) DO <StmtList> ENDWHILE
OrExpr      ::= <AndExpr> { || <AndExpr> }
AndExpr     ::= <EqualExpr> { && <EqualExpr> }
EqualExpr   ::= <RelExpr> [ == <RelExpr> ]
RelExpr     ::= <AddExpr> [ (< | >) <AddExpr> ]
AddExpr     ::= <MultExpr> { (+ | -) <MultExpr> }
MultExpr    ::= <UnaryExpr> { (* | / | %) <UnaryExpr> }
UnaryExpr   ::= (- | !) <PrimaryExpr> | <PrimaryExpr>
PrimaryExpr ::= IDENT | ICONST | FCONST | BCONST | SCONST | ( <OrExpr> )
```

**Example Program**

```
init
    bool flag;
    int rand;
    flag = true;
    rand = (5 + 6) / 10 * (10 % 3);

    if(flag) then
        print("Flag is true and rand is: ", rand);
    else
        print("Flag is false!");
    endif

    int counter = 0;
    while(counter < 10) do
        count = count + 1;
        print("The value of count is: ", count);
    endwhile
halt
```

# Lexer:

**Members**

- **input**: Peekable\<Chars>
- **line**: u32
- **pushed_back_token**: Option\<Token>

**Methods**

- **pub new(&str) -> Lexer**
  - Converts the provided string into a peekable iterator and returns a new Lexer object with the iterator the input field.
- **pub next() -> Option\<Token>**
  - Iterates through characters in input until a token is found and returns it
  - If there is a token in the pushed_back_token field it will return that instead.
  - Returns none if EOF is reached in a healthy state.
  - Returns Token::ERROR(String) in the case of an error.
- **pub push_back(Token)**
  - Sets the pushed_back_token field of the lexer to the parameter token.
- **error(&str) -> Token**
  - Returns a Token::ERROR with a generalized error message inside.
- **cmp_next_char(&char) -> bool**
  - Peeks the next char and checks if it is equal to the provided char.
  - Consumes the next char if they match.

# Parser:

**Members**

- **lexer**: oxide::Lexer

**Methods**

- **pub new(&str) -> Parser**

  - Initializes the lexer member with a new lexer from the provided input string.

- **next_token() -> Result\<Token, ParseError>**

  - Internal utility function to return either the next token or an end of input error.

- **cmp_next_token(Token) -> Result\<Token, ParseError>**

  - Compares the next token with the provided token, if their variants match, returns the next token, if not returns an error with the format "Expected token {}", where {} is the target token.

- **cmp_next_token_many(Vec\<Token>, &str) -> Result\<Token, ParseError>**

  - Compares the next token against a vector of target tokens. If it matches, returns the next token, if not or the token is an error returns an error.

- **parse\_{node} -> Result\<{node}, ParseError>**
  - Implemented for each node in the parse tree, parses terminals and calls the associated functions for nonterminals to build a node of the tree and returns it.

# Interpreter:

- Executes parse tree
- Identifies semantic errors

# Notes:

For now I will write tests for the lexer to be absolutely sure that its working properly.

- Look into using iterator adaptors to refactor lexer.
  - May not be fully utilizing the properties of the peekable iterator that is the input.
- Why getting missing halt every time?
  - I think its because my parse_stmt_list function is at some point consuming the halt token.
  - The function i think is parsing statements until there is an error, where it then returns the accumulated statements.
  - I think that when it reaches the end of the statement list its consuming the halt keyword to try to parse the next stmt.
- It's clear that my next token or cmp next token functions are doing something fundamentally flawed.