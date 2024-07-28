use super::support::Support;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Compat<'a> {
  pub name: &'a str,
  pub description: &'a str,
  pub mdn_url: &'a str,
  pub spec_url: &'a str,
  pub support: Support<'a>,
}

// pub struct Statements<'a> {
//   pub async_function: Compat<'a>,
//   pub async_generator_function: Compat<'a>,
//   pub block: Compat<'a>,
//   pub r#break: Compat<'a>,
//   pub r#class: Compat<'a>,
//   pub r#const: Compat<'a>,
//   pub r#continue: Compat<'a>,
//   pub r#debugger: Compat<'a>,
//   pub do_while: Compat<'a>,
//   pub empty: Compat<'a>,
//   pub r#export: Compat<'a>,
//   pub r#for: Compat<'a>,
//   pub for_await_of: Compat<'a>,
//   pub for_in: Compat<'a>,
//   pub for_of: Compat<'a>,
//   pub r#function: Compat<'a>,
//   pub generator_function: Compat<'a>,
//   pub if_else: Compat<'a>,
//   pub r#import: Compat<'a>,
//   pub r#label: Compat<'a>,
//   pub r#let: Compat<'a>,
//   pub r#return: Compat<'a>,
//   pub r#switch: Compat<'a>,
//   pub r#throw: Compat<'a>,
//   pub try_catch: Compat<'a>,
//   pub r#var: Compat<'a>,
//   pub r#while: Compat<'a>,
//   pub r#with: Compat<'a>,
// }

// pub struct Functions<'a>{
//   pub functions: Compat<'a>,
//   pub arguments: Compat<'a>,
//   pub callee: Compat<'a>,
//   pub length: Compat<'a>,
//   pub iterator: Compat<'a>,
//   pub arrow_functions: Compat<'a>,
//   pub trailing_comma: Compat<'a>,
//   pub block_level_functions: Compat<'a>,
//   pub default_parameters: Compat<'a>,
//   pub destructured_parameter_with_default_value_assignment: Compat<'a>,
//   pub parameters_without_defaults_after_default_parameters: Compat<'a>,
//   pub get: Compat<'a>,
//   pub computed_property_names: Compat<'a>,
//   pub set: Compat<'a>,

//   pub method_definitions: Compat<'a>,
//   pub async_generator_methods: Compat<'a>,
//   pub async_methods: Compat<'a>,
//   pub generator_methods_not_constructable: Compat<'a>,
//   pub rest_parameters: Compat<'a>,
//   pub destructuring: Compat<'a>,
// }

// pub const STATEMENTS: Statements = Statements {

//   async_function: Compat {
//     name: "async_function",
//     description: "<code>async function</code> statement",
//     mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/async_function",
//     spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-functions-and-classes.html#sec-async-function-definitions",
//     support: Support {
//       chrome: "55",
//       edge: "15",
//       firefox: "52",
//       ie: "false",
//       node: "undefined",
//       safari: "10.1",
//     },
//   },

//   async_generator_function: Compat {
//     name: "async_generator_function",
//     description: "<code>async function*</code> statement",
//     mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/async_function*",
//     spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-functions-and-classes.html#sec-async-generator-function-definitions",
//     support: Support {
//       chrome: "63",
//       edge: "undefined",
//       firefox: "55",
//       ie: "false",
//       node: "undefined",
//       safari: "12",
//     },
//   },

//   block: Compat {
//     name: "block",
//     description: "undefined",
//     mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/block",
//     spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-statements-and-declarations.html#sec-block",
//     support: Support {
//       chrome: "1",
//       edge: "12",
//       firefox: "1",
//       ie: "11",
//       node: "undefined",
//       safari: "1",
//     },
//   },

//   r#break: Compat {
//     name: "break",
//     description: "undefined",
//     mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/break",
//     spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-statements-and-declarations.html#sec-break-statement",
//     support: Support {
//       chrome: "1",
//       edge: "12",
//       firefox: "1",
//       ie: "3",
//       node: "undefined",
//       safari: "1",
//     },
//   },

