use bitflags::bitflags;

bitflags! {
    /// Flags that control how nodes are processed by the type checker, emitter, and other systems
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct NodeFlags: u32 {
        /// No flags set
        const NONE = 0;
        /// Variable declaration using 'let'
        const LET = 1 << 0;
        /// Variable declaration using 'const'
        const CONST = 1 << 1;
        /// Variable declaration using 'using'
        const USING = 1 << 2;
        /// Node was synthesized during parsing
        const REPARSED = 1 << 3;
        /// Node was synthesized during transformation
        const SYNTHESIZED = 1 << 4;
        /// Chained MemberExpression rooted to a pseudo-OptionalExpression
        const OPTIONAL_CHAIN = 1 << 5;
        /// Export context (initialized by binding)
        const EXPORT_CONTEXT = 1 << 6;
        /// Interface contains references to "this"
        const CONTAINS_THIS = 1 << 7;
        /// Function implicitly returns on one of codepaths (initialized by binding)
        const HAS_IMPLICIT_RETURN = 1 << 8;
        /// Function has explicit reachable return on one of codepaths (initialized by binding)
        const HAS_EXPLICIT_RETURN = 1 << 9;
        /// Node was parsed in a context where 'in-expressions' are not allowed
        const DISALLOW_IN_CONTEXT = 1 << 10;
        /// Node was parsed in the 'yield' context created when parsing a generator
        const YIELD_CONTEXT = 1 << 11;
        /// Node was parsed as part of a decorator
        const DECORATOR_CONTEXT = 1 << 12;
        /// Node was parsed in the 'await' context created when parsing an async function
        const AWAIT_CONTEXT = 1 << 13;
        /// Node was parsed in a context where conditional types are not allowed
        const DISALLOW_CONDITIONAL_TYPES_CONTEXT = 1 << 14;
        /// Parser encountered an error when parsing the code that created this node
        const THIS_NODE_HAS_ERROR = 1 << 15;
        /// Node was parsed in a JavaScript file
        const JAVASCRIPT_FILE = 1 << 16;
        /// This node or any of its children had an error
        const THIS_NODE_OR_ANY_SUBNODE_HAS_ERROR = 1 << 17;
        /// Computed data from children has been cached in this node
        const HAS_AGGREGATED_CHILD_DATA = 1 << 18;
        /// Node might contain a dynamic import expression
        const POSSIBLY_CONTAINS_DYNAMIC_IMPORT = 1 << 19;
        /// Node might contain an import.meta expression
        const POSSIBLY_CONTAINS_IMPORT_META = 1 << 20;
        /// Node has preceding JSDoc comment(s)
        const HAS_JS_DOC = 1 << 21;
        /// Node was parsed inside JSDoc
        const JS_DOC = 1 << 22;
        /// Node was inside an ambient context (declaration file or 'declare' modifier)
        const AMBIENT = 1 << 23;
        /// Node was the 'statement' of a WithStatement (not the 'expression')
        const IN_WITH_STATEMENT = 1 << 24;
        /// Node was parsed in a JSON file
        const JSON_FILE = 1 << 25;
        /// Node has '@deprecated' JSDoc tag
        const DEPRECATED = 1 << 26;

        // Composite flags
        /// Block-scoped variable declaration (let, const, using)
        const BLOCK_SCOPED = Self::LET.bits() | Self::CONST.bits() | Self::USING.bits();
        /// Constant variable declaration (const, using)
        const CONSTANT = Self::CONST.bits() | Self::USING.bits();
        /// 'await using' variable declaration (special bit pattern)
        const AWAIT_USING = Self::CONST.bits() | Self::USING.bits();
        /// Flags for checking return statement reachability
        const REACHABILITY_CHECK_FLAGS = Self::HAS_IMPLICIT_RETURN.bits() | Self::HAS_EXPLICIT_RETURN.bits();
        /// All context-related parsing flags
        const CONTEXT_FLAGS = Self::DISALLOW_IN_CONTEXT.bits() |
                              Self::DISALLOW_CONDITIONAL_TYPES_CONTEXT.bits() |
                              Self::YIELD_CONTEXT.bits() |
                              Self::DECORATOR_CONTEXT.bits() |
                              Self::AWAIT_CONTEXT.bits() |
                              Self::JAVASCRIPT_FILE.bits() |
                              Self::IN_WITH_STATEMENT.bits() |
                              Self::AMBIENT.bits();
        /// Flags to exclude when parsing a Type
        const TYPE_EXCLUDES_FLAGS = Self::YIELD_CONTEXT.bits() | Self::AWAIT_CONTEXT.bits();
        /// Flags that are set once and never cleared during incremental parsing
        const PERMANENTLY_SET_INCREMENTAL_FLAGS = Self::POSSIBLY_CONTAINS_DYNAMIC_IMPORT.bits() |
                                                  Self::POSSIBLY_CONTAINS_IMPORT_META.bits();
    }
}

impl NodeFlags {
    /// Indicates whether the identifier contains an extended unicode escape sequence
    /// This repurposes CONTAINS_THIS for Identifier nodes
    pub const IDENTIFIER_HAS_EXTENDED_UNICODE_ESCAPE: NodeFlags = NodeFlags::CONTAINS_THIS;
}

impl Default for NodeFlags {
    fn default() -> Self {
        Self::NONE
    }
}
