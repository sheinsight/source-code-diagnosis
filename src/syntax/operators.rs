use super::compat::{Compat, Status, Support};
use crate::str;

#[derive(Debug)]
pub struct Operators {

  pub addition_assignment: Compat,

  pub addition: Compat,

  pub assignment: Compat,

  pub spread: Compat,

  pub spread_in_arrays: Compat,

  pub spread_in_function_calls: Compat,

  pub spread_in_object_literals: Compat,

  pub exponentiation_assignment: Compat,

  pub r#null: Compat,

  pub nullish_coalescing: Compat,

  pub nullish_coalescing_assignment: Compat,

}


pub fn create_operators() -> Operators{
  
  Operators {

    addition_assignment: Compat {
      name: str!("addition_assignment"),
      description: str!("Addition assignment (<code>x += y</code>)"),
      mdn_url:
        str!("https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Addition_assignment"),
      spec_url: str!("https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-assignment-operators"),
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
          opera_android: str!("3"),
          ie: str!("3"),
          edge: str!("12"),
          deno: str!("1.0"),
          node: str!("0.10.0"),
      },
      status: Status {
        experimental: false,
        standard_track: true,
        deprecated: false,
      },
    },
  
    addition: Compat {
      name: str!("addition"),
      description: str!("Addition (<code>+</code>)"), 
      mdn_url: str!("https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Addition"),
      spec_url: str!("https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-addition-operator-plus"), 
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
  
    assignment: Compat {
      name: str!("assignment"),
      description: str!("Assignment (<code>=</code>)"),
      mdn_url: str!("https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Assignment"),
      spec_url: str!("https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-assignment-operators"),
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

    spread: Compat { 
      name: str!("spread"), 
      description: str!("Spread syntax (...)"), 
      mdn_url: str!("https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Spread_syntax"), 
      spec_url: str!("https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#prod-SpreadElement"), 
      tags: vec![
        str!("web-features:snapshot:ecmascript-2015")
      ], 
      support: Support { 
        chrome: str!("46"), 
        chrome_android: str!("46"), 
        firefox: str!("16"), 
        firefox_android: str!("16"), 
        safari: str!("8"), 
        safari_ios: str!("8"), 
        opera: str!("37"), 
        opera_android: str!("37"), 
        ie: str!("0"), 
        edge: str!("12"), 
        deno: str!("1.0"), 
        node: str!("5.0.0")
      }, 
      status: Status { 
        experimental: false,
        standard_track: true,
        deprecated: false 
      }
    },

    spread_in_object_literals: Compat { 
      name: str!("spread_in_object_literals"), 
      description: str!("Spread in object literals"), 
      mdn_url: str!("https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Spread_syntax#Spread_in_object_literals"), 
      spec_url: str!("https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#prod-PropertyDefinition"), 
      tags: vec![], 
      support: Support { 
        chrome: str!("60"),
        chrome_android: str!("60"),
        firefox: str!("55"),
        firefox_android: str!("55"),
        safari: str!("11.1"),
        safari_ios: str!("11.1"),
        opera: str!("60"),
        opera_android: str!("60"),
        ie: str!("0"),
        edge: str!("60"),
        deno: str!("1.0"),
        node: str!("8.3.0")
      }, 
      status: Status { 
        experimental: false, 
        standard_track: true,
        deprecated: false 
      }
    },
  
    spread_in_arrays: Compat { 
      name: str!("spread_in_arrays"),
      description: str!("Spread in array literals"),
      mdn_url: str!("https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Spread_syntax#Spread_in_array_literals"),
      spec_url: str!("https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#prod-SpreadElement"),
      tags: vec![
        str!("web-features:snapshot:ecmascript-2015")
      ], 
      support: Support { 
        chrome: str!("46"),
        chrome_android: str!("46"),
        firefox: str!("16"),
        firefox_android: str!("16"),
        safari: str!("8"),
        safari_ios: str!("8"),
        opera: str!("37"),
        opera_android: str!("37"),
        ie: str!("0"),
        edge: str!("12"),
        deno: str!("1.0"),
        node: str!("5.0.0")
      }, 
      status: Status { 
        experimental: false, 
        standard_track: true, 
        deprecated: false 
      }
    },
  
    spread_in_function_calls: Compat { 
      name: str!("spread_in_function_calls"),
      description: str!("Spread in function calls"),
      mdn_url: str!("https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Spread_syntax#Spread_in_function_calls"), 
      spec_url: str!("https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#prod-ArgumentList"), 
      tags: vec![
        str!("web-features:snapshot:ecmascript-2015")
      ], 
      support: Support { 
        chrome: str!("46"),
        chrome_android: str!("46"),
        firefox: str!("27"),
        firefox_android: str!("27"),
        safari: str!("8"),
        safari_ios: str!("8"),
        opera: str!("46"),
        opera_android: str!("46"),
        ie: str!("0"),
        edge: str!("12"),
        deno: str!("1.0"),
        node: str!("5.0.0")
      }, 
      status: Status { 
        experimental: false, 
        standard_track: true, 
        deprecated: false 
      }
    },
  
    exponentiation_assignment: Compat { 
      name: str!("exponentiation_assignment"),
      description: str!("Exponentiation assignment (<code>x **= y</code>)"),
      mdn_url: str!("https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Exponentiation_assignment"),
      spec_url: str!("https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-assignment-operators"),
      tags: vec![
        str!("web-features:snapshot:ecmascript-2016")
      ], 
      support: Support { 
        chrome: str!("52"),
        chrome_android: str!("52"),
        firefox: str!("52"),
        firefox_android: str!("52"),
        safari: str!("10.1"),
        safari_ios: str!("10.1"),
        opera: str!("52"),
        opera_android: str!("52"),
        ie: str!("0"),
        edge: str!("14"),
        deno: str!("1.0"),
        node: str!("7.0.0")
      }, 
      status: Status { 
        experimental: false, 
        standard_track: true, 
        deprecated: false 
      }
    },

    r#null: Compat { 
      name: str!("null"), 
      description: str!(""), 
      mdn_url: str!("https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/null"), 
      spec_url: str!("https://tc39.es/ecma262/multipage/overview.html#sec-null-value"), 
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
        ie: str!("0"), 
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

    nullish_coalescing_assignment: Compat { 
      name: str!("nullish_coalescing_assignment"), 
      description: str!("Nullish coalescing assignment (<code>x ??= y</code>)"), 
      mdn_url: str!("https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Nullish_coalescing_assignment"), 
      spec_url: str!("https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#sec-assignment-operators"), 
      tags: vec![
        str!("web-features:snapshot:ecmascript-2020")
      ], 
      support: Support { 
        chrome: str!("85"), 
        chrome_android: str!("85"), 
        firefox: str!("79"), 
        firefox_android: str!("79"), 
        safari: str!("14"), 
        safari_ios: str!("14"), 
        opera: str!("85"), 
        opera_android: str!("85"), 
        ie: str!("0"), 
        edge: str!("85"), 
        deno: str!("1.2"), 
        node: str!("15.0.0")
      }, 
      status: Status { 
        experimental: false, 
        standard_track: true, 
        deprecated: false 
      }
    },

    nullish_coalescing: Compat { 
      name: str!("nullish_coalescing"), 
      description: str!("Nullish coalescing operator (<code>??</code>)"), 
      mdn_url: str!("https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Nullish_coalescing"), 
      spec_url: str!("https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#prod-CoalesceExpression"), 
      tags: vec![
        str!("web-features:snapshot:ecmascript-2020")
      ], 
      support: Support { 
        chrome: str!("80"), 
        chrome_android: str!("80"), 
        firefox: str!("72"), 
        firefox_android: str!("72"), 
        safari: str!("13.1"), 
        safari_ios: str!("13.1"), 
        opera: str!("80"), 
        opera_android: str!("80"), 
        ie: str!("0"), 
        edge: str!("80"), 
        deno: str!("1.0"), 
        node: str!("14.0.0")
      }, 
      status: Status { 
        experimental: false, 
        standard_track: true, 
        deprecated: false 
      }
    },

  }
}


