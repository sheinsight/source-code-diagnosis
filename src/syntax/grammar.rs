
use super::{compat::Compat, support::Support};

pub struct Grammar<'a>{
    
        pub array_literals: Compat<'a>,
      
        pub binary_numeric_literals: Compat<'a>,
      
        pub boolean_literals: Compat<'a>,
      
        pub decimal_numeric_literals: Compat<'a>,
      
        pub hash_bang_comments: Compat<'a>,
      
        pub hexadecimal_escape_sequences: Compat<'a>,
      
        pub hexadecimal_numeric_literals: Compat<'a>,
      
        pub null_literal: Compat<'a>,
      
        pub numeric_separators: Compat<'a>,
      
        pub octal_numeric_literals: Compat<'a>,
      
        pub regular_expression_literals: Compat<'a>,
      
        pub string_literals: Compat<'a>,
      
        pub unicode_escape_sequences: Compat<'a>,
      
        pub unicode_point_escapes: Compat<'a>,
      
        pub shorthand_object_literals: Compat<'a>,
      
        pub template_literals: Compat<'a>,
      
        pub template_literals_template_literal_revision: Compat<'a>,
      
        pub trailing_commas: Compat<'a>,
      
        pub trailing_commas_trailing_commas_in_dynamic_import: Compat<'a>,
      
        pub trailing_commas_trailing_commas_in_functions: Compat<'a>,
      
        pub trailing_commas_trailing_commas_in_object_literals: Compat<'a>,
      
}

