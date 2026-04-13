# Query Strategy

## Introduction

### Overview

- The Query Module is responsible for lexing and parsing queries against the database with a SQL like syntax.
- Module consists of 3 main components:
    - Token: this component defines what a token is and how it is created.
    - Lexer: responsible for breaking down the query into tokens.
    - Parser: responsible for parsing the tokens into an abstract syntax tree.


## Token
- This component defines what a token is and how it is created.

### Definition
A token is a sequence of characters that represents a single unit of meaning in a query. Tokens can be classified into different types such as keywords, identifiers, literals, and operators.

### Creation
Tokens are created by the lexer, which is responsible for breaking down the query into individual tokens. The lexer uses a set of rules to identify the type of each token and to extract its value.

### Implementation
Tokens are defined using enums, which provide a clear and concise way to represent the different types of tokens. Each token type is associated with a specific pattern that the lexer uses to match tokens in the query.

```rust
pub enum Token {
    // Type of data
    IDENTIFIER(String),
    NUMBER(u64),
    STRING(String),
    
    // Main keywords
    SELECT,
    FROM,
    WHERE,
    ...
}
```


## Lexer

### Definition
The lexer is responsible for breaking down the query into individual tokens. It uses a set of rules to identify the type of each token and to extract its value.

### Implementation
The lexer is implemented using a finite state machine that transitions between different states based on the input characters. Each state corresponds to a specific token type, and the lexer uses regular expressions to match the input against the patterns defined for each token type.

### Example
Consider the following query:

```sql
SELECT * FROM users WHERE age > 18;
```

The lexer would break this query into the following tokens:

```rust
[
    Token::SELECT,
    Token::STAR,
    Token::FROM,
    Token::IDENTIFIER("users".to_string()),
    Token::WHERE,
    Token::IDENTIFIER("age".to_string()),
    Token::GREATER_THAN,
    Token::NUMBER(18),
    Token::SEMICOLON,
]
```


## Parser

### Definition
The parser is responsible for taking the sequence of tokens produced by the lexer and constructing an abstract syntax tree (AST) that represents the query. The parser uses a set of rules to determine the structure of the query and to validate its syntax.

### Implementation
The parser is implemented using a recursive descent parser that uses a stack to keep track of the current state of the parsing process. The parser uses a set of rules to match the tokens against the expected syntax and to construct the AST.

### Example
Consider the following query:

```sql
SELECT * FROM users WHERE age > 18;
```

The parser would produce the following AST:

```rust
Query::Select {
    columns: vec![Column::All],
    table: "users".to_string(),
    where_clause: Some(Expr::BinaryOp {
        left: Expr::Identifier("age".to_string()),
        op: Token::GreaterThan,
        right: Expr::Number(18),
    }),
}
```
