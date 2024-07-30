
use super::{compat::Compat, support::Support};

pub struct Operators<'a>{
    
      pub addition: Compat<'a>,
    
      pub addition_assignment: Compat<'a>,
    
      pub assignment: Compat<'a>,
    
      pub async_function: Compat<'a>,
    
      pub async_generator_function: Compat<'a>,
    
      pub r#await: Compat<'a>,
    
      pub r#await_top_level: Compat<'a>,
    
      pub bitwise_and: Compat<'a>,
    
      pub bitwise_and_assignment: Compat<'a>,
    
      pub bitwise_not: Compat<'a>,
    
      pub bitwise_or: Compat<'a>,
    
      pub bitwise_or_assignment: Compat<'a>,
    
      pub bitwise_xor: Compat<'a>,
    
      pub bitwise_xor_assignment: Compat<'a>,
    
      pub class: Compat<'a>,
    
      pub comma: Compat<'a>,
    
      pub conditional: Compat<'a>,
    
      pub decrement: Compat<'a>,
    
      pub delete: Compat<'a>,
    
      pub destructuring: Compat<'a>,
    
      pub destructuring_computed_property_names: Compat<'a>,
    
      pub destructuring_rest_in_arrays: Compat<'a>,
    
      pub destructuring_rest_in_objects: Compat<'a>,
    
      pub division: Compat<'a>,
    
      pub division_assignment: Compat<'a>,
    
      pub equality: Compat<'a>,
    
      pub exponentiation: Compat<'a>,
    
      pub exponentiation_assignment: Compat<'a>,
    
      pub function: Compat<'a>,
    
      pub function_trailing_comma: Compat<'a>,
    
      pub generator_function: Compat<'a>,
    
      pub generator_function_trailing_comma: Compat<'a>,
    
      pub greater_than: Compat<'a>,
    
      pub greater_than_or_equal: Compat<'a>,
    
      pub grouping: Compat<'a>,
    
      pub import: Compat<'a>,
    
      pub import_worker_support: Compat<'a>,
    
      pub import_options_parameter: Compat<'a>,
    
      pub import_meta: Compat<'a>,
    
      pub import_meta_resolve: Compat<'a>,
    
      pub r#in: Compat<'a>,
    
      pub increment: Compat<'a>,
    
      pub inequality: Compat<'a>,
    
      pub instance_of: Compat<'a>,
    
      pub left_shift: Compat<'a>,
    
      pub left_shift_assignment: Compat<'a>,
    
      pub less_than: Compat<'a>,
    
      pub less_than_or_equal: Compat<'a>,
    
      pub logical_and: Compat<'a>,
    
      pub logical_and_assignment: Compat<'a>,
    
      pub logical_not: Compat<'a>,
    
      pub logical_or: Compat<'a>,
    
      pub logical_or_assignment: Compat<'a>,
    
      pub multiplication: Compat<'a>,
    
      pub multiplication_assignment: Compat<'a>,
    
      pub new: Compat<'a>,
    
      pub new_target: Compat<'a>,
    
      pub null: Compat<'a>,
    
      pub nullish_coalescing: Compat<'a>,
    
      pub nullish_coalescing_assignment: Compat<'a>,
    
      pub object_initializer: Compat<'a>,
    
      pub object_initializer_computed_property_names: Compat<'a>,
    
      pub object_initializer_shorthand_method_names: Compat<'a>,
    
      pub object_initializer_shorthand_property_names: Compat<'a>,
    
      pub object_initializer_spread_properties: Compat<'a>,
    
      pub optional_chaining: Compat<'a>,
    
      pub property_accessors: Compat<'a>,
    
      pub remainder: Compat<'a>,
    
      pub remainder_assignment: Compat<'a>,
    
      pub right_shift: Compat<'a>,
    
      pub right_shift_assignment: Compat<'a>,
    
      pub spread: Compat<'a>,
    
      pub spread_spread_in_arrays: Compat<'a>,
    
      pub spread_spread_in_function_calls: Compat<'a>,
    
      pub spread_spread_in_object_literals: Compat<'a>,
    
      pub strict_equality: Compat<'a>,
    
      pub strict_inequality: Compat<'a>,
    
      pub subtraction: Compat<'a>,
    
      pub subtraction_assignment: Compat<'a>,
    
      pub r#super_alias: Compat<'a>,
    
      pub this: Compat<'a>,
    
      pub r#typeof: Compat<'a>,
    
      pub unary_negation: Compat<'a>,
    
      pub unary_plus: Compat<'a>,
    
      pub unsigned_right_shift: Compat<'a>,
    
      pub unsigned_right_shift_assignment: Compat<'a>,
    
      pub r#void: Compat<'a>,
    
      pub r#yield: Compat<'a>,
    
      pub yield_star: Compat<'a>,
    
}