pub const GRAMMAR: Grammar = Grammar { 
    
    array_literals: Compat {
      name: "array_literals",
      description: r##"Array literals (<code>[1, 2, 3]</code>)"##,
      mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Lexical_grammar#Array_literals",
      spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-array-initializer",
      support: Support {
        chrome: "1",
        edge: "12",
        firefox: "1",
        ie: "4",
        node: "No",
        safari: "1",
      },
    },

    binary_numeric_literals: Compat {
      name: "binary_numeric_literals",
      description: r##"Binary numeric literals (<code>0b</code>)"##,
      mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Lexical_grammar#Binary",
      spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-lexical-grammar.html#prod-BinaryIntegerLiteral",
      support: Support {
        chrome: "41",
        edge: "12",
        firefox: "25",
        ie: "false",
        node: "No",
        safari: "9",
      },
    },

    boolean_literals: Compat {
      name: "boolean_literals",
      description: r##"Boolean literals (<code>true</code>/<code>false</code>)"##,
      mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Lexical_grammar#Boolean_literal",
      spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-lexical-grammar.html#sec-boolean-literals",
      support: Support {
        chrome: "1",
        edge: "12",
        firefox: "1",
        ie: "3",
        node: "No",
        safari: "1",
      },
    },

    decimal_numeric_literals: Compat {
      name: "decimal_numeric_literals",
      description: r##"Decimal numeric literals (<code>1234567890</code>)"##,
      mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Lexical_grammar#Decimal",
      spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-lexical-grammar.html#prod-DecimalLiteral",
      support: Support {
        chrome: "1",
        edge: "12",
        firefox: "1",
        ie: "3",
        node: "No",
        safari: "1",
      },
    },

    hash_bang_comments: Compat {
      name: "hashbang_comments",
      description: r##"Hashbang (<code>#!</code>) comment syntax"##,
      mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Lexical_grammar#Hashbang_comments",
      spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-lexical-grammar.html#sec-hashbang",
      support: Support {
        chrome: "74",
        edge: "No",
        firefox: "67",
        ie: "false",
        node: "No",
        safari: "13.1",
      },
    },

    hexadecimal_escape_sequences: Compat {
      name: "hexadecimal_escape_sequences",
      description: r##"Hexadecimal escape sequences (<code>'\xA9'</code>)"##,
      mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Lexical_grammar#Hexadecimal_escape_sequences",
      spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-lexical-grammar.html#prod-HexEscapeSequence",
      support: Support {
        chrome: "1",
        edge: "12",
        firefox: "1",
        ie: "4",
        node: "No",
        safari: "1",
      },
    },

    hexadecimal_numeric_literals: Compat {
      name: "hexadecimal_numeric_literals",
      description: r##"Hexadecimal numeric literals (<code>0xAF</code>)"##,
      mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Lexical_grammar#Hexadecimal",
      spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-lexical-grammar.html#prod-HexIntegerLiteral",
      support: Support {
        chrome: "1",
        edge: "12",
        firefox: "1",
        ie: "3",
        node: "No",
        safari: "1",
      },
    },

    null_literal: Compat {
      name: "null_literal",
      description: r##"Null literal (<code>null</code>)"##,
      mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Lexical_grammar#Null_literal",
      spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-lexical-grammar.html#sec-null-literals",
      support: Support {
        chrome: "1",
        edge: "12",
        firefox: "1",
        ie: "3",
        node: "No",
        safari: "1",
      },
    },

    numeric_separators: Compat {
      name: "numeric_separators",
      description: r##"Numeric separators (<code>1_000_000_000_000</code>)"##,
      mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Lexical_grammar#Numeric_separators",
      spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-lexical-grammar.html#prod-NumericLiteralSeparator",
      support: Support {
        chrome: "75",
        edge: "No",
        firefox: "70",
        ie: "false",
        node: "No",
        safari: "13",
      },
    },

    octal_numeric_literals: Compat {
      name: "octal_numeric_literals",
      description: r##"Octal numeric literals (<code>0o</code>)"##,
      mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Lexical_grammar#Octal",
      spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-lexical-grammar.html#prod-OctalIntegerLiteral",
      support: Support {
        chrome: "41",
        edge: "12",
        firefox: "25",
        ie: "false",
        node: "No",
        safari: "9",
      },
    },

    regular_expression_literals: Compat {
      name: "regular_expression_literals",
      description: r##"Regular expression literals (<code>/ab+c/g</code>)"##,
      mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Lexical_grammar#Regular_expression_literals",
      spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-lexical-grammar.html#sec-literals-regular-expression-literals",
      support: Support {
        chrome: "1",
        edge: "12",
        firefox: "1",
        ie: "4",
        node: "No",
        safari: "1",
      },
    },

    string_literals: Compat {
      name: "string_literals",
      description: r##"String literals (<code>'Hello world'</code>)"##,
      mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Lexical_grammar#String_literals",
      spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-lexical-grammar.html#sec-literals-string-literals",
      support: Support {
        chrome: "1",
        edge: "12",
        firefox: "1",
        ie: "3",
        node: "No",
        safari: "1",
      },
    },

    unicode_escape_sequences: Compat {
      name: "unicode_escape_sequences",
      description: r##"Unicode escape sequences (<code>'\u00A9'</code>)"##,
      mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Lexical_grammar#Unicode_escape_sequences",
      spec_url: "https://tc39.es/ecma262/multipage/structured-data.html#sec-unicodeescape",
      support: Support {
        chrome: "1",
        edge: "12",
        firefox: "1",
        ie: "4",
        node: "No",
        safari: "1",
      },
    },

    unicode_point_escapes: Compat {
      name: "unicode_point_escapes",
      description: r##"Unicode point escapes (<code>\u{}</code>)"##,
      mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Lexical_grammar#Unicode_code_point_escapes",
      spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-lexical-grammar.html#prod-UnicodeEscapeSequence",
      support: Support {
        chrome: "44",
        edge: "12",
        firefox: "40",
        ie: "false",
        node: "No",
        safari: "9",
      },
    },

    shorthand_object_literals: Compat {
      name: "shorthand_object_literals",
      description: r##"Shorthand notation for object literals"##,
      mdn_url: "undefined",
      spec_url: "undefined",
      support: Support {
        chrome: "43",
        edge: "12",
        firefox: "33",
        ie: "false",
        node: "No",
        safari: "9",
      },
    },

    template_literals: Compat {
      name: "template_literals",
      description: r##"Template literals"##,
      mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Template_literals",
      spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-template-literals",
      support: Support {
        chrome: "41",
        edge: "12",
        firefox: "34",
        ie: "false",
        node: "No",
        safari: "9",
      },
    },

    template_literals_template_literal_revision: Compat {
      name: "template_literal_revision",
      description: r##"Escape sequences allowed in tagged template literals"##,
      mdn_url: "undefined",
      spec_url: "undefined",
      support: Support {
        chrome: "62",
        edge: "No",
        firefox: "53",
        ie: "false",
        node: "No",
        safari: "11",
      },
    },

    trailing_commas: Compat {
      name: "trailing_commas",
      description: r##"Trailing commas"##,
      mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Trailing_commas",
      spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#prod-Elision,https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#prod-ObjectLiteral,https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#prod-ArrayLiteral,https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#prod-Arguments,https://tc39.es/ecma262/multipage/ecmascript-language-functions-and-classes.html#prod-FormalParameters,https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#prod-CoverParenthesizedExpressionAndArrowParameterList,https://tc39.es/ecma262/multipage/ecmascript-language-scripts-and-modules.html#prod-NamedImports,https://tc39.es/ecma262/multipage/ecmascript-language-scripts-and-modules.html#prod-NamedExports,https://tc39.es/ecma262/multipage/text-processing.html#prod-QuantifierPrefix,https://tc39.es/ecma262/multipage/additional-ecmascript-features-for-web-browsers.html#prod-annexB-InvalidBracedQuantifier",
      support: Support {
        chrome: "1",
        edge: "12",
        firefox: "1",
        ie: "9",
        node: "No",
        safari: "1",
      },
    },

    trailing_commas_trailing_commas_in_dynamic_import: Compat {
      name: "trailing_commas_in_dynamic_import",
      description: r##"Trailing comma in dynamic import"##,
      mdn_url: "undefined",
      spec_url: "undefined",
      support: Support {
        chrome: "91",
        edge: "No",
        firefox: "false",
        ie: "false",
        node: "No",
        safari: "15",
      },
    },

    trailing_commas_trailing_commas_in_functions: Compat {
      name: "trailing_commas_in_functions",
      description: r##"Trailing comma in function parameters"##,
      mdn_url: "undefined",
      spec_url: "undefined",
      support: Support {
        chrome: "58",
        edge: "14",
        firefox: "52",
        ie: "false",
        node: "No",
        safari: "10",
      },
    },

    trailing_commas_trailing_commas_in_object_literals: Compat {
      name: "trailing_commas_in_object_literals",
      description: r##"Trailing comma in object literals"##,
      mdn_url: "undefined",
      spec_url: "undefined",
      support: Support {
        chrome: "1",
        edge: "12",
        firefox: "1",
        ie: "9",
        node: "No",
        safari: "3",
      },
    },
};
    