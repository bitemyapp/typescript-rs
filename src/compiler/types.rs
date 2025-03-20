// compiler/mod.rs
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

/// SyntaxKind represents all possible syntax elements in TypeScript/JavaScript
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i16)]
pub enum SyntaxKind {
    Unknown = 0,
    EndOfFile,
    ConflictMarkerTrivia,
    NonTextFileMarkerTrivia,
    NumericLiteral,
    BigintLiteral,
    StringLiteral,
    JsxText,
    JsxTextAllWhiteSpaces,
    RegularExpressionLiteral,
    NoSubstitutionTemplateLiteral,
    // Pseudo-literals
    TemplateHead,
    TemplateMiddle,
    TemplateTail,
    // Punctuation
    OpenBraceToken,
    CloseBraceToken,
    OpenParenToken,
    CloseParenToken,
    OpenBracketToken,
    CloseBracketToken,
    DotToken,
    DotDotDotToken,
    SemicolonToken,
    CommaToken,
    QuestionDotToken,
    LessThanToken,
    LessThanSlashToken,
    GreaterThanToken,
    LessThanEqualsToken,
    GreaterThanEqualsToken,
    EqualsEqualsToken,
    ExclamationEqualsToken,
    EqualsEqualsEqualsToken,
    ExclamationEqualsEqualsToken,
    EqualsGreaterThanToken,
    PlusToken,
    MinusToken,
    AsteriskToken,
    AsteriskAsteriskToken,
    SlashToken,
    PercentToken,
    PlusPlusToken,
    MinusMinusToken,
    LessThanLessThanToken,
    GreaterThanGreaterThanToken,
    GreaterThanGreaterThanGreaterThanToken,
    AmpersandToken,
    BarToken,
    CaretToken,
    ExclamationToken,
    TildeToken,
    AmpersandAmpersandToken,
    BarBarToken,
    QuestionToken,
    ColonToken,
    AtToken,
    QuestionQuestionToken,
    // Only the JSDoc scanner produces BacktickToken. Normal scanner produces NoSubstitutionTemplateLiteral
    BacktickToken,
    // Only the JSDoc scanner produces HashToken. Normal scanner produces PrivateIdentifier
    HashToken,
    // Assignments
    EqualsToken,
    PlusEqualsToken,
    MinusEqualsToken,
    AsteriskEqualsToken,
    AsteriskAsteriskEqualsToken,
    SlashEqualsToken,
    PercentEqualsToken,
    LessThanLessThanEqualsToken,
    GreaterThanGreaterThanEqualsToken,
    GreaterThanGreaterThanGreaterThanEqualsToken,
    AmpersandEqualsToken,
    BarEqualsToken,
    CaretEqualsToken,
    // Identifiers and PrivateIdentifier
    Identifier,
    PrivateIdentifier,

    // ... more enum variants would go here ...

    // We're truncating the full list for brevity
    // In a real implementation, all variants would be included
    Count,
}

// Define marker constants for SyntaxKind ranges
impl SyntaxKind {
    pub const FIRST_ASSIGNMENT: SyntaxKind = SyntaxKind::EqualsToken;
    pub const LAST_ASSIGNMENT: SyntaxKind = SyntaxKind::CaretEqualsToken;
    pub const FIRST_COMPOUND_ASSIGNMENT: SyntaxKind = SyntaxKind::PlusEqualsToken;
    pub const LAST_COMPOUND_ASSIGNMENT: SyntaxKind = SyntaxKind::CaretEqualsToken;
    // ... other markers would be defined similarly
}

use bitflags::bitflags;

