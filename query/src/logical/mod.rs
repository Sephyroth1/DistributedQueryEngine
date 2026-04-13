use crate::base::{Expr, Query};

#[derive(Debug, Clone)]
pub enum LogicalPlan {
    Scan {
        table: String,
    },
    Project {
        input: Box<LogicalPlan>,
        columns: Vec<Expr>,
    },
    Filter {
        input: Box<LogicalPlan>,
        predicate: Box<Expr>,
    },
}

impl LogicalPlan {
    pub fn clone(&self) -> Self {
        match self {
            LogicalPlan::Scan { table } => LogicalPlan::Scan {
                table: table.clone(),
            },
            LogicalPlan::Project { input, columns } => LogicalPlan::Project {
                input: input.clone(),
                columns: columns.clone(),
            },
            LogicalPlan::Filter { input, predicate } => LogicalPlan::Filter {
                input: input.clone(),
                predicate: predicate.clone(),
            },
        }
    }

    // Ok(Select { columns: [Ident("hello")], from: "world", where_clause: None })
    pub fn ast_to_lplan(q: Query) -> LogicalPlan {
        match q {
            Query::Select {
                columns,
                from,
                where_clause,
            } => {
                let columns = columns;
                let from = from;
                let where_clause = where_clause;
                let plan = LogicalPlan::Scan { table: from };
                let plan1 = LogicalPlan::Filter {
                    input: Box::new(plan),
                    predicate: where_clause.unwrap_or(Box::new(Expr::Ident("true".to_string()))),
                };
                let plan2 = LogicalPlan::Project {
                    input: Box::new(plan1),
                    columns,
                };

                plan2
            }
            _ => {
                panic!("Unsupported query type")
            }
        }
    }
}
