use crate::base::{Expr, Table};
use crate::logical::LogicalPlan;

// pub enum OpCode {
//     Scan,
//     Project,
//     Filter,
// }

#[derive(Debug)]
pub enum Instruction {
    Scan {
        dest: u8,
        table: Table,
    },
    Project {
        src: u8,
        dest: u8,
        columns: Vec<Expr>,
    },
    Filter {
        src: u8,
        dest: u8,
        predicate: Box<Expr>,
    },
    Return {
        src: u8,
    },
}

#[derive(Debug)]
pub struct ByteCode {
    instructions: Vec<Instruction>,
}

impl ByteCode {
    pub fn new() -> Self {
        ByteCode {
            instructions: Vec::new(),
        }
    }

    pub fn compile_to_bytecode(plan: &LogicalPlan, next_reg: &mut u8) -> (u8, ByteCode) {
        match plan {
            LogicalPlan::Scan { table } => {
                let dest = *next_reg;
                *next_reg += 1;
                let mut bytecode = Vec::new();
                bytecode.push(Instruction::Scan {
                    dest,
                    table: table.clone(),
                });
                (
                    dest,
                    ByteCode {
                        instructions: bytecode,
                    },
                )
            }
            LogicalPlan::Filter { input, predicate } => {
                let (src_reg, mut bytecode) = ByteCode::compile_to_bytecode(input, next_reg);
                let dest = *next_reg;
                *next_reg += 1;
                bytecode.instructions.push(Instruction::Filter {
                    src: src_reg,
                    dest,
                    predicate: predicate.clone(),
                });
                (
                    dest,
                    ByteCode {
                        instructions: bytecode.instructions,
                    },
                )
            }
            LogicalPlan::Project { input, columns } => {
                let (src_reg, mut bytecode) = ByteCode::compile_to_bytecode(input, next_reg);
                let dest = *next_reg;
                *next_reg += 1;
                bytecode.instructions.push(Instruction::Project {
                    src: src_reg,
                    dest,
                    columns: columns.clone(),
                });
                (dest, bytecode)
            }
        }
    }

    pub fn compile(plan: &LogicalPlan, next_reg: &mut u8) -> ByteCode {
        let (dest, mut bytecode) = ByteCode::compile_to_bytecode(plan, next_reg);
        bytecode
            .instructions
            .push(Instruction::Return { src: dest });
        bytecode
    }
}