bitflags! {
    /// NodeFlags represents attributes of AST nodes
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct NodeFlags: u32 {
        const NONE = 0;
        const LET = 1 << 0;
        const CONST = 1 << 1;
        const USING = 1 << 2;
        const NESTED_NAMESPACE = 1 << 3;
        const SYNTHESIZED = 1 << 4;
        const NAMESPACE = 1 << 5;
        const OPTIONAL_CHAIN = 1 << 6;
        const EXPORT_CONTEXT = 1 << 7;
        const CONTAINS_THIS = 1 << 8;
        const HAS_IMPLICIT_RETURN = 1 << 9;
        const HAS_EXPLICIT_RETURN = 1 << 10;
        const GLOBAL_AUGMENTATION = 1 << 11;
        const HAS_ASYNC_FUNCTIONS = 1 << 12;
        const DISALLOW_IN_CONTEXT = 1 << 13;
        const YIELD_CONTEXT = 1 << 14;
        const DECORATOR_CONTEXT = 1 << 15;
        const AWAIT_CONTEXT = 1 << 16;
        const DISALLOW_CONDITIONAL_TYPES_CONTEXT = 1 << 17;
        const THIS_NODE_HAS_ERROR = 1 << 18;
        const JAVASCRIPT_FILE = 1 << 19;
        const THIS_NODE_OR_ANY_SUBNODES_HAS_ERROR = 1 << 20;
        const HAS_AGGREGATED_CHILD_DATA = 1 << 21;
        const POSSIBLY_CONTAINS_DYNAMIC_IMPORT = 1 << 22;
        const POSSIBLY_CONTAINS_IMPORT_META = 1 << 23;
        const JSDOC = 1 << 24;
        const AMBIENT = 1 << 25;
        const IN_WITH_STATEMENT = 1 << 26;
        const JSON_FILE = 1 << 27;
        const TYPE_CACHED = 1 << 28;
        const DEPRECATED = 1 << 29;

        // Compound flags
        const BLOCK_SCOPED = Self::LET.bits() | Self::CONST.bits() | Self::USING.bits();
        const CONSTANT = Self::CONST.bits() | Self::USING.bits();
        const AWAIT_USING = Self::CONST.bits() | Self::USING.bits();
        const REACHABILITY_CHECK_FLAGS = Self::HAS_IMPLICIT_RETURN.bits() | Self::HAS_EXPLICIT_RETURN.bits();
        const REACHABILITY_AND_EMIT_FLAGS = Self::REACHABILITY_CHECK_FLAGS.bits() | Self::HAS_ASYNC_FUNCTIONS.bits();
        const CONTEXT_FLAGS = Self::DISALLOW_IN_CONTEXT.bits() | Self::DISALLOW_CONDITIONAL_TYPES_CONTEXT.bits() |
                             Self::YIELD_CONTEXT.bits() | Self::DECORATOR_CONTEXT.bits() | Self::AWAIT_CONTEXT.bits() |
                             Self::JAVASCRIPT_FILE.bits() | Self::IN_WITH_STATEMENT.bits() | Self::AMBIENT.bits();
        const TYPE_EXCLUDES_FLAGS = Self::YIELD_CONTEXT.bits() | Self::AWAIT_CONTEXT.bits();
        const PERMANENTLY_SET_INCREMENTAL_FLAGS = Self::POSSIBLY_CONTAINS_DYNAMIC_IMPORT.bits() | Self::POSSIBLY_CONTAINS_IMPORT_META.bits();

        // Flags repurposed for Identifier nodes
        const IDENTIFIER_HAS_EXTENDED_UNICODE_ESCAPE = Self::CONTAINS_THIS.bits();
        const IDENTIFIER_IS_IN_JSDOC_NAMESPACE = Self::HAS_ASYNC_FUNCTIONS.bits();
    }
}

