# oxide: 

**Grammar in EBNF Notation**
```
Program     ::= init <StmtList> halt
StmtList    ::= <Stmt>; { <Stmt>; }
Stmt        ::= <DeclStmt> | <CtrlStmt>
DeclStmt    ::= (int | float | bool | string) <VarList>
CtrlStmt    ::= <AssignStmt> | <PrintStmt> | <IfStmt> | <LoopStmt>
VarList     ::= <Var> { , <Var> }
Var         ::= IDENT
AssignStmt  ::= <Var> = <OrExpr>
PrintStmt   ::= print(<ExprList>)
IfStmt      ::= if (<OrExpr>) then <StmtList> [ else <StmtList> ] endif
LoopStmt    ::= while (<OrExpr>) do <StmtList> endwhile
ExprList    ::= <OrExpr> { , <OrExpr> }
OrExpr      ::= <AndExpr> { || <AndExpr> }
AndExpr     ::= <EqualExpr> { && <EqualExpr> }
EqualExpr   ::= <RelExpr> [ == <RelExpr> ]
RelExpr     ::= <AddExpr> [ (< | >) <AddExpr> ]
AddExpr     ::= <MultExpr> { (+ | -) <MultExpr> }
MultExpr    ::= <UnaryExpr> { (* | / | %) <UnaryExpr> }
UnaryExpr   ::= (+ | - | !) <PrimaryExpr> | <PrimaryExpr>
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
```

# Lexer:

- Members
    - input: Peekable\<Chars>
    - line: u32
    - pushed_back_token: Option\<Token>

- Methods
    - pub new(&str) -> Lexer
        - Constructor
    - pub next() -> Option\<Token>
    - pub push_back(Token)
    - error(&str) -> Token
    - cmp_next_char(&char) -> bool

- Errors
    - Errors are returned as a Token::Error(String) which contains "Unrecognized token on line {line #}: {lexeme}".
    - When reaching end of file, will return the last token if not in start state or None if in start state.

# Parser:
- Builds parse tree from tokenized input
- Identifies syntactic errors

# Interpreter:
- Executes parse tree
- Identifies semantic errors 