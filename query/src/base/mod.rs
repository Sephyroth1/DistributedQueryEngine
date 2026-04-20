pub enum DataTypes {
    INT,
    FLOAT,
    BOOL,
    STRING,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(u64),
    Float(f64),
    Bool(bool),
    String(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Table {
    pub table_id: usize,
    pub name: String,
    pub columns: Vec<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Column {
        name: String,
        column_id: usize,
    },
    Literal(Value),
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
pub enum Expr1 {
    Ident(String),
    Number(u64),
    String(String),
    Wildcard,
    Binary {
        left: Box<Expr1>,
        op: String,
        right: Box<Expr1>,
    },
    Unary {
        op: String,
        expr: Box<Expr1>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Query {
    Select {
        columns: Vec<Expr>,
        from: Table,
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