bitflags! {
    /// ModifierFlags for declarations
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct ModifierFlags: u32 {
        const NONE = 0;
        // Syntactic/JSDoc modifiers
        const PUBLIC = 1 << 0;
        const PRIVATE = 1 << 1;
        const PROTECTED = 1 << 2;
        const READONLY = 1 << 3;
        const OVERRIDE = 1 << 4;
        // Syntactic-only modifiers
        const EXPORT = 1 << 5;
        const ABSTRACT = 1 << 6;
        const AMBIENT = 1 << 7;
        const STATIC = 1 << 8;
        const ACCESSOR = 1 << 9;
        const ASYNC = 1 << 10;
        const DEFAULT = 1 << 11;
        const CONST = 1 << 12;
        const IN = 1 << 13;
        const OUT = 1 << 14;
        const DECORATOR = 1 << 15;
        const IMMEDIATE = 1 << 16;
        // JSDoc-only modifiers
        const DEPRECATED = 1 << 17;
        const JSDOC_IMMEDIATE = 1 << 18;
        // Cache-only JSDoc-modifiers
        const JSDOC_PUBLIC = 1 << 23;
        const JSDOC_PRIVATE = 1 << 24;
        const JSDOC_PROTECTED = 1 << 25;
        const JSDOC_READONLY = 1 << 26;
        const JSDOC_OVERRIDE = 1 << 27;
        const HAS_COMPUTED_JSDOC_MODIFIERS = 1 << 28;
        const HAS_COMPUTED_FLAGS = 1 << 29;

        // Compound flags
        const SYNTACTIC_OR_JSDOC_MODIFIERS = Self::PUBLIC.bits() | Self::PRIVATE.bits() | Self::PROTECTED.bits() |
                                          Self::READONLY.bits() | Self::OVERRIDE.bits();
        const SYNTACTIC_ONLY_MODIFIERS = Self::EXPORT.bits() | Self::AMBIENT.bits() | Self::ABSTRACT.bits() |
                                      Self::STATIC.bits() | Self::ACCESSOR.bits() | Self::ASYNC.bits() |
                                      Self::DEFAULT.bits() | Self::CONST.bits() | Self::IN.bits() |
                                      Self::OUT.bits() | Self::DECORATOR.bits() | Self::IMMEDIATE.bits();
        const SYNTACTIC_MODIFIERS = Self::SYNTACTIC_OR_JSDOC_MODIFIERS.bits() | Self::SYNTACTIC_ONLY_MODIFIERS.bits();
        const JSDOC_CACHE_ONLY_MODIFIERS = Self::JSDOC_PUBLIC.bits() | Self::JSDOC_PRIVATE.bits() |
                                         Self::JSDOC_PROTECTED.bits() | Self::JSDOC_READONLY.bits() |
                                         Self::JSDOC_OVERRIDE.bits();
        const JSDOC_ONLY_MODIFIERS = Self::DEPRECATED.bits() | Self::JSDOC_IMMEDIATE.bits();
        const NON_CACHE_ONLY_MODIFIERS = Self::SYNTACTIC_OR_JSDOC_MODIFIERS.bits() | Self::SYNTACTIC_ONLY_MODIFIERS.bits() |
                                      Self::JSDOC_ONLY_MODIFIERS.bits();

        const ACCESSIBILITY_MODIFIER = Self::PUBLIC.bits() | Self::PRIVATE.bits() | Self::PROTECTED.bits();
        const PARAMETER_PROPERTY_MODIFIER = Self::ACCESSIBILITY_MODIFIER.bits() | Self::READONLY.bits() | Self::OVERRIDE.bits();
        const NON_PUBLIC_ACCESSIBILITY_MODIFIER = Self::PRIVATE.bits() | Self::PROTECTED.bits();

        const TYPESCRIPT_MODIFIER = Self::AMBIENT.bits() | Self::PUBLIC.bits() | Self::PRIVATE.bits() |
                                 Self::PROTECTED.bits() | Self::READONLY.bits() | Self::ABSTRACT.bits() |
                                 Self::CONST.bits() | Self::OVERRIDE.bits() | Self::IN.bits() | Self::OUT.bits() |
                                 Self::IMMEDIATE.bits();
        const EXPORT_DEFAULT = Self::EXPORT.bits() | Self::DEFAULT.bits();
        const ALL = Self::EXPORT.bits() | Self::AMBIENT.bits() | Self::PUBLIC.bits() | Self::PRIVATE.bits() |
                 Self::PROTECTED.bits() | Self::STATIC.bits() | Self::READONLY.bits() | Self::ABSTRACT.bits() |
                 Self::ACCESSOR.bits() | Self::ASYNC.bits() | Self::DEFAULT.bits() | Self::CONST.bits() |
                 Self::DEPRECATED.bits() | Self::OVERRIDE.bits() | Self::IN.bits() | Self::OUT.bits() |
                 Self::IMMEDIATE.bits() | Self::DECORATOR.bits();
        const MODIFIER = Self::ALL.bits() & !Self::DECORATOR.bits();
    }
}

bitflags! {
    /// SignatureFlags for function signatures
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct SignatureFlags: u32 {
        const NONE = 0;
        const YIELD = 1 << 0;
        const AWAIT = 1 << 1;
        const TYPE = 1 << 2;
        const IGNORE_MISSING_OPEN_BRACE = 1 << 4;
        const JSDOC = 1 << 5;
    }
}

