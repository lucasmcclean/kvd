use indexmap::IndexMap;

#[derive(Default, Clone, PartialEq, Debug)]
pub enum Value {
    #[default]
    Null,
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Array(Vec<Value>),
    Object(IndexMap<String, Value>),
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Element {
    Key(String),
    Index(usize),
}

#[derive(Default, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Path {
    elements: Vec<Element>,
}

impl Path {
    pub fn elements(&self) -> &[Element] {
        &self.elements
    }

    pub fn push_key(&mut self, key: impl Into<String>) {
        self.elements.push(Element::Key(key.into()));
    }

    pub fn push_index(&mut self, index: usize) {
        self.elements.push(Element::Index(index));
    }

    pub fn pop(&mut self) -> Option<Element> {
        self.elements.pop()
    }

    pub fn is_root(&self) -> bool {
        self.elements.is_empty()
    }
}
