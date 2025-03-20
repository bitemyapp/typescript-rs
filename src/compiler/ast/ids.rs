/// Unique identifier for AST nodes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NodeId(pub(crate) u64);

/// Unique identifier for symbols
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SymbolId(pub(crate) u64);

impl NodeId {
    /// Creates a new NodeId with the given value
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    /// Gets the raw u64 value
    pub fn value(&self) -> u64 {
        self.0
    }
}

impl SymbolId {
    /// Creates a new SymbolId with the given value
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    /// Gets the raw u64 value
    pub fn value(&self) -> u64 {
        self.0
    }
}
