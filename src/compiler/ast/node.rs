use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::compiler::ast::kind::SyntaxKind;
use crate::compiler::ast::node_flags::NodeFlags;
use crate::compiler::ast::symbol::Symbol;
use crate::compiler::ast::visitor::{NodeVisitor, Visitor};
use crate::compiler::text::TextRange;

/// AST Node - fundamental building block of the TypeScript abstract syntax tree
///
/// Interface values stored in AST nodes are never null. Construction code must ensure that
/// interface valued properties either store None or a reference to a valid struct.
#[derive(Debug)]
pub struct Node {
    /// Syntactic kind of the node
    pub kind: SyntaxKind,

    /// Flags applicable to this node
    pub flags: NodeFlags,

    /// Source text position information
    pub loc: TextRange,

    /// Unique identifier for this node
    id: AtomicU64,

    /// Parent node reference
    pub parent: Option<Arc<Node>>,

    /// Underlying node data (uses trait object instead of Go's interface)
    data: Arc<dyn NodeData>,
}

/// NodeData provides specialized data and behavior based on node type
pub trait NodeData: std::fmt::Debug + Send + Sync {
    /// Process each child node with the given visitor
    fn for_each_child(&self, v: &mut dyn Visitor) -> bool;

    /// Create a deep copy of this node
    fn clone_node(&self, factory: &NodeFactory) -> Arc<Node>;

    /// Transform child nodes with the given visitor
    fn visit_each_child(&self, v: &NodeVisitor) -> Arc<Node>;

    /// Get the declaration name if this node has one
    fn name(&self) -> Option<Arc<DeclarationName>> {
        None
    }

    /// Get modifiers if this node has them
    fn modifiers(&self) -> Option<Arc<ModifierList>> {
        None
    }

    /// Get flow analysis data if applicable
    fn flow_node_data(&self) -> Option<Arc<FlowNodeBase>> {
        None
    }

    /// Get declaration data if this node is a declaration
    fn declaration_data(&self) -> Option<Arc<DeclarationBase>> {
        None
    }

    /// Get exportable data if this node is exportable
    fn exportable_data(&self) -> Option<Arc<ExportableBase>> {
        None
    }

    /// Get locals container data if this node contains local variables
    fn locals_container_data(&self) -> Option<Arc<LocalsContainerBase>> {
        None
    }

    /// Get function-like data if this node is a function-like declaration
    fn function_like_data(&self) -> Option<Arc<FunctionLikeBase>> {
        None
    }

    /// Get class-like data if this node is a class-like declaration
    fn class_like_data(&self) -> Option<Arc<ClassLikeBase>> {
        None
    }

    /// Get body data if this node has a body
    fn body_data(&self) -> Option<Arc<BodyBase>> {
        None
    }

    /// Get literal-like data if this node is a literal
    fn literal_like_data(&self) -> Option<Arc<LiteralLikeBase>> {
        None
    }

    /// Get template literal data if this node is a template literal
    fn template_literal_like_data(&self) -> Option<Arc<TemplateLiteralLikeBase>> {
        None
    }
}

impl Node {
    /// Get the node itself
    pub fn as_node(&self) -> &Node {
        self
    }

    /// Get start position in source
    pub fn pos(&self) -> usize {
        self.loc.pos()
    }

    /// Get end position in source
    pub fn end(&self) -> usize {
        self.loc.end()
    }

    /// Process each child with the given visitor
    pub fn for_each_child(&self, v: &mut dyn Visitor) -> bool {
        self.data.for_each_child(v)
    }

    /// Create a deep copy of this node
    pub fn clone(&self, f: &NodeFactory) -> Arc<Node> {
        self.data.clone_node(f)
    }

    /// Transform child nodes with the given visitor
    pub fn visit_each_child(&self, v: &NodeVisitor) -> Arc<Node> {
        self.data.visit_each_child(v)
    }

    /// Get the declaration name if this node has one
    pub fn name(&self) -> Option<Arc<DeclarationName>> {
        self.data.name()
    }

