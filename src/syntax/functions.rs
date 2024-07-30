use super::compat::{Compat, Status, Support};

pub struct Functions<'a> {
  pub arguments: Compat<'a>,

  pub arrow_functions: Compat<'a>,

  pub trailing_comma: Compat<'a>,
}

pub const FUNCTIONS: Functions = Functions {
  arrow_functions: Compat {
    name: "arrow_functions",
    description: "Arrow functions",
    mdn_url:
      "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Functions/Arrow_functions",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-functions-and-classes.html#sec-arrow-function-definitions",
    tags: &[
      "web-features:snapshot:ecmascript-2015"
    ],
    support: Support {
      chrome: "45", 
      chrome_android: "45", 
      firefox: "22", 
      firefox_android: "22", 
      safari: "10", 
      safari_ios: "10", 
      opera: "45", 
      opera_android: "45", 
      ie: "0", 
      edge: "12", 
      deno: "1.0", 
      node: "4.0.0" 
    },
    status: Status {
      experimental: false,
      standard_track: true,
      deprecated: false
    },
  },

  trailing_comma: Compat {
    name: "trailing_comma", 
    description: "Trailing comma in parameters", 
    mdn_url: "", 
    spec_url: "", 
    tags: &["web-features:snapshot:ecmascript-2017"], 
    support: Support {
      chrome: "58", 
      chrome_android: "58", 
      firefox: "52", 
      firefox_android: "52", 
      safari: "10", 
      safari_ios: "10", 
      opera: "58", 
      opera_android: "58", 
      ie: "0", 
      edge: "12", 
      deno: "1.0", 
      node: "8.0.0" 
    },
    status: Status {
      experimental: false,
      standard_track: true,
      deprecated: false
    }
  },
  arguments: Compat {
    name: "arguments", 
    description: "", 
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Functions/arguments", 
    spec_url: "https://tc39.es/ecma262/multipage/ordinary-and-exotic-objects-behaviours.html#sec-arguments-exotic-objects", 
    tags:&[
      "web-features:snapshot:ecmascript-1"
    ],
    support: Support {
      chrome: "1",
      chrome_android: "1",
      firefox: "1", 
      firefox_android: "1", 
      safari: "1", 
      safari_ios: "1", 
      opera: "3", 
      opera_android: "3", 
      ie: "3", 
      edge: "12", 
      deno:"1.0",
      node: "0.10.0"
    },
    status: Status {
      experimental: false,
      standard_track: true,
      deprecated: false
    }
  },
};
