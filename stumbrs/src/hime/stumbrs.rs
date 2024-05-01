//! Module for the lexer and parser for `Stumbrs`
//! WARNING: this file has been generated by
//! Hime Parser Generator 4.2.4

use std::io::Read;

use hime_redist::ast::AstNode;
use hime_redist::errors::ParseErrors;
use hime_redist::lexers::automaton::Automaton;
use hime_redist::lexers::impls::ContextFreeLexer;
use hime_redist::lexers::Lexer;
use hime_redist::parsers::lrk::LRkAutomaton;
use hime_redist::parsers::lrk::LRkParser;
use hime_redist::parsers::Parser;
use hime_redist::result::ParseResult;
use hime_redist::symbols::SemanticBody;
use hime_redist::symbols::SemanticElementTrait;
use hime_redist::symbols::Symbol;
use hime_redist::text::Text;
use hime_redist::tokens::TokenRepository;

/// Static resource for the serialized lexer automaton
const LEXER_AUTOMATON: &[u8] = include_bytes!("stumbrs_lexer.bin");

/// The unique identifier for terminal `SEPARATOR`
pub const ID_TERMINAL_SEPARATOR: u32 = 0x0007;
/// The unique identifier for terminal `ID`
pub const ID_TERMINAL_ID: u32 = 0x0008;
/// The unique identifier for terminal `QUOTE`
pub const ID_TERMINAL_QUOTE: u32 = 0x0009;
/// The unique identifier for terminal `NUMBER`
pub const ID_TERMINAL_NUMBER: u32 = 0x000B;
/// The unique identifier for terminal `STRING`
pub const ID_TERMINAL_STRING: u32 = 0x000C;
/// The unique identifier for terminal `CITADI`
pub const ID_TERMINAL_CITADI: u32 = 0x000D;
/// The unique identifier for terminal `ARRAY`
pub const ID_TERMINAL_ARRAY: u32 = 0x000E;
/// The unique identifier for terminal `RETURN`
pub const ID_TERMINAL_RETURN: u32 = 0x000F;

/// The unique identifier for the default context
pub const CONTEXT_DEFAULT: u16 = 0;

/// The collection of terminals matched by this lexer
/// The terminals are in an order consistent with the automaton,
/// so that terminal indices in the automaton can be used to retrieve the terminals in this table
pub const TERMINALS: &[Symbol] = &[
    Symbol {
        id: 0x0001,
        name: "ε"
    },
    Symbol {
        id: 0x0002,
        name: "$"
    },
    Symbol {
        id: 0x0007,
        name: "SEPARATOR"
    },
    Symbol {
        id: 0x0008,
        name: "ID"
    },
    Symbol {
        id: 0x0009,
        name: "QUOTE"
    },
    Symbol {
        id: 0x000B,
        name: "NUMBER"
    },
    Symbol {
        id: 0x000C,
        name: "STRING"
    },
    Symbol {
        id: 0x000D,
        name: "CITADI"
    },
    Symbol {
        id: 0x000E,
        name: "ARRAY"
    },
    Symbol {
        id: 0x000F,
        name: "RETURN"
    },
    Symbol {
        id: 0x001E,
        name: "būls"
    },
    Symbol {
        id: 0x001F,
        name: "bl"
    },
    Symbol {
        id: 0x0020,
        name: "teksts"
    },
    Symbol {
        id: 0x0021,
        name: "tk"
    },
    Symbol {
        id: 0x0022,
        name: "sk"
    },
    Symbol {
        id: 0x0023,
        name: "skaitlis"
    },
    Symbol {
        id: 0x0024,
        name: "PAT"
    },
    Symbol {
        id: 0x0025,
        name: "PATIESS"
    },
    Symbol {
        id: 0x0026,
        name: "NEPAT"
    },
    Symbol {
        id: 0x0027,
        name: "NEPATIESS"
    },
    Symbol {
        id: 0x0028,
        name: ";"
    },
    Symbol {
        id: 0x002A,
        name: "("
    },
    Symbol {
        id: 0x002B,
        name: ")"
    },
    Symbol {
        id: 0x002D,
        name: "["
    },
    Symbol {
        id: 0x002E,
        name: "]"
    },
    Symbol {
        id: 0x0030,
        name: "{"
    },
    Symbol {
        id: 0x0031,
        name: "}"
    }
];