bitflags! {
    /// SymbolFlags for symbols in the type system
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct SymbolFlags: u32 {
        const NONE = 0;
        const FUNCTION_SCOPED_VARIABLE = 1 << 0;
        const BLOCK_SCOPED_VARIABLE = 1 << 1;
        const PROPERTY = 1 << 2;
        const ENUM_MEMBER = 1 << 3;
        const FUNCTION = 1 << 4;
        const CLASS = 1 << 5;
        const INTERFACE = 1 << 6;
        const CONST_ENUM = 1 << 7;
        const REGULAR_ENUM = 1 << 8;
        const VALUE_MODULE = 1 << 9;
        const NAMESPACE_MODULE = 1 << 10;
        const TYPE_LITERAL = 1 << 11;
        const OBJECT_LITERAL = 1 << 12;
        const METHOD = 1 << 13;
        const CONSTRUCTOR = 1 << 14;
        const GET_ACCESSOR = 1 << 15;
        const SET_ACCESSOR = 1 << 16;
        const SIGNATURE = 1 << 17;
        const TYPE_PARAMETER = 1 << 18;
        const TYPE_ALIAS = 1 << 19;
        const EXPORT_VALUE = 1 << 20;
        const ALIAS = 1 << 21;
        const PROTOTYPE = 1 << 22;
        const EXPORT_STAR = 1 << 23;
        const OPTIONAL = 1 << 24;
        const TRANSIENT = 1 << 25;
        const ASSIGNMENT = 1 << 26;
        const MODULE_EXPORTS = 1 << 27;
        const ALL = 0xFFFFFFFF;

        // Compound flags
        const ENUM = Self::REGULAR_ENUM.bits() | Self::CONST_ENUM.bits();
        const VARIABLE = Self::FUNCTION_SCOPED_VARIABLE.bits() | Self::BLOCK_SCOPED_VARIABLE.bits();
        const VALUE = Self::VARIABLE.bits() | Self::PROPERTY.bits() | Self::ENUM_MEMBER.bits() |
                   Self::OBJECT_LITERAL.bits() | Self::FUNCTION.bits() | Self::CLASS.bits() |
                   Self::ENUM.bits() | Self::VALUE_MODULE.bits() | Self::METHOD.bits() |
                   Self::GET_ACCESSOR.bits() | Self::SET_ACCESSOR.bits();
        const TYPE = Self::CLASS.bits() | Self::INTERFACE.bits() | Self::ENUM.bits() |
                 Self::ENUM_MEMBER.bits() | Self::TYPE_LITERAL.bits() | Self::TYPE_PARAMETER.bits() |
                 Self::TYPE_ALIAS.bits();
        const NAMESPACE = Self::VALUE_MODULE.bits() | Self::NAMESPACE_MODULE.bits() | Self::ENUM.bits();
        const MODULE = Self::VALUE_MODULE.bits() | Self::NAMESPACE_MODULE.bits();
        const ACCESSOR = Self::GET_ACCESSOR.bits() | Self::SET_ACCESSOR.bits();

        // ... other compound flags would be defined similarly
    }
}

bitflags! {
    /// TypeFlags for types in the type system
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct TypeFlags: u32 {
        const NONE = 0;
        const ANY = 1 << 0;
        const UNKNOWN = 1 << 1;
        const STRING = 1 << 2;
        const NUMBER = 1 << 3;
        const BOOLEAN = 1 << 4;
        const ENUM = 1 << 5;
        const BIGINT = 1 << 6;
        const STRING_LITERAL = 1 << 7;
        const NUMBER_LITERAL = 1 << 8;
        const BOOLEAN_LITERAL = 1 << 9;
        const ENUM_LITERAL = 1 << 10;
        const BIGINT_LITERAL = 1 << 11;
        const ES_SYMBOL = 1 << 12;
        const UNIQUE_ES_SYMBOL = 1 << 13;
        const VOID = 1 << 14;
        const UNDEFINED = 1 << 15;
        const NULL = 1 << 16;
        const NEVER = 1 << 17;
        const TYPE_PARAMETER = 1 << 18;
        const OBJECT = 1 << 19;
        const UNION = 1 << 20;
        const INTERSECTION = 1 << 21;
        const INDEX = 1 << 22;
        const INDEXED_ACCESS = 1 << 23;
        const CONDITIONAL = 1 << 24;
        const SUBSTITUTION = 1 << 25;
        const NON_PRIMITIVE = 1 << 26;
        const TEMPLATE_LITERAL = 1 << 27;
        const STRING_MAPPING = 1 << 28;
        const RESERVED1 = 1 << 29;
        const RESERVED2 = 1 << 30;
        const RESERVED3 = 1 << 31;

        // Compound flags
        const ANY_OR_UNKNOWN = Self::ANY.bits() | Self::UNKNOWN.bits();
        const NULLABLE = Self::UNDEFINED.bits() | Self::NULL.bits();
        const LITERAL = Self::STRING_LITERAL.bits() | Self::NUMBER_LITERAL.bits() |
                    Self::BIGINT_LITERAL.bits() | Self::BOOLEAN_LITERAL.bits();
        const UNIT = Self::ENUM.bits() | Self::LITERAL.bits() | Self::UNIQUE_ES_SYMBOL.bits() | Self::NULLABLE.bits();
        const FRESHABLE = Self::ENUM.bits() | Self::LITERAL.bits();
        const STRING_OR_NUMBER_LITERAL = Self::STRING_LITERAL.bits() | Self::NUMBER_LITERAL.bits();
        const STRING_OR_NUMBER_LITERAL_OR_UNIQUE = Self::STRING_LITERAL.bits() | Self::NUMBER_LITERAL.bits() | Self::UNIQUE_ES_SYMBOL.bits();

        // ... other compound flags would be defined similarly
    }
}

