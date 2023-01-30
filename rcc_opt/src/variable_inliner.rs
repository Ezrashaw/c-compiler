use std::collections::HashMap;

use rcc_parser::ast::{BlockItem, Expression, Statement};

use crate::OptimizationPass;

pub struct VariableInliner {
    /// Map of inlined variables to constant values
    inlined: HashMap<u32, i32>,
}

impl VariableInliner {
    pub fn new() -> Self {
        Self {
            inlined: HashMap::new(),
        }
    }
}

impl OptimizationPass for VariableInliner {
    fn opt_block_item(&mut self, item: &mut BlockItem) {
        if let BlockItem::Declaration(id, val) = item {
            if let Some(Expression::Literal { val }) = val {
                self.inlined.insert(*id, *val);
                *item =
                    BlockItem::Statement(Statement::Expression(Expression::Literal { val: *val }));
            } else if let None = val {
                // variables are init'ed to 0 implicitly
                self.inlined.insert(*id, 0);
                *item = BlockItem::Statement(Statement::Expression(Expression::Literal { val: 0 }));
            }
        }
    }

    fn opt_expression(&mut self, expr: &mut Expression) {
        if let Expression::Variable { identifier } = expr {
            if let Some(val) = self.inlined.get(&identifier) {
                *expr = Expression::Literal { val: *val };
            }
        } else if let Expression::Assignment {
            identifier,
            expression: init,
        } = expr
        {
            if let Expression::Literal { val } = init.as_ref() {
                *self.inlined.get_mut(&identifier).expect("internal error") = *val;

                *expr = Expression::Literal { val: *val };
            }
        }
    }
}