/// Creates a new lexer
fn new_lexer<'a: 'b, 'b, 'c>(
    repository: TokenRepository<'a, 'b, 'c>,
    errors: &'c mut ParseErrors<'a>
) -> Lexer<'a, 'b, 'c> {
    let automaton = Automaton::new(LEXER_AUTOMATON);
    Lexer::ContextFree(ContextFreeLexer::new(repository, errors, automaton, 0x0007))
}

/// Static resource for the serialized parser automaton
const PARSER_AUTOMATON: &[u8] = include_bytes!("stumbrs_parser.bin");

/// The unique identifier for variable `BOOL_DEF`
pub const ID_VARIABLE_BOOL_DEF: u32 = 0x0010;
/// The unique identifier for variable `TEXT`
pub const ID_VARIABLE_TEXT: u32 = 0x0011;
/// The unique identifier for variable `NUM`
pub const ID_VARIABLE_NUM: u32 = 0x0012;
/// The unique identifier for variable `TYPE`
pub const ID_VARIABLE_TYPE: u32 = 0x0013;
/// The unique identifier for variable `TRUE`
pub const ID_VARIABLE_TRUE: u32 = 0x0014;
/// The unique identifier for variable `FALSE`
pub const ID_VARIABLE_FALSE: u32 = 0x0015;
/// The unique identifier for variable `BOOL`
pub const ID_VARIABLE_BOOL: u32 = 0x0016;
/// The unique identifier for variable `array`
pub const ID_VARIABLE_ARRAY: u32 = 0x0017;
/// The unique identifier for variable `exp_atom`
pub const ID_VARIABLE_EXP_ATOM: u32 = 0x0018;
/// The unique identifier for variable `SCHEM_ITEM`
pub const ID_VARIABLE_SCHEM_ITEM: u32 = 0x0019;
/// The unique identifier for variable `SCHEMATIC`
pub const ID_VARIABLE_SCHEMATIC: u32 = 0x001A;
/// The unique identifier for variable `VALUE`
pub const ID_VARIABLE_VALUE: u32 = 0x001B;
/// The unique identifier for variable `VALUES`
pub const ID_VARIABLE_VALUES: u32 = 0x001C;
/// The unique identifier for variable `root`
pub const ID_VARIABLE_ROOT: u32 = 0x001D;


/// The collection of variables matched by this parser
/// The variables are in an order consistent with the automaton,
/// so that variable indices in the automaton can be used to retrieve the variables in this table
pub const VARIABLES: &[Symbol] = &[
    Symbol {
        id: 0x0010,
        name: "BOOL_DEF"
    },
    Symbol {
        id: 0x0011,
        name: "TEXT"
    },
    Symbol {
        id: 0x0012,
        name: "NUM"
    },
    Symbol {
        id: 0x0013,
        name: "TYPE"
    },
    Symbol {
        id: 0x0014,
        name: "TRUE"
    },
    Symbol {
        id: 0x0015,
        name: "FALSE"
    },
    Symbol {
        id: 0x0016,
        name: "BOOL"
    },
    Symbol {
        id: 0x0017,
        name: "array"
    },
    Symbol {
        id: 0x0018,
        name: "exp_atom"
    },
    Symbol {
        id: 0x0019,
        name: "SCHEM_ITEM"
    },
    Symbol {
        id: 0x001A,
        name: "SCHEMATIC"
    },
    Symbol {
        id: 0x001B,
        name: "VALUE"
    },
    Symbol {
        id: 0x001C,
        name: "VALUES"
    },
    Symbol {
        id: 0x001D,
        name: "root"
    },
    Symbol {
        id: 0x0029,
        name: "__V41"
    },
    Symbol {
        id: 0x002C,
        name: "__V44"
    },
    Symbol {
        id: 0x002F,
        name: "__V47"
    },
    Symbol {
        id: 0x0032,
        name: "__VAxiom"
    }
];

/// The collection of virtuals matched by this parser
/// The virtuals are in an order consistent with the automaton,
/// so that virtual indices in the automaton can be used to retrieve the virtuals in this table
pub const VIRTUALS: &[Symbol] = &[

];

/// Parses the specified string with this parser
#[must_use]
pub fn parse_str(input: &str) -> ParseResult<'static, '_, 'static> {
    let text = Text::from_str(input);
    parse_text(text)
}

/// Parses the specified string with this parser
#[must_use]
pub fn parse_string(input: String) -> ParseResult<'static, 'static, 'static> {
    let text = Text::from_string(input);
    parse_text(text)
}

