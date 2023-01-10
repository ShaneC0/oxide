# oxide: 

**Grammar in EBNF Notation**
```
Program     ::= init <StmtList> halt
StmtList    ::= <Stmt>; { <Stmt>; }
Stmt        ::= <DeclStmt> | <CtrlStmt>
DeclStmt    ::= (int | float | bool | string) IDENT { , IDENT }
CtrlStmt    ::= <AssignStmt> | <PrintStmt> | <IfStmt> | <LoopStmt>
AssignStmt  ::= IDENT = <OrExpr>
PrintStmt   ::= print(<Expr> { , <Expr> })
IfStmt      ::= if (<OrExpr>) then <StmtList> [ else <StmtList> ] endif
LoopStmt    ::= while (<OrExpr>) do <StmtList> endwhile
OrExpr      ::= <AndExpr> { || <AndExpr> }
AndExpr     ::= <EqualExpr> { && <EqualExpr> }
EqualExpr   ::= <RelExpr> [ == <RelExpr> ]
RelExpr     ::= <AddExpr> [ (< | >) <AddExpr> ]
AddExpr     ::= <MultExpr> { (+ | -) <MultExpr> }
MultExpr    ::= <UnaryExpr> { (* | / | %) <UnaryExpr> }
UnaryExpr   ::= (- | !) <PrimaryExpr> | <PrimaryExpr>
PrimaryExpr ::= ICONST | FCONST | BCONST | SCONST | (<Expr>)
```

**Format of Lexemes**
```
Identifier: /[a-zA-Z_][a-zA-Z0-9_]*/
Integer:    /[0-9]+/
Float:      /[0-9]+\.[0-9]+/
String:     /"  Pretty much anything in here   "/
Bool:       /(true | false)/
Keyword:    /[a-zA-Z]+/
```

**Example Program**
```
init 
    bool flag;
    flag = true;
    if(flag) then
        print("Flag is true!");
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

Note: Remember to implement the logic for returning a pushed back token.

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
- Builds parse tree from tokenized input
- Identifies syntactic errors

# Interpreter:
- Executes parse tree
- Identifies semantic errors 