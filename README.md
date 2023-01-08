# O X I D E: 

**Grammar in EBNF Notation**
```
Program     ::= INIT <StmtList> HALT
StmtList    ::= <Stmt>; { <Stmt>; }
Stmt        ::= <DeclStmt> | <CtrlStmt>
DeclStmt    ::= (INT | FLOAT | BOOL | STRING) <VarList>
CtrlStmt    ::= <AssignStmt> | <PrintStmt> | <IfStmt> | <LoopStmt>
VarList     ::= <Var> { , <Var> }
Var         ::= IDENT
AssignStmt  ::= <Var> = <OrExpr>
PrintStmt   ::= PRINT(<ExprList>)
IfStmt      ::= IF (<OrExpr>) THEN <StmtList> [ ELSE <StmtList> ] ENDIF
LoopStmt    ::= WHILE (<OrExpr>) DO <StmtList> ENDWHILE
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
- Tokenizes input
- Identifies grammatical errors

# Parser:
- Builds parse tree from tokenized input
- Identifies syntactic errors

# Interpreter:
- Executes parse tree
- Identifies semantic errors 

How am i going to represent things?
How am i going to group together functions and data?