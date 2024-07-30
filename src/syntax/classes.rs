
use super::{compat::Compat, support::Support};

pub struct Classes<'a>{
    
      pub r#constructor: Compat<'a>,
    
      pub extends: Compat<'a>,
    
      pub private_class_fields: Compat<'a>,
    
      pub private_class_fields_in: Compat<'a>,
    
      pub private_class_methods: Compat<'a>,
    
      pub public_class_fields: Compat<'a>,
    
      pub r#static: Compat<'a>,
    
      pub static_class_fields: Compat<'a>,
    
      pub static_initialization_blocks: Compat<'a>,
    
}

pub const CLASSES: Classes = Classes { 
    
  r#constructor: Compat {
    name: "constructor",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Classes/constructor",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-functions-and-classes.html#sec-static-semantics-constructormethod",
    support: Support {
      chrome: "49",
      edge: "13",
      firefox: "45",
      ie: "false",
      node: "No",
      safari: "9",
    },
  },

  extends: Compat {
    name: "extends",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Classes/extends",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-functions-and-classes.html#sec-class-definitions",
    support: Support {
      chrome: "49",
      edge: "13",
      firefox: "45",
      ie: "false",
      node: "No",
      safari: "9",
    },
  },

  private_class_fields: Compat {
    name: "private_class_fields",
    description: r##"Private class fields"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Classes/Private_properties",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-lexical-grammar.html#prod-PrivateIdentifier",
    support: Support {
      chrome: "74",
      edge: "No",
      firefox: "90",
      ie: "false",
      node: "No",
      safari: "14.1",
    },
  },

  private_class_fields_in: Compat {
    name: "private_class_fields_in",
    description: r##"Private class fields 'in'"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Classes/Private_properties",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#prod-00OK517S",
    support: Support {
      chrome: "91",
      edge: "No",
      firefox: "90",
      ie: "false",
      node: "No",
      safari: "15",
    },
  },

  private_class_methods: Compat {
    name: "private_class_methods",
    description: r##"Private class methods"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Classes/Private_properties",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-lexical-grammar.html#prod-PrivateIdentifier",
    support: Support {
      chrome: "84",
      edge: "No",
      firefox: "90",
      ie: "false",
      node: "No",
      safari: "15",
    },
  },

  public_class_fields: Compat {
    name: "public_class_fields",
    description: r##"Public class fields"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Classes/Public_class_fields",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-functions-and-classes.html#prod-FieldDefinition",
    support: Support {
      chrome: "72",
      edge: "No",
      firefox: "69",
      ie: "false",
      node: "No",
      safari: "16",
    },
  },

  r#static: Compat {
    name: "static",
    description: r##"undefined"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Classes/static",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-functions-and-classes.html#sec-class-definitions",
    support: Support {
      chrome: "49",
      edge: "13",
      firefox: "45",
      ie: "false",
      node: "No",
      safari: "9",
    },
  },

  static_class_fields: Compat {
    name: "static_class_fields",
    description: r##"Static class fields"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Classes/Public_class_fields",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-functions-and-classes.html#prod-FieldDefinition",
    support: Support {
      chrome: "72",
      edge: "No",
      firefox: "75",
      ie: "false",
      node: "No",
      safari: "14.1",
    },
  },

  static_initialization_blocks: Compat {
    name: "static_initialization_blocks",
    description: r##"Class static initialization blocks"##,
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Classes/Static_initialization_blocks",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-functions-and-classes.html#prod-ClassStaticBlock",
    support: Support {
      chrome: "94",
      edge: "No",
      firefox: "93",
      ie: "false",
      node: "No",
      safari: "16.4",
    },
  },
};
    