//   r#class: Compat {
//     name: "class",
//     description: "undefined",
//     mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/class",
//     spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-functions-and-classes.html#sec-class-definitions",
//     support: Support {
//       chrome: "49",
//       edge: "13",
//       firefox: "45",
//       ie: "false",
//       node: "undefined",
//       safari: "10.1",
//     },
//   },

//   r#const: Compat {
//     name: "const",
//     description: "undefined",
//     mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/const",
//     spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-statements-and-declarations.html#sec-let-and-const-declarations",
//     support: Support {
//       chrome: "21",
//       edge: "12",
//       firefox: "36",
//       ie: "11",
//       node: "undefined",
//       safari: "5.1",
//     },
//   },

//   r#continue: Compat {
//     name: "continue",
//     description: "undefined",
//     mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/continue",
//     spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-statements-and-declarations.html#sec-continue-statement",
//     support: Support {
//       chrome: "1",
//       edge: "12",
//       firefox: "1",
//       ie: "3",
//       node: "undefined",
//       safari: "1",
//     },
//   },

//   r#debugger: Compat {
//     name: "debugger",
//     description: "undefined",
//     mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/debugger",
//     spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-statements-and-declarations.html#sec-debugger-statement",
//     support: Support {
//       chrome: "5",
//       edge: "12",
//       firefox: "1",
//       ie: "4",
//       node: "undefined",
//       safari: "5",
//     },
//   },

//   do_while: Compat {
//     name: "do_while",
//     description: "<code>do...while</code>",
//     mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/do...while",
//     spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-statements-and-declarations.html#sec-do-while-statement",
//     support: Support {
//       chrome: "1",
//       edge: "12",
//       firefox: "1",
//       ie: "4",
//       node: "undefined",
//       safari: "1",
//     },
//   },

//   empty: Compat {
//     name: "empty",
//     description: "Empty statement (<code>;</code>)",
//     mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/Empty",
//     spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-statements-and-declarations.html#sec-empty-statement",
//     support: Support {
//       chrome: "3",
//       edge: "12",
//       firefox: "1",
//       ie: "3",
//       node: "undefined",
//       safari: "5",
//     },
//   },

//   r#export: Compat {
//     name: "export",
//     description: "undefined",
//     mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/export",
//     spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-scripts-and-modules.html#sec-exports",
//     support: Support {
//       chrome: "61",
//       edge: "16",
//       firefox: "60",
//       ie: "false",
//       node: "undefined",
//       safari: "10.1",
//     },
//   },

//   r#for: Compat {
//     name: "for",
//     description: "undefined",
//     mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/for",
//     spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-statements-and-declarations.html#sec-for-statement",
//     support: Support {
//       chrome: "1",
//       edge: "12",
//       firefox: "1",
//       ie: "3",
//       node: "undefined",
//       safari: "1",
//     },
//   },

//   for_await_of: Compat {
//     name: "for_await_of",
//     description: "<code>for await...of</code>",
//     mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/for-await...of",
//     spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-statements-and-declarations.html#sec-for-in-and-for-of-statements",
//     support: Support {
//       chrome: "63",
//       edge: "undefined",
//       firefox: "57",
//       ie: "false",
//       node: "undefined",
//       safari: "12",
//     },
//   },

//   for_in: Compat {
//     name: "for_in",
//     description: "<code>for...in</code>",
//     mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/for...in",
//     spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-statements-and-declarations.html#sec-for-in-and-for-of-statements",
//     support: Support {
//       chrome: "1",
//       edge: "12",
//       firefox: "1",
//       ie: "3",
//       node: "undefined",
//       safari: "1",
//     },
//   },

//   for_of: Compat {
//     name: "for_of",
//     description: "<code>for...of</code>",
//     mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/for...of",
//     spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-statements-and-declarations.html#sec-for-in-and-for-of-statements",
//     support: Support {
//       chrome: "38",
//       edge: "12",
//       firefox: "13",
//       ie: "false",
//       node: "undefined",
//       safari: "7",
//     },
//   },

//   r#function: Compat {
//     name: "function",
//     description: "<code>function</code> statement",
//     mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/function",
//     spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-functions-and-classes.html#sec-function-definitions",
//     support: Support {
//       chrome: "1",
//       edge: "12",
//       firefox: "1",
//       ie: "3",
//       node: "undefined",
//       safari: "1",
//     },
//   },