pub const OPERATORS: Operators = Operators { 
    
  addition: Compat {
    name: "addition",
    description: r##"Addition (<code>+</code>)"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Addition",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-addition-operator-plus",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  addition_assignment: Compat {
    name: "addition_assignment",
    description: r##"Addition assignment (<code>x += y</code>)"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Addition_assignment",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-assignment-operators",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  assignment: Compat {
    name: "assignment",
    description: r##"Assignment (<code>x = y</code>)"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Assignment",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-assignment-operators",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  async_function: Compat {
    name: "async_function",
    description: r##"<code>async function</code> expression"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/async_function",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-functions-and-classes.html#sec-async-function-definitions",
    support: Support {
      chrome: "55",
      edge: "15",
      firefox: "52",
      ie: "false",
      node: "No",
      safari: "10.1",
    },
  },

  async_generator_function: Compat {
    name: "async_generator_function",
    description: r##"<code>async function*</code> expression"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/async_function*",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-functions-and-classes.html#sec-async-generator-function-definitions",
    support: Support {
      chrome: "63",
      edge: "No",
      firefox: "55",
      ie: "false",
      node: "No",
      safari: "12",
    },
  },

  r#await: Compat {
    name: "await",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/await",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-functions-and-classes.html#sec-async-function-definitions",
    support: Support {
      chrome: "55",
      edge: "14",
      firefox: "52",
      ie: "false",
      node: "No",
      safari: "10.1",
    },
  },

  r#await_top_level: Compat {
    name: "top_level",
    description: r##"Use at module top level"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/await#top_level_await",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-functions-and-classes.html#sec-async-function-definitions",
    support: Support {
      chrome: "89",
      edge: "No",
      firefox: "89",
      ie: "false",
      node: "No",
      safari: "15",
    },
  },

  bitwise_and: Compat {
    name: "bitwise_and",
    description: r##"Bitwise AND (<code>a &amp; b</code>)"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Bitwise_AND",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#prod-BitwiseANDExpression",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  bitwise_and_assignment: Compat {
    name: "bitwise_and_assignment",
    description: r##"Bitwise AND assignment (<code>x &amp;= y</code>)"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Bitwise_AND_assignment",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-assignment-operators",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  bitwise_not: Compat {
    name: "bitwise_not",
    description: r##"Bitwise NOT (<code>~a</code>)"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Bitwise_NOT",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-bitwise-not-operator",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  bitwise_or: Compat {
    name: "bitwise_or",
    description: r##"Bitwise OR (<code>a | b</code>)"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Bitwise_OR",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#prod-BitwiseORExpression",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  bitwise_or_assignment: Compat {
    name: "bitwise_or_assignment",
    description: r##"Bitwise OR assignment (<code>x |= y</code>)"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Bitwise_OR_assignment",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-assignment-operators",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  bitwise_xor: Compat {
    name: "bitwise_xor",
    description: r##"Bitwise XOR (<code>a ^ b</code>)"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Bitwise_XOR",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#prod-BitwiseXORExpression",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  bitwise_xor_assignment: Compat {
    name: "bitwise_xor_assignment",
    description: r##"Bitwise XOR assignment (<code>x ^= y</code>)"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Bitwise_XOR_assignment",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-assignment-operators",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  class: Compat {
    name: "class",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/class",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-functions-and-classes.html#sec-class-definitions",
    support: Support {
      chrome: "42",
      edge: "13",
      firefox: "45",
      ie: "false",
      node: "No",
      safari: "7",
    },
  },

  comma: Compat {
    name: "comma",
    description: r##"Comma operator"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Comma_operator",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-comma-operator",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  conditional: Compat {
    name: "conditional",
    description: r##"Conditional operator (<code>c ? t : f</code>)"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Conditional_operator",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-conditional-operator",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  decrement: Compat {
    name: "decrement",
    description: r##"Decrement (<code>--</code>)"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Decrement",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-postfix-decrement-operator",
    support: Support {
      chrome: "2",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "4",
    },
  },

  delete: Compat {
    name: "delete",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/delete",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-delete-operator",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  destructuring: Compat {
    name: "destructuring",
    description: r##"Destructuring assignment"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Destructuring_assignment",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-destructuring-assignment,https://tc39.es/ecma262/multipage/ecmascript-language-statements-and-declarations.html#sec-destructuring-binding-patterns",
    support: Support {
      chrome: "49",
      edge: "14",
      firefox: "41",
      ie: "false",
      node: "No",
      safari: "8",
    },
  },

  destructuring_computed_property_names: Compat {
    name: "computed_property_names",
    description: r##"Computed property names"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "49",
      edge: "14",
      firefox: "41",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  destructuring_rest_in_arrays: Compat {
    name: "rest_in_arrays",
    description: r##"Rest in arrays"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "49",
      edge: "16",
      firefox: "41",
      ie: "false",
      node: "No",
      safari: "9.1",
    },
  },

  destructuring_rest_in_objects: Compat {
    name: "rest_in_objects",
    description: r##"Rest in objects"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "60",
      edge: "No",
      firefox: "55",
      ie: "false",
      node: "No",
      safari: "11.1",
    },
  },

  division: Compat {
    name: "division",
    description: r##"Division (<code>/</code>)"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Division",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-multiplicative-operators",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  division_assignment: Compat {
    name: "division_assignment",
    description: r##"Division assignment (<code>x /= y</code>)"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Division_assignment",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-assignment-operators",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  equality: Compat {
    name: "equality",
    description: r##"Equality (<code>a == b</code>)"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Equality",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-equality-operators",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  exponentiation: Compat {
    name: "exponentiation",
    description: r##"Exponentiation (<code>**</code>)"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Exponentiation",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-exp-operator",
    support: Support {
      chrome: "52",
      edge: "14",
      firefox: "52",
      ie: "false",
      node: "No",
      safari: "10.1",
    },
  },

  exponentiation_assignment: Compat {
    name: "exponentiation_assignment",
    description: r##"Exponentiation assignment (<code>x **= y</code>)"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Exponentiation_assignment",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-assignment-operators",
    support: Support {
      chrome: "52",
      edge: "14",
      firefox: "52",
      ie: "false",
      node: "No",
      safari: "10.1",
    },
  },

  function: Compat {
    name: "function",
    description: r##"<code>function</code> expression"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/function",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-functions-and-classes.html#sec-function-definitions",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  function_trailing_comma: Compat {
    name: "trailing_comma",
    description: r##"Trailing comma in parameters"##,
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

  generator_function: Compat {
    name: "generator_function",
    description: r##"<code>function*</code> expression"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/function*",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-functions-and-classes.html#sec-generator-function-definitions",
    support: Support {
      chrome: "49",
      edge: "12",
      firefox: "26",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  generator_function_trailing_comma: Compat {
    name: "trailing_comma",
    description: r##"Trailing comma in parameters"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "58",
      edge: "No",
      firefox: "52",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  greater_than: Compat {
    name: "greater_than",
    description: r##"Greater than (<code>a &gt; b</code>)"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Greater_than",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-relational-operators",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  greater_than_or_equal: Compat {
    name: "greater_than_or_equal",
    description: r##"Greater than or equal (<code>a &gt;= b</code>)"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Greater_than_or_equal",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-relational-operators",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  grouping: Compat {
    name: "grouping",
    description: r##"Grouping operator <code>()</code>"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Grouping",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-grouping-operator",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  import: Compat {
    name: "import",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/import",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-import-calls",
    support: Support {
      chrome: "63",
      edge: "No",
      firefox: "67",
      ie: "false",
      node: "No",
      safari: "11.1",
    },
  },

  import_worker_support: Compat {
    name: "worker_support",
    description: r##"Available in workers"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "80",
      edge: "No",
      firefox: "114",
      ie: "false",
      node: "No",
      safari: "15",
    },
  },

  import_options_parameter: Compat {
    name: "options_parameter",
    description: r##"The <code>options</code> parameter"##,
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

  import_meta: Compat {
    name: "import_meta",
    description: r##"<code>import.meta</code>"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/import.meta",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#prod-ImportMeta,https://html.spec.whatwg.org/multipage/webappapis.html#hostgetimportmetaproperties",
    support: Support {
      chrome: "64",
      edge: "No",
      firefox: "62",
      ie: "false",
      node: "No",
      safari: "11.1",
    },
  },

  import_meta_resolve: Compat {
    name: "resolve",
    description: r##"<code>import.meta.resolve</code>"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/import.meta/resolve",
    spec_url: "https://html.spec.whatwg.org/multipage/webappapis.html#hostgetimportmetaproperties",
    support: Support {
      chrome: "105",
      edge: "No",
      firefox: "106",
      ie: "false",
      node: "No",
      safari: "16.4",
    },
  },

  r#in: Compat {
    name: "in",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/in",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-relational-operators",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "5.5",
      node: "No",
      safari: "1",
    },
  },

  increment: Compat {
    name: "increment",
    description: r##"Increment (<code>++</code>)"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Increment",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-postfix-increment-operator",
    support: Support {
      chrome: "2",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "4",
    },
  },

  inequality: Compat {
    name: "inequality",
    description: r##"Inequality (<code>a != b</code>)"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Inequality",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-equality-operators",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  instance_of: Compat {
    name: "instanceof",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/instanceof",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-relational-operators",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "5",
      node: "No",
      safari: "1",
    },
  },

  left_shift: Compat {
    name: "left_shift",
    description: r##"Bitwise left shift (<code>a &lt;&lt; b</code>)"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Left_shift",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-left-shift-operator",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  left_shift_assignment: Compat {
    name: "left_shift_assignment",
    description: r##"Left shift assignment (<code>x &lt;&lt;= y</code>)"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Left_shift_assignment",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-assignment-operators",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  less_than: Compat {
    name: "less_than",
    description: r##"Less than (<code>a &lt; b</code>)"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Less_than",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-relational-operators",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  less_than_or_equal: Compat {
    name: "less_than_or_equal",
    description: r##"Less than or equal (<code>a &lt;= b</code>)"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Less_than_or_equal",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-relational-operators",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  logical_and: Compat {
    name: "logical_and",
    description: r##"Logical AND (<code>&amp;&amp;</code>)"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Logical_AND",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#prod-LogicalANDExpression",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  logical_and_assignment: Compat {
    name: "logical_and_assignment",
    description: r##"Logical AND assignment (<code>x &amp;&amp;= y</code>)"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Logical_AND_assignment",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-assignment-operators",
    support: Support {
      chrome: "85",
      edge: "No",
      firefox: "79",
      ie: "false",
      node: "No",
      safari: "14",
    },
  },

  logical_not: Compat {
    name: "logical_not",
    description: r##"Logical NOT (<code>!</code>)"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Logical_NOT",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-logical-not-operator",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  logical_or: Compat {
    name: "logical_or",
    description: r##"Logical OR (<code>||</code>)"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Logical_OR",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#prod-LogicalORExpression",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  logical_or_assignment: Compat {
    name: "logical_or_assignment",
    description: r##"Logical OR assignment (<code>x ||= y</code>)"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Logical_OR_assignment",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-assignment-operators",
    support: Support {
      chrome: "85",
      edge: "No",
      firefox: "79",
      ie: "false",
      node: "No",
      safari: "14",
    },
  },

  multiplication: Compat {
    name: "multiplication",
    description: r##"Multiplication (<code>*</code>)"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Multiplication",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-multiplicative-operators",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  multiplication_assignment: Compat {
    name: "multiplication_assignment",
    description: r##"Multiplication assignment (<code>x *= y</code>)"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Multiplication_assignment",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-assignment-operators",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  new: Compat {
    name: "new",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/new",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-new-operator",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  new_target: Compat {
    name: "new_target",
    description: r##"<code>new.target</code>"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/new.target",
    spec_url: "https://tc39.es/ecma262/multipage/ordinary-and-exotic-objects-behaviours.html#sec-built-in-function-objects",
    support: Support {
      chrome: "46",
      edge: "13",
      firefox: "41",
      ie: "false",
      node: "No",
      safari: "11",
    },
  },

  null: Compat {
    name: "null",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/null",
    spec_url: "https://tc39.es/ecma262/multipage/overview.html#sec-null-value",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  nullish_coalescing: Compat {
    name: "nullish_coalescing",
    description: r##"Nullish coalescing operator (<code>??</code>)"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Nullish_coalescing",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#prod-CoalesceExpression",
    support: Support {
      chrome: "80",
      edge: "No",
      firefox: "72",
      ie: "false",
      node: "No",
      safari: "13.1",
    },
  },

  nullish_coalescing_assignment: Compat {
    name: "nullish_coalescing_assignment",
    description: r##"Nullish coalescing assignment (<code>x ??= y</code>)"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Nullish_coalescing_assignment",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-assignment-operators",
    support: Support {
      chrome: "85",
      edge: "No",
      firefox: "79",
      ie: "false",
      node: "No",
      safari: "14",
    },
  },

  object_initializer: Compat {
    name: "object_initializer",
    description: r##"Object initializer"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Object_initializer",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-object-initializer",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "1",
      node: "No",
      safari: "1",
    },
  },

  object_initializer_computed_property_names: Compat {
    name: "computed_property_names",
    description: r##"Computed property names"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "47",
      edge: "12",
      firefox: "34",
      ie: "false",
      node: "No",
      safari: "8",
    },
  },

  object_initializer_shorthand_method_names: Compat {
    name: "shorthand_method_names",
    description: r##"Shorthand method names"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "47",
      edge: "12",
      firefox: "34",
      ie: "false",
      node: "No",
      safari: "9",
    },
  },

  object_initializer_shorthand_property_names: Compat {
    name: "shorthand_property_names",
    description: r##"Shorthand property names"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "47",
      edge: "12",
      firefox: "33",
      ie: "false",
      node: "No",
      safari: "9",
    },
  },

  object_initializer_spread_properties: Compat {
    name: "spread_properties",
    description: r##"Spread properties"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "60",
      edge: "No",
      firefox: "55",
      ie: "false",
      node: "No",
      safari: "11.1",
    },
  },

  optional_chaining: Compat {
    name: "optional_chaining",
    description: r##"Optional chaining operator (<code>?.</code>)"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Optional_chaining",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#prod-OptionalExpression",
    support: Support {
      chrome: "80",
      edge: "No",
      firefox: "74",
      ie: "false",
      node: "No",
      safari: "13.1",
    },
  },

  property_accessors: Compat {
    name: "property_accessors",
    description: r##"Property accessors"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Property_accessors",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-property-accessors",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  remainder: Compat {
    name: "remainder",
    description: r##"Remainder (<code>%</code>)"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Remainder",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-multiplicative-operators",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  remainder_assignment: Compat {
    name: "remainder_assignment",
    description: r##"Remainder assignment (<code>x %= y</code>)"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Remainder_assignment",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-assignment-operators",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  right_shift: Compat {
    name: "right_shift",
    description: r##"Bitwise right shift (<code>a &gt;&gt; b</code>)"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Right_shift",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-signed-right-shift-operator",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  right_shift_assignment: Compat {
    name: "right_shift_assignment",
    description: r##"Right shift assignment (<code>x &gt;&gt;= y</code>)"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Right_shift_assignment",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-assignment-operators",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  spread: Compat {
    name: "spread",
    description: r##"Spread syntax (...)"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Spread_syntax",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#prod-SpreadElement,https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#prod-ArgumentList,https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#prod-PropertyDefinition",
    support: Support {
      chrome: "46",
      edge: "12",
      firefox: "16",
      ie: "false",
      node: "No",
      safari: "8",
    },
  },

  spread_spread_in_arrays: Compat {
    name: "spread_in_arrays",
    description: r##"Spread in array literals"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Spread_syntax#Spread_in_array_literals",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#prod-SpreadElement",
    support: Support {
      chrome: "46",
      edge: "12",
      firefox: "16",
      ie: "false",
      node: "No",
      safari: "8",
    },
  },

  spread_spread_in_function_calls: Compat {
    name: "spread_in_function_calls",
    description: r##"Spread in function calls"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Spread_syntax#Spread_in_function_calls",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#prod-ArgumentList",
    support: Support {
      chrome: "46",
      edge: "12",
      firefox: "27",
      ie: "false",
      node: "No",
      safari: "8",
    },
  },

  spread_spread_in_object_literals: Compat {
    name: "spread_in_object_literals",
    description: r##"Spread in object literals"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Spread_syntax#Spread_in_object_literals",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#prod-PropertyDefinition",
    support: Support {
      chrome: "60",
      edge: "No",
      firefox: "55",
      ie: "false",
      node: "No",
      safari: "11.1",
    },
  },

  strict_equality: Compat {
    name: "strict_equality",
    description: r##"Strict equality (<code>a === b</code>)"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Strict_equality",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-equality-operators",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  strict_inequality: Compat {
    name: "strict_inequality",
    description: r##"Strict inequality (<code>a !== b</code>)"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Strict_inequality",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-equality-operators",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  subtraction: Compat {
    name: "subtraction",
    description: r##"Subtraction (<code>-</code>)"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Subtraction",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-subtraction-operator-minus",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  subtraction_assignment: Compat {
    name: "subtraction_assignment",
    description: r##"Subtraction assignment (<code>x -= y</code>)"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Subtraction_assignment",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-assignment-operators",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  r#super_alias: Compat {
    name: "super",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/super",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-super-keyword",
    support: Support {
      chrome: "42",
      edge: "13",
      firefox: "45",
      ie: "false",
      node: "No",
      safari: "7",
    },
  },

  this: Compat {
    name: "this",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/this",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-this-keyword",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  r#typeof: Compat {
    name: "typeof",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/typeof",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-typeof-operator",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  unary_negation: Compat {
    name: "unary_negation",
    description: r##"Unary negation (<code>-</code>)"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Unary_negation",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-unary-minus-operator",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  unary_plus: Compat {
    name: "unary_plus",
    description: r##"Unary plus (<code>+</code>)"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Unary_plus",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-unary-plus-operator",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  unsigned_right_shift: Compat {
    name: "unsigned_right_shift",
    description: r##"Bitwise unsigned right shift (<code>a &gt;&gt;&gt; b</code>)"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Unsigned_right_shift",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-unsigned-right-shift-operator",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  unsigned_right_shift_assignment: Compat {
    name: "unsigned_right_shift_assignment",
    description: r##"Unsigned right shift assignment (<code>x &gt;&gt;&gt;= y</code>)"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Unsigned_right_shift_assignment",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-assignment-operators",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  r#void: Compat {
    name: "void",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/void",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-void-operator",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "5",
      node: "No",
      safari: "3.1",
    },
  },

  r#yield: Compat {
    name: "yield",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/yield",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-functions-and-classes.html#prod-YieldExpression",
    support: Support {
      chrome: "39",
      edge: "12",
      firefox: "26",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  yield_star: Compat {
    name: "yield_star",
    description: r##"<code>yield*</code>"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/yield*",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-functions-and-classes.html#sec-generator-function-definitions-runtime-semantics-evaluation",
    support: Support {
      chrome: "39",
      edge: "12",
      firefox: "27",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },
};
    