bitflags! {
    /// ObjectFlags for object types
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct ObjectFlags: u32 {
        const NONE = 0;
        const CLASS = 1 << 0;
        const INTERFACE = 1 << 1;
        const REFERENCE = 1 << 2;
        const TUPLE = 1 << 3;
        const ANONYMOUS = 1 << 4;
        const MAPPED = 1 << 5;
        const INSTANTIATED = 1 << 6;
        const OBJECT_LITERAL = 1 << 7;
        const EVOLVING_ARRAY = 1 << 8;
        const OBJECT_LITERAL_PATTERN_WITH_COMPUTED_PROPERTIES = 1 << 9;
        const REVERSE_MAPPED = 1 << 10;
        const JSX_ATTRIBUTES = 1 << 11;
        const JS_LITERAL = 1 << 12;
        const FRESH_LITERAL = 1 << 13;
        const ARRAY_LITERAL = 1 << 14;
        const PRIMITIVE_UNION = 1 << 15;
        const CONTAINS_WIDENING_TYPE = 1 << 16;
        const CONTAINS_OBJECT_OR_ARRAY_LITERAL = 1 << 17;
        const NON_INFERRABLE_TYPE = 1 << 18;
        const COULD_CONTAIN_TYPE_VARIABLES_COMPUTED = 1 << 19;
        const COULD_CONTAIN_TYPE_VARIABLES = 1 << 20;
        const MEMBERS_RESOLVED = 1 << 21;

        // Compound flags
        const CLASS_OR_INTERFACE = Self::CLASS.bits() | Self::INTERFACE.bits();
        const REQUIRES_WIDENING = Self::CONTAINS_WIDENING_TYPE.bits() | Self::CONTAINS_OBJECT_OR_ARRAY_LITERAL.bits();
        const PROPAGATING_FLAGS = Self::CONTAINS_WIDENING_TYPE.bits() | Self::CONTAINS_OBJECT_OR_ARRAY_LITERAL.bits() | Self::NON_INFERRABLE_TYPE.bits();
        const INSTANTIATED_MAPPED = Self::MAPPED.bits() | Self::INSTANTIATED.bits();

        // Object flags that uniquely identify the kind of ObjectType
        const OBJECT_TYPE_KIND_MASK = Self::CLASS_OR_INTERFACE.bits() | Self::REFERENCE.bits() | Self::TUPLE.bits() |
                                   Self::ANONYMOUS.bits() | Self::MAPPED.bits() | Self::REVERSE_MAPPED.bits() |
                                   Self::EVOLVING_ARRAY.bits();

        // ... other compound flags and purpose-specific flags would be defined similarly
    }
}

// Define the tristate enum for three-valued logic
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tristate {
    Unknown = 0,
    False = 1,
    True = 2,
}

/// Node ID (similar to TypeScript's NodeId)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NodeId(pub u32);

/// Symbol ID (similar to TypeScript's SymbolId)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SymbolId(pub u32);

/// Merge ID (similar to TypeScript's MergeId)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MergeId(pub u32);

/// Type ID (similar to TypeScript's TypeId)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TypeId(pub u32);

/// Core type representing a node in the AST
#[derive(Debug)]
pub struct Node {
    // These fields would depend on the specific node kind
    // For a generic node, we'd have common fields
    pub kind: SyntaxKind,
    pub flags: NodeFlags,
    pub parent: Option<Rc<Node>>,
    // ... other common fields
}

/// Symbol table mapping names to symbols
pub type SymbolTable = HashMap<String, Rc<Symbol>>;