//   generator_function: Compat {
//     name: "generator_function",
//     description: "<code>function*</code> statement",
//     mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/function*",
//     spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-functions-and-classes.html#sec-generator-function-definitions",
//     support: Support {
//       chrome: "39",
//       edge: "13",
//       firefox: "26",
//       ie: "false",
//       node: "undefined",
//       safari: "10",
//     },
//   },

//   if_else: Compat {
//     name: "if_else",
//     description: "<code>if...else</code>",
//     mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/if...else",
//     spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-statements-and-declarations.html#sec-if-statement",
//     support: Support {
//       chrome: "1",
//       edge: "12",
//       firefox: "1",
//       ie: "3",
//       node: "undefined",
//       safari: "1",
//     },
//   },

//   r#import: Compat {
//     name: "import",
//     description: "undefined",
//     mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/import",
//     spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-scripts-and-modules.html#sec-imports",
//     support: Support {
//       chrome: "61",
//       edge: "16",
//       firefox: "60",
//       ie: "false",
//       node: "undefined",
//       safari: "10.1",
//     },
//   },

//   r#label: Compat {
//     name: "label",
//     description: "undefined",
//     mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/label",
//     spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-statements-and-declarations.html#sec-labelled-statements",
//     support: Support {
//       chrome: "1",
//       edge: "12",
//       firefox: "1",
//       ie: "4",
//       node: "undefined",
//       safari: "1",
//     },
//   },

//   r#let: Compat {
//     name: "let",
//     description: "undefined",
//     mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/let",
//     spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-statements-and-declarations.html#sec-let-and-const-declarations",
//     support: Support {
//       chrome: "49",
//       edge: "14",
//       firefox: "44",
//       ie: "11",
//       node: "undefined",
//       safari: "10",
//     },
//   },

//   r#return: Compat {
//     name: "return",
//     description: "undefined",
//     mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/return",
//     spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-statements-and-declarations.html#sec-return-statement",
//     support: Support {
//       chrome: "1",
//       edge: "12",
//       firefox: "1",
//       ie: "3",
//       node: "undefined",
//       safari: "1",
//     },
//   },

//   r#switch: Compat {
//     name: "switch",
//     description: "undefined",
//     mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/switch",
//     spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-statements-and-declarations.html#sec-switch-statement",
//     support: Support {
//       chrome: "1",
//       edge: "12",
//       firefox: "1",
//       ie: "4",
//       node: "undefined",
//       safari: "1",
//     },
//   },

//   r#throw: Compat {
//     name: "throw",
//     description: "undefined",
//     mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/throw",
//     spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-statements-and-declarations.html#sec-throw-statement",
//     support: Support {
//       chrome: "1",
//       edge: "12",
//       firefox: "1",
//       ie: "5",
//       node: "undefined",
//       safari: "1",
//     },
//   },

//   try_catch: Compat {
//     name: "try_catch",
//     description: "<code>try...catch</code>",
//     mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/try...catch",
//     spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-statements-and-declarations.html#sec-try-statement",
//     support: Support {
//       chrome: "1",
//       edge: "12",
//       firefox: "1",
//       ie: "5",
//       node: "undefined",
//       safari: "1",
//     },
//   },

//   r#var: Compat {
//     name: "var",
//     description: "undefined",
//     mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/var",
//     spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-statements-and-declarations.html#sec-variable-statement",
//     support: Support {
//       chrome: "1",
//       edge: "12",
//       firefox: "1",
//       ie: "3",
//       node: "undefined",
//       safari: "1",
//     },
//   },

//   r#while: Compat {
//     name: "while",
//     description: "undefined",
//     mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/while",
//     spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-statements-and-declarations.html#sec-while-statement",
//     support: Support {
//       chrome: "1",
//       edge: "12",
//       firefox: "1",
//       ie: "3",
//       node: "undefined",
//       safari: "1",
//     },
//   },