/// Parses the specified stream of UTF-8 with this parser
pub fn parse_utf8_stream(input: &mut dyn Read) -> ParseResult<'static, 'static, 'static> {
    let text = Text::from_utf8_stream(input).unwrap();
    parse_text(text)
}

/// Parses the specified text with this parser
fn parse_text(text: Text) -> ParseResult<'static, '_, 'static> {
    parse_text_with(text, TERMINALS, VARIABLES, VIRTUALS)
}

/// Parses the specified text with this parser
fn parse_text_with<'s, 't, 'a>(
    text: Text<'t>,
    terminals: &'a [Symbol<'s>],
    variables: &'a [Symbol<'s>],
    virtuals: &'a [Symbol<'s>]
) -> ParseResult<'s, 't, 'a> {
    let mut my_actions = |_index: usize, _head: Symbol, _body: &dyn SemanticBody| ();
    let mut result = ParseResult::new(terminals, variables, virtuals, text);
    {
        let data = result.get_parsing_data();
        let mut lexer = new_lexer(data.0, data.1);
        let automaton = LRkAutomaton::new(PARSER_AUTOMATON);
        let mut parser = LRkParser::new(&mut lexer, automaton, data.2, &mut my_actions);
        parser.parse();
    }
    result
}

/// Visitor interface
pub trait Visitor {
    fn on_terminal_separator(&self, _node: &AstNode) {}
    fn on_terminal_id(&self, _node: &AstNode) {}
    fn on_terminal_quote(&self, _node: &AstNode) {}
    fn on_terminal_number(&self, _node: &AstNode) {}
    fn on_terminal_string(&self, _node: &AstNode) {}
    fn on_terminal_citadi(&self, _node: &AstNode) {}
    fn on_terminal_array(&self, _node: &AstNode) {}
    fn on_terminal_return(&self, _node: &AstNode) {}
    fn on_variable_bool_def(&self, _node: &AstNode) {}
    fn on_variable_text(&self, _node: &AstNode) {}
    fn on_variable_num(&self, _node: &AstNode) {}
    fn on_variable_type(&self, _node: &AstNode) {}
    fn on_variable_true(&self, _node: &AstNode) {}
    fn on_variable_false(&self, _node: &AstNode) {}
    fn on_variable_bool(&self, _node: &AstNode) {}
    fn on_variable_array(&self, _node: &AstNode) {}
    fn on_variable_exp_atom(&self, _node: &AstNode) {}
    fn on_variable_schem_item(&self, _node: &AstNode) {}
    fn on_variable_schematic(&self, _node: &AstNode) {}
    fn on_variable_value(&self, _node: &AstNode) {}
    fn on_variable_values(&self, _node: &AstNode) {}
    fn on_variable_root(&self, _node: &AstNode) {}
}

/// Walk the AST of a result using a visitor
pub fn visit(result: &ParseResult, visitor: &dyn Visitor) {
    let ast = result.get_ast();
    let root = ast.get_root();
    visit_ast_node(root, visitor);
}

/// Walk the sub-AST from the specified node using a visitor
pub fn visit_ast_node(node: AstNode, visitor: &dyn Visitor) {
    let children = node.children();
    for child in children.iter() {
        visit_ast_node(child, visitor);
    }
    match node.get_symbol().id {
        0x0007 => visitor.on_terminal_separator(&node),
        0x0008 => visitor.on_terminal_id(&node),
        0x0009 => visitor.on_terminal_quote(&node),
        0x000B => visitor.on_terminal_number(&node),
        0x000C => visitor.on_terminal_string(&node),
        0x000D => visitor.on_terminal_citadi(&node),
        0x000E => visitor.on_terminal_array(&node),
        0x000F => visitor.on_terminal_return(&node),
        0x0010 => visitor.on_variable_bool_def(&node),
        0x0011 => visitor.on_variable_text(&node),
        0x0012 => visitor.on_variable_num(&node),
        0x0013 => visitor.on_variable_type(&node),
        0x0014 => visitor.on_variable_true(&node),
        0x0015 => visitor.on_variable_false(&node),
        0x0016 => visitor.on_variable_bool(&node),
        0x0017 => visitor.on_variable_array(&node),
        0x0018 => visitor.on_variable_exp_atom(&node),
        0x0019 => visitor.on_variable_schem_item(&node),
        0x001A => visitor.on_variable_schematic(&node),
        0x001B => visitor.on_variable_value(&node),
        0x001C => visitor.on_variable_values(&node),
        0x001D => visitor.on_variable_root(&node),
        _ => ()
    };
}
