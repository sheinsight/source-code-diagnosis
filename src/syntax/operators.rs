use super::compat::{Compat, Status, Support};

pub struct Operators<'a> {
  pub addition_assignment: Compat<'a>,

  pub addition: Compat<'a>,

  pub assignment: Compat<'a>,
}

pub const OPERATORS: Operators = Operators {
  
  addition_assignment: Compat {
    name: "addition_assignment",
    description: "Addition assignment (<code>x += y</code>)",
    mdn_url:
      "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Addition_assignment",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-assignment-operators",
    tags: &["web-features:snapshot:ecmascript-1"],
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
        deno: "1.0",
        node: "0.10.0",
    },
    status: Status {
      experimental: false,
      standard_track: true,
      deprecated: false,
    },
  },

  addition: Compat { 
    name: "addition", 
    description: "Addition (<code>+</code>)", 
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Addition",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-addition-operator-plus", 
    tags: &["web-features:snapshot:ecmascript-1"], 
    support: Support { 
      chrome: "1", 
      chrome_android: "1", 
      firefox: "1", 
      firefox_android: "1", 
      safari: "1", 
      safari_ios: "1", 
      opera: "3", 
      opera_android: "10.1", 
      ie: "3", 
      edge: "12", 
      deno: "1.0", 
      node: "0.10.0" 
    }, 
    status: Status { 
      experimental: false,
      standard_track: true, 
      deprecated: false 
    }
  },
  
  assignment: Compat { 
    name: "assignment",
    description: "Assignment (<code>x = y</code>)",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Assignment",
    spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-assignment-operators",
    tags: &["web-features:snapshot:ecmascript-1"],
    support: Support { 
      chrome: "1", 
      chrome_android: "1", 
      firefox: "1", 
      firefox_android: "1", 
      safari: "1", 
      safari_ios: "1", 
      opera: "3", 
      opera_android: "10.1", 
      ie: "3", 
      edge: "12", 
      deno: "1.0", 
      node: "0.10.0"
    },
    status: Status { 
      experimental: false,
      standard_track: true,
      deprecated: false
    }
  },
};
