use std::cell::RefCell;
use std::rc::Rc;
use tir_backend::{AsmPrintable, BinaryEmittable};
use tir_core::{
    utils::{trait_id, TraitId},
    Dialect, Op, Operation, OperationImpl,
};
use tir_macros::{dialect, operation, populate_dialect_ops, populate_dialect_types};

dialect!(test_backend);
populate_dialect_ops!(AddOp);
populate_dialect_types!();

#[operation(name = "add", traits(BinaryEmittable, AsmPrintable))]
struct AddOp {}

impl BinaryEmittable for AddOp {
    fn encode(
        &self,
        _target_opts: &tir_backend::TargetOptions,
        _stream: &mut Box<dyn tir_backend::BinaryStream>,
    ) -> tir_core::Result<()> {
        unimplemented!()
    }

    fn try_decode(_data: &[u8]) -> tir_core::Result<Operation> {
        unimplemented!()
    }
}

impl AsmPrintable for AddOp {
    fn print(&self, _target_opts: &tir_backend::TargetOptions) {
        unimplemented!()
    }

    fn try_parse(_instruction: &str) -> tir_core::Result<Operation> {
        unimplemented!()
    }
}

#[test]
fn test_has_traits() {
    assert!(AddOp::has_trait::<dyn BinaryEmittable>());
    assert!(AddOp::has_trait::<dyn AsmPrintable>());
}
