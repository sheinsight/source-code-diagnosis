
use super::{compat::Compat, support::Support};

pub struct Functions<'a>{
    
      pub functions: Compat<'a>,
    
      pub functions_arguments: Compat<'a>,
    
      pub functions_arguments_callee: Compat<'a>,
    
      pub functions_arguments_length: Compat<'a>,
    
      pub functions_arguments_iterator: Compat<'a>,
    
      pub functions_arrow_functions: Compat<'a>,
    
      pub functions_arrow_functions_trailing_comma: Compat<'a>,
    
      pub functions_block_level_functions: Compat<'a>,
    
      pub functions_default_parameters: Compat<'a>,
    
      pub functions_default_parameters_destructured_parameter_with_default_value_assignment: Compat<'a>,
    
      pub functions_default_parameters_parameters_without_defaults_after_default_parameters: Compat<'a>,
    
      pub functions_get: Compat<'a>,
    
      pub functions_get_computed_property_names: Compat<'a>,
    
      pub functions_method_definitions: Compat<'a>,
    
      pub functions_method_definitions_async_generator_methods: Compat<'a>,
    
      pub functions_method_definitions_async_methods: Compat<'a>,
    
      pub functions_method_definitions_generator_methods_not_constructable: Compat<'a>,
    
      pub functions_rest_parameters: Compat<'a>,
    
      pub functions_rest_parameters_destructuring: Compat<'a>,
    
      pub functions_set: Compat<'a>,
    
      pub functions_set_computed_property_names: Compat<'a>,
    
}

pub const FUNCTIONS: Functions = Functions { 
    
  functions: Compat {
    name: "functions",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Functions",
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

  functions_arguments: Compat {
    name: "arguments",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Functions/arguments",
    spec_url: "https://tc39.es/ecma262/multipage/ordinary-and-exotic-objects-behaviours.html#sec-arguments-exotic-objects",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "3",
      node: "No",
      safari: "1",
    },
  },

  functions_arguments_callee: Compat {
    name: "callee",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Functions/arguments/callee",
    spec_url: "https://tc39.es/ecma262/multipage/ordinary-and-exotic-objects-behaviours.html#sec-arguments-exotic-objects",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "6",
      node: "No",
      safari: "1",
    },
  },

  functions_arguments_length: Compat {
    name: "length",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Functions/arguments/length",
    spec_url: "https://tc39.es/ecma262/multipage/ordinary-and-exotic-objects-behaviours.html#sec-arguments-exotic-objects",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1",
      ie: "4",
      node: "No",
      safari: "1",
    },
  },

  functions_arguments_iterator: Compat {
    name: "@@iterator",
    description: r##"[Symbol.iterator]"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Functions/arguments/Symbol.iterator",
    spec_url: "https://tc39.es/ecma262/multipage/ordinary-and-exotic-objects-behaviours.html#sec-createunmappedargumentsobject,https://tc39.es/ecma262/multipage/ordinary-and-exotic-objects-behaviours.html#sec-createmappedargumentsobject",
    support: Support {
      chrome: "52",
      edge: "12",
      firefox: "46",
      ie: "false",
      node: "No",
      safari: "9",
    },
  },

  functions_arrow_functions: Compat {
    name: "arrow_functions",
    description: r##"Arrow functions"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Functions/Arrow_functions",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-functions-and-classes.html#sec-arrow-function-definitions",
    support: Support {
      chrome: "45",
      edge: "12",
      firefox: "22",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  functions_arrow_functions_trailing_comma: Compat {
    name: "trailing_comma",
    description: r##"Trailing comma in parameters"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "58",
      edge: "12",
      firefox: "52",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  functions_block_level_functions: Compat {
    name: "block_level_functions",
    description: r##"Block-level functions"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "49",
      edge: "12",
      firefox: "46",
      ie: "11",
      node: "No",
      safari: "10",
    },
  },

  functions_default_parameters: Compat {
    name: "default_parameters",
    description: r##"Default parameters"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Functions/Default_parameters",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-functions-and-classes.html#sec-function-definitions",
    support: Support {
      chrome: "49",
      edge: "14",
      firefox: "15",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  functions_default_parameters_destructured_parameter_with_default_value_assignment: Compat {
    name: "destructured_parameter_with_default_value_assignment",
    description: r##"Destructured parameter with default value assignment"##,
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

  functions_default_parameters_parameters_without_defaults_after_default_parameters: Compat {
    name: "parameters_without_defaults_after_default_parameters",
    description: r##"Parameters without defaults after default parameters"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "49",
      edge: "14",
      firefox: "26",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  functions_get: Compat {
    name: "get",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Functions/get",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-functions-and-classes.html#sec-method-definitions",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1.5",
      ie: "9",
      node: "No",
      safari: "3",
    },
  },

  functions_get_computed_property_names: Compat {
    name: "computed_property_names",
    description: r##"Computed property names"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "46",
      edge: "12",
      firefox: "34",
      ie: "false",
      node: "No",
      safari: "9.1",
    },
  },

  functions_method_definitions: Compat {
    name: "method_definitions",
    description: r##"Method definitions"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Functions/Method_definitions",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-functions-and-classes.html#sec-method-definitions",
    support: Support {
      chrome: "39",
      edge: "12",
      firefox: "34",
      ie: "false",
      node: "No",
      safari: "9",
    },
  },

  functions_method_definitions_async_generator_methods: Compat {
    name: "async_generator_methods",
    description: r##"Async generator methods"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "63",
      edge: "No",
      firefox: "55",
      ie: "false",
      node: "No",
      safari: "12",
    },
  },

  functions_method_definitions_async_methods: Compat {
    name: "async_methods",
    description: r##"Async methods"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "55",
      edge: "15",
      firefox: "52",
      ie: "false",
      node: "No",
      safari: "10.1",
    },
  },

  functions_method_definitions_generator_methods_not_constructable: Compat {
    name: "generator_methods_not_constructable",
    description: r##"Generator methods are not constructable (ES2016)"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "42",
      edge: "13",
      firefox: "43",
      ie: "false",
      node: "No",
      safari: "9.1",
    },
  },

  functions_rest_parameters: Compat {
    name: "rest_parameters",
    description: r##"Rest parameters"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Functions/rest_parameters",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-functions-and-classes.html#sec-function-definitions",
    support: Support {
      chrome: "47",
      edge: "12",
      firefox: "15",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  functions_rest_parameters_destructuring: Compat {
    name: "destructuring",
    description: r##"Destructuring rest parameters"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "49",
      edge: "No",
      firefox: "52",
      ie: "false",
      node: "No",
      safari: "10",
    },
  },

  functions_set: Compat {
    name: "set",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Functions/set",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-functions-and-classes.html#sec-method-definitions",
    support: Support {
      chrome: "1",
      edge: "12",
      firefox: "1.5",
      ie: "9",
      node: "No",
      safari: "3",
    },
  },

  functions_set_computed_property_names: Compat {
    name: "computed_property_names",
    description: r##"Computed property names"##,
    mdn_url: "undefined",
    spec_url: "undefined",
    support: Support {
      chrome: "46",
      edge: "12",
      firefox: "34",
      ie: "false",
      node: "No",
      safari: "9.1",
    },
  },
};
    