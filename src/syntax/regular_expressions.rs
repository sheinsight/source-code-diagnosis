
use super::{compat::Compat, support::Support};

pub struct RegularExpressions<'a> {
  
      pub backreference: Compat<'a>,
    
      pub capturing_group: Compat<'a>,
    
      pub character_class: Compat<'a>,
    
      pub character_class_escape: Compat<'a>,
    
      pub character_escape: Compat<'a>,
    
      pub character_escape_unicode: Compat<'a>,
    
      pub disjunction: Compat<'a>,
    
      pub input_boundary_assertion: Compat<'a>,
    
      pub literal_character: Compat<'a>,
    
      pub lookahead_assertion: Compat<'a>,
    
      pub lookbehind_assertion: Compat<'a>,
    
      pub modifier: Compat<'a>,
    
      pub named_backreference: Compat<'a>,
    
      pub named_capturing_group: Compat<'a>,
    
      pub named_capturing_group_duplicate_named_capturing_groups: Compat<'a>,
    
      pub non_capturing_group: Compat<'a>,
    
      pub quantifier: Compat<'a>,
    
      pub unicode_character_class_escape: Compat<'a>,
    
      pub wildcard: Compat<'a>,
    
      pub word_boundary_assertion: Compat<'a>,
    
}

pub const REGULAR_EXPRESSIONS: RegularExpressions = RegularExpressions {
    
  backreference: Compat {
    name: "backreference",
    description: r##"Backreference: <code>\1</code>, <code>\2</code>"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Regular_expressions/Backreference",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#prod-DecimalEscape",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },
  capturing_group: Compat {
    name: "capturing_group",
    description: r##"Capturing group: <code>(...)</code>"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Regular_expressions/Capturing_group",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#prod-Atom",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },
  character_class: Compat {
    name: "character_class",
    description: r##"Character class: <code>[...]</code>, <code>[^...]</code>"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Regular_expressions/Character_class",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#prod-CharacterClass",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },
  character_class_escape: Compat {
    name: "character_class_escape",
    description: r##"Character class escape: <code>\d</code>, <code>\D</code>, <code>\w</code>, <code>\W</code>, <code>\s</code>, <code>\S</code>"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Regular_expressions/Character_class_escape",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#prod-CharacterClassEscape",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },
  character_escape: Compat {
    name: "character_escape",
    description: r##"Character escape: <code>\n</code>, <code>\x</code>"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Regular_expressions/Character_escape",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#prod-CharacterEscape",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },
  character_escape_unicode: Compat {
    name: "unicode",
    description: r##"Unicode character escape: <code>\u{...}</code>"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "50",
      edge: "12",
      firefox: "46",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },
  disjunction: Compat {
    name: "disjunction",
    description: r##"Disjunction: <code>|</code>"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Regular_expressions/Disjunction",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#prod-Disjunction",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },
  input_boundary_assertion: Compat {
    name: "input_boundary_assertion",
    description: r##"Input boundary assertion: <code>^</code>, <code>$</code>"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Regular_expressions/Input_boundary_assertion",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#prod-Assertion",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },
  literal_character: Compat {
    name: "literal_character",
    description: r##"Literal character: <code>a</code>, <code>b</code>"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Regular_expressions/Literal_character",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#prod-PatternCharacter",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },
  lookahead_assertion: Compat {
    name: "lookahead_assertion",
    description: r##"Lookahead assertion: <code>(?=...)</code>, <code>(?!...)</code>"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Regular_expressions/Lookahead_assertion",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#prod-Assertion",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },
  lookbehind_assertion: Compat {
    name: "lookbehind_assertion",
    description: r##"Lookbehind assertion: <code>(?&lt;=...)</code>, <code>(?&lt;!...)</code>"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Regular_expressions/Lookbehind_assertion",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#prod-Assertion",
    support: Support {
      chrome: "62",
      edge: "No",
      firefox: "78",
      ie: "false",
      node: "No",
      safari: "16.4",
    },
  },
  modifier: Compat {
    name: "modifier",
    description: r##"Modifier: <code>(?ims-ims:...)</code>"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Regular_expressions/Modifier",
    spec_url: "https://github.com/tc39/proposal-regexp-modifiers#syntax",
    support: Support {
      chrome: "125",
      edge: "No",
      firefox: "false",
      ie: "false",
      node: "No",
      safari: "false",
    },
  },
  named_backreference: Compat {
    name: "named_backreference",
    description: r##"Named backreference: <code>\k&lt;name&gt;</code>"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Regular_expressions/Named_backreference",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#prod-AtomEscape",
    support: Support {
      chrome: "64",
      edge: "No",
      firefox: "78",
      ie: "false",
      node: "No",
      safari: "11.1",
    },
  },
  named_capturing_group: Compat {
    name: "named_capturing_group",
    description: r##"Named capture group: <code>(?&lt;name&gt;...)</code>"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Regular_expressions/Named_capturing_group",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#prod-Atom",
    support: Support {
      chrome: "64",
      edge: "No",
      firefox: "78",
      ie: "false",
      node: "No",
      safari: "11.1",
    },
  },
  named_capturing_group_duplicate_named_capturing_groups: Compat {
    name: "duplicate_named_capturing_groups",
    description: r##"Duplicate names in different disjunction alternatives are allowed"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "125",
      edge: "No",
      firefox: "129",
      ie: "false",
      node: "No",
      safari: "17",
    },
  },
  non_capturing_group: Compat {
    name: "non_capturing_group",
    description: r##"Non-capturing group: <code>(?:...)</code>"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Regular_expressions/Non-capturing_group",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#prod-Atom",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },
  quantifier: Compat {
    name: "quantifier",
    description: r##"Quantifier: <code>*</code>, <code>+</code>, <code>?</code>, <code>{n}</code>, <code>{n,}</code>, <code>{n,m}</code>"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Regular_expressions/Quantifier",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#prod-Quantifier",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },
  unicode_character_class_escape: Compat {
    name: "unicode_character_class_escape",
    description: r##"Unicode character class escape: <code>\p{...}</code>, <code>\P{...}</code>"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Regular_expressions/Unicode_character_class_escape",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#prod-CharacterClassEscape",
    support: Support {
      chrome: "64",
      edge: "No",
      firefox: "78",
      ie: "false",
      node: "No",
      safari: "11.1",
    },
  },
  wildcard: Compat {
    name: "wildcard",
    description: r##"Wildcard: <code>.</code>"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Regular_expressions/Wildcard",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#prod-Atom",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },
  word_boundary_assertion: Compat {
    name: "word_boundary_assertion",
    description: r##"Word boundary assertion: <code>\b</code>, <code>\B</code>"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Regular_expressions/Word_boundary_assertion",
    spec_url: "https://tc39.es/ecma262/multipage/text-processing.html#prod-Assertion",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },
};