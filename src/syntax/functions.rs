use super::compat::{Compat, Status, Support};
use crate::str;

#[derive(Debug)]
pub struct Functions {

  pub functions: Compat,

  pub arguments: Compat,

  pub arrow_functions: Compat,

  pub arrow_functions_trailing_comma: Compat,

  pub default_parameters: Compat,

  pub getter: Compat,

  pub getter_computed_property_names: Compat,

  pub method_definitions: Compat,

  pub method_definitions_async_generator_methods: Compat,

  pub method_definitions_async_methods: Compat,

  pub method_definitions_generator_methods_not_constructable: Compat,
}

pub fn create_functions() -> Functions {

  Functions {

    functions: Compat { 
      name: str!("functions"),
      description: str!(""), 
      mdn_url: str!("https://developer.mozilla.org/docs/Web/JavaScript/Reference/Functions"), 
      spec_url: str!("https://tc39.es/ecma262/multipage/ecmascript-language-functions-and-classes.html#sec-function-definitions"), 
      tags: vec![
        str!("web-features:snapshot:ecmascript-1")
      ], 
      support: Support { 
        chrome: str!("1"), 
        chrome_android: str!("1"), 
        firefox: str!("1"), 
        firefox_android: str!("1"), 
        safari: str!("1"), 
        safari_ios: str!("1"), 
        opera: str!("3"), 
        opera_android: str!("10.1"), 
        ie: str!("3"), 
        edge: str!("12"), 
        deno: str!("1.0"), 
        node: str!("0.10.0")
      }, 
      status: Status { 
        experimental: false, 
        standard_track: true, 
        deprecated: false 
      }
    },
  
    arrow_functions: Compat {
      name: str!("arrow_functions"),
      description: str!("Arrow functions"),
      mdn_url:
        str!("https://developer.mozilla.org/docs/Web/JavaScript/Reference/Functions/Arrow_functions"),
      spec_url: str!("https://tc39.es/ecma262/multipage/ecmascript-language-functions-and-classes.html#sec-arrow-function-definitions"),
      tags: vec![
        str!("web-features:snapshot:ecmascript-2015")
      ],
      support: Support {
        chrome: str!("45"), 
        chrome_android: str!("45"), 
        firefox: str!("22"), 
        firefox_android: str!("22"), 
        safari: str!("10"), 
        safari_ios: str!("10"), 
        opera: str!("45"), 
        opera_android: str!("45"), 
        ie: str!("0"), 
        edge: str!("12"), 
        deno: str!("1.0"), 
        node: str!("4.0.0" )
      },
      status: Status {
        experimental: false,
        standard_track: true,
        deprecated: false
      },
    },
  
    arrow_functions_trailing_comma: Compat {
      name: str!("trailing_comma"), 
      description: str!("Trailing comma in parameters"), 
      mdn_url: str!(""), 
      spec_url: str!(""), 
      tags: vec![
        str!("web-features:snapshot:ecmascript-2017")
      ], 
      support: Support {
        chrome: str!("58"), 
        chrome_android: str!("58"), 
        firefox: str!("52"), 
        firefox_android: str!("52"), 
        safari: str!("10"), 
        safari_ios: str!("10"), 
        opera: str!("58"), 
        opera_android: str!("58"), 
        ie: str!("0"), 
        edge: str!("12"), 
        deno: str!("1.0"), 
        node: str!("8.0.0") 
      },
      status: Status {
        experimental: false,
        standard_track: true,
        deprecated: false
      }
    },
  
    arguments: Compat {
      name: str!("arguments"),
      description: str!(""), 
      mdn_url: str!("https://developer.mozilla.org/docs/Web/JavaScript/Reference/Functions/arguments"), 
      spec_url: str!("https://tc39.es/ecma262/multipage/ordinary-and-exotic-objects-behaviours.html#sec-arguments-exotic-objects"), 
      tags:vec![
        str!("web-features:snapshot:ecmascript-1")
      ],
      support: Support {
        chrome: str!("1"),
        chrome_android: str!("1"),
        firefox: str!("1"), 
        firefox_android: str!("1"), 
        safari: str!("1"), 
        safari_ios: str!("1"), 
        opera: str!("3"), 
        opera_android: str!("3"), 
        ie: str!("3"), 
        edge: str!("12"), 
        deno:str!("1.0"),
        node: str!("0.10.0")
      },
      status: Status {
        experimental: false,
        standard_track: true,
        deprecated: false
      }
    },
  
    default_parameters: Compat { 
      name: str!("default_parameters"), 
      description:str!("Default parameters"),
      mdn_url:str!("https://developer.mozilla.org/docs/Web/JavaScript/Reference/Functions/Default_parameters"), 
      spec_url:str!("https://tc39.es/ecma262/multipage/ecmascript-language-functions-and-classes.html#sec-function-definitions"),
      tags:vec![
        str!("web-features:snapshot:ecmascript-2015")
      ],
      support: Support { 
        chrome: str!("49"), 
        chrome_android: str!("49"), 
        firefox: str!("15"), 
        firefox_android: str!("15"), 
        safari: str!("10"), 
        safari_ios: str!("10"), 
        opera: str!("49"), 
        opera_android: str!("49"), 
        ie: str!("0"), 
        edge: str!("14"), 
        deno: str!("1.0"), 
        node: str!("6.0.0")
      }, status: Status { 
        experimental: false, 
        standard_track: true, 
        deprecated: false 
      } 
    },
  
    getter: Compat { 
      name: str!("get"), 
      description: str!(""), 
      mdn_url: str!("https://developer.mozilla.org/docs/Web/JavaScript/Reference/Functions/get"), 
      spec_url: str!("https://tc39.es/ecma262/multipage/ecmascript-language-functions-and-classes.html#sec-method-definitions"), 
      tags: vec![
        str!("web-features:snapshot:ecmascript-5")
      ], 
      support: Support { 
        chrome: str!("1"), 
        chrome_android: str!("1"), 
        firefox: str!("1.5"), 
        firefox_android: str!("1.5"), 
        safari: str!("3"), 
        safari_ios: str!("1"), 
        opera: str!("9.5"), 
        opera_android: str!("9.5"), 
        ie: str!("9"), 
        edge: str!("12"), 
        deno: str!("1.0"), 
        node: str!("0.10.0") 
      },
      status: Status { 
        experimental: false,
        standard_track: true,
        deprecated: false 
      }
    },
  
    getter_computed_property_names: Compat { 
      name: str!("computed_property_names"), 
      description: str!("Computed property names"), 
      mdn_url: str!("https://developer.mozilla.org/docs/Web/JavaScript/Reference/Functions/get"), 
      spec_url: str!("https://tc39.es/ecma262/multipage/ecmascript-language-functions-and-classes.html#sec-method-definitions"), 
      tags: vec![
        str!("web-features:snapshot:ecmascript-5")
      ], 
      support: Support { 
        chrome: str!("46"), 
        chrome_android: str!("46"), 
        firefox: str!("34"), 
        firefox_android: str!("34"), 
        safari: str!("9.1"), 
        safari_ios: str!("9.1"), 
        opera: str!("47"), 
        opera_android: str!("47"), 
        ie: str!("0"), 
        edge: str!("12"), 
        deno: str!("1.0"), 
        node: str!("4.0.0")
      }, 
      status: Status { 
        experimental: false, 
        standard_track: true, 
        deprecated: false 
      }
    },
  
    method_definitions: Compat { 
      name: str!("method_definitions"), 
      description: str!("Method definitions"), 
      mdn_url: str!("https://developer.mozilla.org/docs/Web/JavaScript/Reference/Functions/Method_definitions"), 
      spec_url: str!("https://tc39.es/ecma262/multipage/ecmascript-language-functions-and-classes.html#sec-method-definitions"), 
      tags: vec![
        str!("web-features:snapshot:ecmascript-2015")
      ], 
      support: Support { 
        chrome: str!("39"), 
        chrome_android: str!("39"), 
        firefox: str!("34"), 
        firefox_android: str!("34"), 
        safari: str!("9"), 
        safari_ios: str!("9"), 
        opera: str!("39"), 
        opera_android: str!("39"), 
        ie: str!("0"), 
        edge: str!("12"), 
        deno: str!("1.0"), 
        node: str!("4.0.0")
      }, 
      status: Status { 
        experimental: false, 
        standard_track: true, 
        deprecated: false 
      }
    },
    
    method_definitions_async_generator_methods: Compat { 
      name: str!("async_generator_methods"), 
      description: str!("Method definitions"),
      mdn_url: str!("https://developer.mozilla.org/docs/Web/JavaScript/Reference/Functions/Method_definitions"), 
      spec_url: str!("https://tc39.es/ecma262/multipage/ecmascript-language-functions-and-classes.html#sec-method-definitions"), 
      tags: vec![
        str!("web-features:snapshot:ecmascript-2015")
      ], 
      support: Support { 
        chrome: str!("39"), 
        chrome_android: str!("39"), 
        firefox: str!("34"), 
        firefox_android: str!("34"), 
        safari: str!("9"), 
        safari_ios: str!("9"), 
        opera: str!("39"), 
        opera_android: str!("39"), 
        ie: str!("0"), 
        edge: str!("12"), 
        deno: str!("1.0"), 
        node: str!("4.0.0") 
      }, 
      status: Status { 
        experimental: false, 
        standard_track: true, 
        deprecated: false 
      }
    },
    
    method_definitions_async_methods: Compat { 
      name: str!("async_methods"), 
      description: str!("Async methods"), 
      mdn_url: str!("https://developer.mozilla.org/docs/Web/JavaScript/Reference/Functions/Method_definitions"), 
      spec_url: str!("https://tc39.es/ecma262/multipage/ecmascript-language-functions-and-classes.html#sec-method-definitions"), 
      tags: vec![
        str!("web-features:snapshot:ecmascript-2015")
      ], 
      support: Support { 
        chrome: str!("55"), 
        chrome_android: str!("55"), 
        firefox: str!("52"), 
        firefox_android: str!("52"), 
        safari: str!("10.1"), 
        safari_ios: str!("10.1"), 
        opera: str!("55"), 
        opera_android: str!("55"), 
        ie: str!("0"), 
        edge: str!("15"),
        deno: str!("1.0"), 
        node: str!("7.6.0")
      }, 
      status: Status { 
        experimental: false, 
        standard_track: true, 
        deprecated: false 
      }
    },
    
    method_definitions_generator_methods_not_constructable: Compat { 
      name: str!("generator_methods_not_constructable"), 
      description: str!("Generator methods are not constructable (ES2016)"), 
      mdn_url: str!("https://developer.mozilla.org/docs/Web/JavaScript/Reference/Functions/Method_definitions"), 
      spec_url: str!("https://tc39.es/ecma262/multipage/ecmascript-language-functions-and-classes.html#sec-method-definitions"), 
      tags: vec![
        str!("web-features:snapshot:ecmascript-2015")
      ], 
      support: Support { 
        chrome: str!("42"), 
        chrome_android: str!("42"), 
        firefox: str!("43"), 
        firefox_android: str!("43"), 
        safari: str!("9.1"), 
        safari_ios: str!("9.1"), 
        opera: str!("42"), 
        opera_android: str!("42"), 
        ie: str!("0"), 
        edge: str!("13"), 
        deno: str!("1.0"), 
        node: str!("6.0.0")
      }, 
      status: Status { 
        experimental: false, 
        standard_track: true, 
        deprecated: false 
      }
    },
      
  }
}
