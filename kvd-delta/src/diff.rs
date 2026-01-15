use crate::value::{Path, Value};

#[derive(Clone, PartialEq, Debug)]
pub enum Delta {
    Add { path: Path, value: Value },
    Remove { path: Path, value: Value },
    Change { path: Path, from: Value, to: Value },
}

#[derive(Default, Clone, PartialEq, Debug)]
pub struct Diff {
    deltas: Vec<Delta>,
}

impl Diff {
    pub fn deltas(&self) -> &[Delta] {
        &self.deltas
    }

    pub fn is_empty(&self) -> bool {
        self.deltas.is_empty()
    }
}

pub fn diff(left: &Value, right: &Value) -> Diff {
    unimplemented!()
}