/// Symbol representing a named entity in the program
#[derive(Debug)]
pub struct Symbol {
    pub flags: SymbolFlags,
    pub name: String,
    pub declarations: Vec<Rc<Node>>,
    pub value_declaration: Option<Rc<Node>>,
    pub members: Option<SymbolTable>,
    pub exports: Option<SymbolTable>,
    pub id: SymbolId,
    pub merge_id: Option<MergeId>,
    pub parent: Option<Rc<Symbol>>,
    pub export_symbol: Option<Rc<Symbol>>,
    pub assignment_declaration_members: Option<HashMap<NodeId, Rc<Node>>>,
    pub global_exports: Option<SymbolTable>,
    // ... other fields from the original Symbol struct
}

/// Type representing the core of the type system
#[derive(Debug)]
pub struct Type {
    pub flags: TypeFlags,
    pub object_flags: ObjectFlags,
    pub id: TypeId,
    pub symbol: Option<Rc<Symbol>>,
    pub data: Box<dyn TypeData>,
}

/// TypeData trait for different type representations
pub trait TypeData: fmt::Debug {
    fn as_object_type(&self) -> Option<&ObjectTypeBase> {
        None
    }
    fn as_parameterized_type(&self) -> Option<&ParameterizedTypeBase> {
        None
    }
}

/// Base for all type data
#[derive(Debug)]
pub struct TypeBase;

impl TypeData for TypeBase {}

/// Base for all object types
#[derive(Debug)]
pub struct ObjectTypeBase {
    pub members: Option<SymbolTable>,
    pub properties: Vec<Rc<Symbol>>,
    pub call_signatures: Vec<Rc<Signature>>,
    pub construct_signatures: Vec<Rc<Signature>>,
    pub index_infos: Vec<Rc<IndexInfo>>,
}

impl TypeData for ObjectTypeBase {
    fn as_object_type(&self) -> Option<&ObjectTypeBase> {
        Some(self)
    }
}

/// Signature representing function signature information
#[derive(Debug)]
pub struct Signature {
    // Fields would be filled in based on the TypeScript implementation
}

/// IndexInfo representing indexed access type information
#[derive(Debug)]
pub struct IndexInfo {
    pub key_type: Rc<Type>,
    pub value_type: Rc<Type>,
    pub is_readonly: bool,
    pub declaration: Option<Rc<Node>>,
}

/// Flow node for control flow analysis
#[derive(Debug)]
pub struct FlowNode {
    pub flags: FlowFlags,
    pub node: Option<Rc<Node>>,
    pub antecedent: Option<Rc<FlowNode>>,
    pub antecedents: Option<Rc<FlowList>>,
}

bitflags! {
    /// FlowFlags for control flow analysis
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct FlowFlags: u32 {
        const UNREACHABLE = 1 << 0;
        const START = 1 << 1;
        const BRANCH_LABEL = 1 << 2;
        const LOOP_LABEL = 1 << 3;
        const ASSIGNMENT = 1 << 4;
        const TRUE_CONDITION = 1 << 5;
        const FALSE_CONDITION = 1 << 6;
        const SWITCH_CLAUSE = 1 << 7;
        const ARRAY_MUTATION = 1 << 8;
        const CALL = 1 << 9;
        const REDUCE_LABEL = 1 << 10;
        const REFERENCED = 1 << 11;
        const SHARED = 1 << 12;

        const LABEL = Self::BRANCH_LABEL.bits() | Self::LOOP_LABEL.bits();
        const CONDITION = Self::TRUE_CONDITION.bits() | Self::FALSE_CONDITION.bits();
    }
}

/// Linked list of flow nodes
#[derive(Debug)]
pub struct FlowList {
    pub node: Rc<FlowNode>,
    pub next: Option<Rc<FlowList>>,
}

/// ParameterizedTypeBase for instantiated generic types
#[derive(Debug)]
pub struct ParameterizedTypeBase {
    pub object_base: ObjectTypeBase,
    pub target: Rc<Type>,
    pub resolved_type_arguments: Vec<Rc<Type>>,
}

impl TypeData for ParameterizedTypeBase {
    fn as_object_type(&self) -> Option<&ObjectTypeBase> {
        Some(&self.object_base)
    }

    fn as_parameterized_type(&self) -> Option<&ParameterizedTypeBase> {
        Some(self)
    }
}

// Additional type structs would be implemented similarly

/// TypeMapper trait for type instantiation
pub trait TypeMapper {
    fn map(&self, ty: &Type) -> Rc<Type>;
}
