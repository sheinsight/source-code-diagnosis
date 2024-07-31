use super::compat::{Compat, Status, Support};

pub struct Operators<'a> {
  pub addition_assignment: Compat<'a>,

  pub addition: Compat<'a>,

  pub assignment: Compat<'a>,

  pub spread: Compat<'a>,

  pub spread_in_arrays: Compat<'a>,

  pub spread_in_object_literals: Compat<'a>,

  pub spread_in_function_calls: Compat<'a>,

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
    spread: Compat { 
      name: "spread", 
      description: "Spread syntax (...)", 
      mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Spread_syntax", 
      spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#prod-SpreadElement", 
      tags: &[
        "web-features:snapshot:ecmascript-2015"
      ], 
      support: Support { 
        chrome: "46", 
        chrome_android: "46", 
        firefox: "16", 
        firefox_android: "16", 
        safari: "8", 
        safari_ios: "8", 
        opera: "37", 
        opera_android: "37", 
        ie: "0", 
        edge: "12", 
        deno: "1.0", 
        node: "5.0.0"
      }, 
      status: Status { 
        experimental: false,
        standard_track: true,
        deprecated: false 
      }
    },
    spread_in_object_literals: Compat { 
      name: "spread_in_object_literals", 
      description: "Spread in object literals", 
      mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Spread_syntax#Spread_in_object_literals", 
      spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#prod-PropertyDefinition", 
      tags: &[], 
      support: Support { 
        chrome: "60", 
        chrome_android: "60", 
        firefox: "55", 
        firefox_android: "55", 
        safari: "11.1", 
        safari_ios: "11.1", 
        opera: "60", 
        opera_android: "60", 
        ie: "0", 
        edge: "60", 
        deno: "1.0", 
        node: "8.3.0" 
      }, 
      status: Status { 
        experimental: false, 
        standard_track: true,
        deprecated: false 
      }
    },

    spread_in_arrays: Compat { 
      name: "spread_in_arrays", 
      description: "Spread in array literals", 
      mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Spread_syntax#Spread_in_array_literals", 
      spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#prod-SpreadElement", 
      tags: &[
        "web-features:snapshot:ecmascript-2015"
      ], 
      support: Support { 
        chrome: "46", 
        chrome_android: "46", 
        firefox: "16", 
        firefox_android: "16", 
        safari: "8", 
        safari_ios: "8", 
        opera: "37", 
        opera_android: "37", 
        ie: "0", 
        edge: "12", 
        deno: "1.0", 
        node: "5.0.0"
      }, 
      status: Status { 
        experimental: false, 
        standard_track: true, 
        deprecated: false 
      }
    },

    spread_in_function_calls: Compat { 
      name: "spread_in_function_calls", 
      description: "Spread in function calls", 
      mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Spread_syntax#Spread_in_function_calls", 
      spec_url: "https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#prod-ArgumentList", 
      tags: &[
        "web-features:snapshot:ecmascript-2015"
      ], 
      support: Support { 
        chrome: "46", 
        chrome_android: "46", 
        firefox: "27", 
        firefox_android: "27", 
        safari: "8", 
        safari_ios: "8", 
        opera: "46", 
        opera_android: "46", 
        ie: "0", 
        edge: "12", 
        deno: "1.0", 
        node: "5.0.0"
      }, 
      status: Status { 
        experimental: false, 
        standard_track: true, 
        deprecated: false 
      }
    },
};