//   r#with: Compat {
//     name: "with",
//     description: "undefined",
//     mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/with",
//     spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-statements-and-declarations.html#sec-with-statement",
//     support: Support {
//       chrome: "1",
//       edge: "12",
//       firefox: "1",
//       ie: "3",
//       node: "undefined",
//       safari: "1",
//     },
//   },

// };

// pub const FUNCTIONS: Functions = Functions {
//   functions: Compat {
//     name: "functions",
//     description: "undefined",
//     mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Functions",
//     spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-functions-and-classes.html#sec-function-definitions",
//     support: Support {
//       chrome: "1",
//       edge: "12",
//       firefox: "1",
//       ie: "3",
//       node: "No",
//       safari: "1",
//     },
//   }
// ,
//   arguments: Compat {
//     name: "arguments",
//     description: "undefined",
//     mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Functions/arguments",
//     spec_url: "https://tc39.es/ecma262/multipage/ordinary-and-exotic-objects-behaviours.html#sec-arguments-exotic-objects",
//     support: Support {
//       chrome: "1",
//       edge: "12",
//       firefox: "1",
//       ie: "3",
//       node: "No",
//       safari: "1",
//     },
//   }
// ,
//   callee: Compat {
//     name: "callee",
//     description: "undefined",
//     mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Functions/arguments/callee",
//     spec_url: "https://tc39.es/ecma262/multipage/ordinary-and-exotic-objects-behaviours.html#sec-arguments-exotic-objects",
//     support: Support {
//       chrome: "1",
//       edge: "12",
//       firefox: "1",
//       ie: "6",
//       node: "No",
//       safari: "1",
//     },
//   }
// ,
//   length: Compat {
//     name: "length",
//     description: "undefined",
//     mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Functions/arguments/length",
//     spec_url: "https://tc39.es/ecma262/multipage/ordinary-and-exotic-objects-behaviours.html#sec-arguments-exotic-objects",
//     support: Support {
//       chrome: "1",
//       edge: "12",
//       firefox: "1",
//       ie: "4",
//       node: "No",
//       safari: "1",
//     },
//   }
// ,
//   iterator: Compat {
//     name: "@@iterator",
//     description: "[Symbol.iterator]",
//     mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Functions/arguments/Symbol.iterator",
//     spec_url: "https://tc39.es/ecma262/multipage/ordinary-and-exotic-objects-behaviours.html#sec-createunmappedargumentsobject,https://tc39.es/ecma262/multipage/ordinary-and-exotic-objects-behaviours.html#sec-createmappedargumentsobject",
//     support: Support {
//       chrome: "52",
//       edge: "12",
//       firefox: "46",
//       ie: "false",
//       node: "No",
//       safari: "9",
//     },
//   }
// ,
//   arrow_functions: Compat {
//     name: "arrow_functions",
//     description: "Arrow functions",
//     mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Functions/Arrow_functions",
//     spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-functions-and-classes.html#sec-arrow-function-definitions",
//     support: Support {
//       chrome: "45",
//       edge: "12",
//       firefox: "22",
//       ie: "false",
//       node: "No",
//       safari: "10",
//     },
//   }
// ,
//   trailing_comma: Compat {
//     name: "trailing_comma",
//     description: "Trailing comma in parameters",
//     mdn_url: "undefined",
//     spec_url: "undefined",
//     support: Support {
//       chrome: "58",
//       edge: "12",
//       firefox: "52",
//       ie: "false",
//       node: "No",
//       safari: "10",
//     },
//   }
// ,
//   block_level_functions: Compat {
//     name: "block_level_functions",
//     description: "Block-level functions",
//     mdn_url: "undefined",
//     spec_url: "undefined",
//     support: Support {
//       chrome: "49",
//       edge: "12",
//       firefox: "46",
//       ie: "11",
//       node: "No",
//       safari: "10",
//     },
//   }
// ,
//   default_parameters: Compat {
//     name: "default_parameters",
//     description: "Default parameters",
//     mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Functions/Default_parameters",
//     spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-functions-and-classes.html#sec-function-definitions",
//     support: Support {
//       chrome: "49",
//       edge: "14",
//       firefox: "15",
//       ie: "false",
//       node: "No",
//       safari: "10",
//     },
//   }
// ,
//   destructured_parameter_with_default_value_assignment: Compat {
//     name: "destructured_parameter_with_default_value_assignment",
//     description: "Destructured parameter with default value assignment",
//     mdn_url: "undefined",
//     spec_url: "undefined",
//     support: Support {
//       chrome: "49",
//       edge: "14",
//       firefox: "41",
//       ie: "false",
//       node: "No",
//       safari: "10",
//     },
//   }
// ,
//   parameters_without_defaults_after_default_parameters: Compat {
//     name: "parameters_without_defaults_after_default_parameters",
//     description: "Parameters without defaults after default parameters",
//     mdn_url: "undefined",
//     spec_url: "undefined",
//     support: Support {
//       chrome: "49",
//       edge: "14",
//       firefox: "26",
//       ie: "false",
//       node: "No",
//       safari: "10",
//     },
//   }
// ,
//   get: Compat {
//     name: "get",
//     description: "undefined",
//     mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Functions/get",
//     spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-functions-and-classes.html#sec-method-definitions",
//     support: Support {
//       chrome: "1",
//       edge: "12",
//       firefox: "1.5",
//       ie: "9",
//       node: "No",
//       safari: "3",
//     },
//   }
// ,
//   computed_property_names: Compat {
//     name: "computed_property_names",
//     description: "Computed property names",
//     mdn_url: "undefined",
//     spec_url: "undefined",
//     support: Support {
//       chrome: "46",
//       edge: "12",
//       firefox: "34",
//       ie: "false",
//       node: "No",
//       safari: "9.1",
//     },
//   }
// ,
//   method_definitions: Compat {
//     name: "method_definitions",
//     description: "Method definitions",
//     mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Functions/Method_definitions",
//     spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-functions-and-classes.html#sec-method-definitions",
//     support: Support {
//       chrome: "39",
//       edge: "12",
//       firefox: "34",
//       ie: "false",
//       node: "No",
//       safari: "9",
//     },
//   }
// ,
//   async_generator_methods: Compat {
//     name: "async_generator_methods",
//     description: "Async generator methods",
//     mdn_url: "undefined",
//     spec_url: "undefined",
//     support: Support {
//       chrome: "63",
//       edge: "No",
//       firefox: "55",
//       ie: "false",
//       node: "No",
//       safari: "12",
//     },
//   }
// ,
//   async_methods: Compat {
//     name: "async_methods",
//     description: "Async methods",
//     mdn_url: "undefined",
//     spec_url: "undefined",
//     support: Support {
//       chrome: "55",
//       edge: "15",
//       firefox: "52",
//       ie: "false",
//       node: "No",
//       safari: "10.1",
//     },
//   }
// ,
//   generator_methods_not_constructable: Compat {
//     name: "generator_methods_not_constructable",
//     description: "Generator methods are not constructable (ES2016)",
//     mdn_url: "undefined",
//     spec_url: "undefined",
//     support: Support {
//       chrome: "42",
//       edge: "13",
//       firefox: "43",
//       ie: "false",
//       node: "No",
//       safari: "9.1",
//     },
//   }
// ,
//   rest_parameters: Compat {
//     name: "rest_parameters",
//     description: "Rest parameters",
//     mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Functions/rest_parameters",
//     spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-functions-and-classes.html#sec-function-definitions",
//     support: Support {
//       chrome: "47",
//       edge: "12",
//       firefox: "15",
//       ie: "false",
//       node: "No",
//       safari: "10",
//     },
//   }
// ,
//   destructuring: Compat {
//     name: "destructuring",
//     description: "Destructuring rest parameters",
//     mdn_url: "undefined",
//     spec_url: "undefined",
//     support: Support {
//       chrome: "49",
//       edge: "No",
//       firefox: "52",
//       ie: "false",
//       node: "No",
//       safari: "10",
//     },
//   }
// ,
//   set: Compat {
//     name: "set",
//     description: "undefined",
//     mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Functions/set",
//     spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-functions-and-classes.html#sec-method-definitions",
//     support: Support {
//       chrome: "1",
//       edge: "12",
//       firefox: "1.5",
//       ie: "9",
//       node: "No",
//       safari: "3",
//     },
//   }
// };
