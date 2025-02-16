use std::{cell::RefCell, fmt::Debug, rc::Rc};

use crate::Dialect;

/// Context holds all the resources required for building an IR
///
/// Examples:
/// ```
/// use tir_core::{Dialect, Context};
///
/// fn foo(dialect_obj: Dialect) {
///     // Create a new context
///     let mut context = Context::new();
///     // Register dialects
///     let dialect = context.borrow_mut().add_dialect(dialect_obj);
/// }
/// ```
pub struct Context {
    dialects: Vec<Rc<RefCell<Dialect>>>,
}

impl Debug for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Context")?;
        Ok(())
    }
}

impl Context {
    /// Create a new context
    ///
    /// Every newly-created context will have at least two dialects registered:
    /// builtin dialect and std dialect
    pub fn new() -> Rc<RefCell<Context>> {
        let mut ctx = Context { dialects: vec![] };
        ctx.add_dialect(crate::builtin::create_dialect());
        Rc::new(RefCell::new(ctx))
    }

    /// Register a new dialect with a context
    pub fn add_dialect(&mut self, dialect: Dialect) -> Rc<RefCell<Dialect>> {
        self.dialects.push(Rc::new(RefCell::new(dialect)));
        let dialect_ref = self.dialects.last().unwrap();
        dialect_ref
            .borrow_mut()
            .set_id((self.dialects.len() - 1).try_into().unwrap());

        dialect_ref.clone()
    }

    pub fn get_dialect_by_name(&self, name: &str) -> Option<Rc<RefCell<Dialect>>> {
        for dialect in &self.dialects {
            if dialect.borrow().get_name() == name {
                return Some(dialect.clone());
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_context() {
        let context = Context::new();
        assert_eq!(context.borrow().dialects.len(), 1);
        assert!(context.borrow().get_dialect_by_name("builtin").is_some());
    }
}
