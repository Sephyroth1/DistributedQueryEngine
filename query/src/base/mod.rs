use std::fmt;

#[derive(Debug, Clone)]
pub enum Op {
    EQ,
    ASSIGN,
    NE,
    LT,
    GT,
    AND,
    OR,
}
impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Op::EQ => write!(f, "="),
            Op::ASSIGN => write!(f, ":="),
            Op::NE => write!(f, "!="),
            Op::LT => write!(f, "<"),
            Op::GT => write!(f, ">"),
            Op::AND => write!(f, "AND"),
            Op::OR => write!(f, "OR"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Identifier(String),
    Number(u64),
    Binary {
        op: Op,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    WildCard,
}

impl Expr {
    pub fn print(&self) -> String {
        match self {
            Expr::Identifier(id) => id.clone(),
            Expr::Number(num) => num.to_string(),
            Expr::Binary { op, left, right } => {
                format!("({} {} {})", left.print(), op, right.print())
            }
            Expr::WildCard => "*".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum SelectNode {
    Select {
        table: String,
        columns: Vec<Expr>,
        where_clause: Option<Expr>,
    },
}

pub enum InsertNode {
    Insert {
        table: String,
        columns: Vec<String>,
        values: Vec<String>,
    },
}

pub enum Node {
    Select(SelectNode),
    Insert(InsertNode),
}
