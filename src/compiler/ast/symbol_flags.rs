use bitflags::bitflags;

bitflags! {
    /// Flags used to track special properties of TypeScript symbols
    #[derive(Debug)]
    pub struct SymbolFlags: u32 {
        /// No flags
        const NONE = 0;
        /// Variable (var) or parameter
        const FUNCTION_SCOPED_VARIABLE = 1 << 0;
        /// A block-scoped variable (let or const)
        const BLOCK_SCOPED_VARIABLE = 1 << 1;
        /// Property or enum member
        const PROPERTY = 1 << 2;
        /// Enum member
        const ENUM_MEMBER = 1 << 3;
        /// Function
        const FUNCTION = 1 << 4;
        /// Class
        const CLASS = 1 << 5;
        /// Interface
        const INTERFACE = 1 << 6;
        /// Const enum
        const CONST_ENUM = 1 << 7;
        /// Enum
        const REGULAR_ENUM = 1 << 8;
        /// Instantiated module
        const VALUE_MODULE = 1 << 9;
        /// Uninstantiated module
        const NAMESPACE_MODULE = 1 << 10;
        /// Type Literal or mapped type
        const TYPE_LITERAL = 1 << 11;
        /// Object Literal
        const OBJECT_LITERAL = 1 << 12;
        /// Method
        const METHOD = 1 << 13;
        /// Constructor
        const CONSTRUCTOR = 1 << 14;
        /// Get accessor
        const GET_ACCESSOR = 1 << 15;
        /// Set accessor
        const SET_ACCESSOR = 1 << 16;
        /// Call, construct, or index signature
        const SIGNATURE = 1 << 17;
        /// Type parameter
        const TYPE_PARAMETER = 1 << 18;
        /// Type alias
        const TYPE_ALIAS = 1 << 19;
        /// Exported value marker
        const EXPORT_VALUE = 1 << 20;
        /// An alias for another symbol
        const ALIAS = 1 << 21;
        /// Prototype property (no source representation)
        const PROTOTYPE = 1 << 22;
        /// Export * declaration
        const EXPORT_STAR = 1 << 23;
        /// Optional property
        const OPTIONAL = 1 << 24;
        /// Transient symbol (created during type check)
        const TRANSIENT = 1 << 25;
        /// Assignment to property on function acting as declaration (eg `func.prop = 1`)
        const ASSIGNMENT = 1 << 26;
        /// Symbol for CommonJS `module` of `module.exports`
        const MODULE_EXPORTS = 1 << 27;
        /// Module contains only const enums or other modules with only const enums
        const CONST_ENUM_ONLY_MODULE = 1 << 28;
        /// Symbol that can be replaced by a method
        const REPLACEABLE_BY_METHOD = 1 << 29;
        /// Flag to signal this is a global lookup
        const GLOBAL_LOOKUP = 1 << 30;
        /// All flags except GLOBAL_LOOKUP
        const ALL = (1 << 30) - 1;

        // Composite flags
        /// Regular and const enums
        const ENUM = Self::REGULAR_ENUM.bits() | Self::CONST_ENUM.bits();
        /// Function and block scoped variables
        const VARIABLE = Self::FUNCTION_SCOPED_VARIABLE.bits() | Self::BLOCK_SCOPED_VARIABLE.bits();
        /// All value space symbols
        const VALUE = Self::VARIABLE.bits() | Self::PROPERTY.bits() | Self::ENUM_MEMBER.bits() |
                      Self::OBJECT_LITERAL.bits() | Self::FUNCTION.bits() | Self::CLASS.bits() |
                      Self::ENUM.bits() | Self::VALUE_MODULE.bits() | Self::METHOD.bits() |
                      Self::GET_ACCESSOR.bits() | Self::SET_ACCESSOR.bits();
        /// All type space symbols
        const TYPE = Self::CLASS.bits() | Self::INTERFACE.bits() | Self::ENUM.bits() |
                     Self::ENUM_MEMBER.bits() | Self::TYPE_LITERAL.bits() |
                     Self::TYPE_PARAMETER.bits() | Self::TYPE_ALIAS.bits();
        /// All namespace symbols
        const NAMESPACE = Self::VALUE_MODULE.bits() | Self::NAMESPACE_MODULE.bits() | Self::ENUM.bits();
        /// All module symbols
        const MODULE = Self::VALUE_MODULE.bits() | Self::NAMESPACE_MODULE.bits();
        /// All accessor symbols
        const ACCESSOR = Self::GET_ACCESSOR.bits() | Self::SET_ACCESSOR.bits();

        // Exclusion flags - symbols that can't be merged with particular kinds

        /// Variables can be redeclared, but can not redeclare a block-scoped declaration with the
        /// same name, or any other value that is not a variable
        const FUNCTION_SCOPED_VARIABLE_EXCLUDES = Self::VALUE.bits() & !(Self::FUNCTION_SCOPED_VARIABLE.bits());

        /// Block-scoped declarations are not allowed to be re-declared
        /// they can not merge with anything in the value space
        const BLOCK_SCOPED_VARIABLE_EXCLUDES = Self::VALUE.bits();

        const PARAMETER_EXCLUDES = Self::VALUE.bits();
        const PROPERTY_EXCLUDES = Self::VALUE.bits() & !(Self::PROPERTY.bits());
        const ENUM_MEMBER_EXCLUDES = Self::VALUE.bits() | Self::TYPE.bits();
        const FUNCTION_EXCLUDES = Self::VALUE.bits() & !(Self::FUNCTION.bits() | Self::VALUE_MODULE.bits() | Self::CLASS.bits());
        /// Class-interface mergability done in type checker
        const CLASS_EXCLUDES = (Self::VALUE.bits() | Self::TYPE.bits()) &
                              !(Self::VALUE_MODULE.bits() | Self::INTERFACE.bits() | Self::FUNCTION.bits());
        const INTERFACE_EXCLUDES = Self::TYPE.bits() & !(Self::INTERFACE.bits() | Self::CLASS.bits());
        /// Regular enums merge only with regular enums and modules
        const REGULAR_ENUM_EXCLUDES = (Self::VALUE.bits() | Self::TYPE.bits()) &
                                     !(Self::REGULAR_ENUM.bits() | Self::VALUE_MODULE.bits());
        /// Const enums merge only with const enums
        const CONST_ENUM_EXCLUDES = (Self::VALUE.bits() | Self::TYPE.bits()) & !(Self::CONST_ENUM.bits());
        const VALUE_MODULE_EXCLUDES = Self::VALUE.bits() &
                                     !(Self::FUNCTION.bits() | Self::CLASS.bits() | Self::REGULAR_ENUM.bits() | Self::VALUE_MODULE.bits());
        const NAMESPACE_MODULE_EXCLUDES = Self::NONE.bits();
        const METHOD_EXCLUDES = Self::VALUE.bits() & !(Self::METHOD.bits());
        const GET_ACCESSOR_EXCLUDES = Self::VALUE.bits() & !(Self::SET_ACCESSOR.bits());
        const SET_ACCESSOR_EXCLUDES = Self::VALUE.bits() & !(Self::GET_ACCESSOR.bits());
        const ACCESSOR_EXCLUDES = Self::VALUE.bits() & !(Self::ACCESSOR.bits());
        const TYPE_PARAMETER_EXCLUDES = Self::TYPE.bits() & !(Self::TYPE_PARAMETER.bits());
        const TYPE_ALIAS_EXCLUDES = Self::TYPE.bits();
        const ALIAS_EXCLUDES = Self::ALIAS.bits();

        // Other composite flags
        const MODULE_MEMBER = Self::VARIABLE.bits() | Self::FUNCTION.bits() | Self::CLASS.bits() |
                              Self::INTERFACE.bits() | Self::ENUM.bits() | Self::MODULE.bits() |
                              Self::TYPE_ALIAS.bits() | Self::ALIAS.bits();
        const EXPORT_HAS_LOCAL = Self::FUNCTION.bits() | Self::CLASS.bits() | Self::ENUM.bits() | Self::VALUE_MODULE.bits();
        const BLOCK_SCOPED = Self::BLOCK_SCOPED_VARIABLE.bits() | Self::CLASS.bits() | Self::ENUM.bits();
        const PROPERTY_OR_ACCESSOR = Self::PROPERTY.bits() | Self::ACCESSOR.bits();
        const CLASS_MEMBER = Self::METHOD.bits() | Self::ACCESSOR.bits() | Self::PROPERTY.bits();
        const EXPORT_SUPPORTS_DEFAULT_MODIFIER = Self::CLASS.bits() | Self::FUNCTION.bits() | Self::INTERFACE.bits();
        /// The set of things we consider semantically classifiable.
        /// Used to speed up the language service during classification.
        const CLASSIFIABLE = Self::CLASS.bits() | Self::ENUM.bits() | Self::TYPE_ALIAS.bits() |
                             Self::INTERFACE.bits() | Self::TYPE_PARAMETER.bits() |
                             Self::MODULE.bits() | Self::ALIAS.bits();
        const LATE_BINDING_CONTAINER = Self::CLASS.bits() | Self::INTERFACE.bits() |
                                       Self::TYPE_LITERAL.bits() | Self::OBJECT_LITERAL.bits() |
                                       Self::FUNCTION.bits();
    }
}

impl Default for SymbolFlags {
    fn default() -> Self {
        Self::NONE
    }
}

impl SymbolFlags {
    /// Anything that does not support default export modifier
    pub const EXPORT_DOES_NOT_SUPPORT_DEFAULT_MODIFIER: u32 =
        !Self::EXPORT_SUPPORTS_DEFAULT_MODIFIER.bits();
}