    /// Get modifiers if this node has them
    pub fn modifiers(&self) -> Option<Arc<ModifierList>> {
        self.data.modifiers()
    }

    /// Get flow analysis data if applicable
    pub fn flow_node_data(&self) -> Option<Arc<FlowNodeBase>> {
        self.data.flow_node_data()
    }

    /// Get declaration data if this node is a declaration
    pub fn declaration_data(&self) -> Option<Arc<DeclarationBase>> {
        self.data.declaration_data()
    }

    /// Get exportable data if this node is exportable
    pub fn exportable_data(&self) -> Option<Arc<ExportableBase>> {
        self.data.exportable_data()
    }

    /// Get locals container data if this node contains local variables
    pub fn locals_container_data(&self) -> Option<Arc<LocalsContainerBase>> {
        self.data.locals_container_data()
    }

    /// Get function-like data if this node is a function-like declaration
    pub fn function_like_data(&self) -> Option<Arc<FunctionLikeBase>> {
        self.data.function_like_data()
    }

    /// Get parameter list if this node is a function-like declaration
    pub fn parameter_list(&self) -> Option<Arc<ParameterList>> {
        self.function_like_data()
            .map(|data| data.parameters.clone())
    }

    /// Get parameters if this node is a function-like declaration
    pub fn parameters(&self) -> Option<Vec<Arc<ParameterDeclarationNode>>> {
        self.parameter_list().map(|list| list.nodes.clone())
    }

    /// Get class-like data if this node is a class-like declaration
    pub fn class_like_data(&self) -> Option<Arc<ClassLikeBase>> {
        self.data.class_like_data()
    }

    /// Get body data if this node has a body
    pub fn body_data(&self) -> Option<Arc<BodyBase>> {
        self.data.body_data()
    }

    /// Get literal-like data if this node is a literal
    pub fn literal_like_data(&self) -> Option<Arc<LiteralLikeBase>> {
        self.data.literal_like_data()
    }

    /// Get template literal data if this node is a template literal
    pub fn template_literal_like_data(&self) -> Option<Arc<TemplateLiteralLikeBase>> {
        self.data.template_literal_like_data()
    }

    /// Get the symbol for this node
    pub fn symbol(&self) -> Option<Arc<Symbol>> {
        self.declaration_data().and_then(|data| data.symbol.clone())
    }

    /// Get the local symbol for this node if it's exportable
    pub fn local_symbol(&self) -> Option<Arc<Symbol>> {
        self.exportable_data()
            .and_then(|data| data.local_symbol.clone())
    }

    /// Get the locals table for this node if it's a locals container
    pub fn locals(&self) -> Option<HashMap<String, Arc<Symbol>>> {
        self.locals_container_data()
            .and_then(|data| data.locals.clone())
    }

    /// Get the body node if this node has a body
    pub fn body(&self) -> Option<Arc<Node>> {
        self.body_data().and_then(|data| data.body.clone())
    }

    /// Get the unique ID for this node
    pub fn id(&self) -> u64 {
        self.id.load(Ordering::SeqCst)
    }

    /// Set the unique ID for this node
    pub fn set_id(&self, id: u64) {
        self.id.store(id, Ordering::SeqCst);
    }

    /// Determines if this node contains the given descendant
    ///
    /// Works by walking up the Parent pointers from descendant. This method will
    /// panic if descendant or one of its ancestors is not parented, except when
    /// that node is a SourceFile.
    pub fn contains(&self, descendant: &Node) -> bool {
        let mut current = Some(descendant);

        while let Some(node) = current {
            if std::ptr::eq(node, self) {
                return true;
            }

            // Handle SourceFile as a special case
            if node.kind == SyntaxKind::SourceFile {
                return false;
            }

            // Get the parent or panic if there's no parent
            match &node.parent {
                Some(parent) => current = Some(parent.as_ref()),
                None => panic!("Node not parented: {:?}", node),
            }
        }

        false
    }
}
