#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Ident(String),
    Number(u64),
    String(String),
    Wildcard,
    Binary {
        left: Box<Expr>,
        op: String,
        right: Box<Expr>,
    },
    Unary {
        op: String,
        expr: Box<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Query {
    Select {
        columns: Vec<Expr>,
        from: String,
        where_clause: Option<Box<Expr>>,
    },
    Insert {
        table: String,
        columns: Vec<String>,
        values: Vec<Vec<Expr>>,
    },
    Update {
        table: String,
        set_clause: Vec<(String, Box<Expr>)>,
        where_clause: Option<Box<Expr>>,
    },
    Delete {
        table: String,
        where_clause: Option<Box<Expr>>,
    },
}
