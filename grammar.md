Tokens
-------
<Morpheme> ::= .+
<Str> ::= ".*"
<Pipe> ::= |

AST Nodes
----------
<List> ::= (<Morpheme> | <Str>)*
<Invocable> ::= <Morpheme>
<Invocation> ::= <Invocable> <List>
