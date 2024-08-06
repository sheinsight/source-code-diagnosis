use std::marker::PhantomData;

use oxc_ast::{visit::walk, AstKind, Visit};
use serde_json::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox}, // operators::common_trait::CommonTrait,
};

pub struct ArrowFunctionsVisitor<'a> {
  pub cache: Vec<CompatBox>,
  parent_stack: Vec<AstKind<'a>>,
  _phantom: PhantomData<&'a ()>,
  compat: Compat,
}

impl CommonTrait for ArrowFunctionsVisitor<'_> {
  fn get_cache(&self) -> Vec<CompatBox> {
    self.cache.clone()
  }
}

impl Default for ArrowFunctionsVisitor<'_> {
  fn default() -> Self {
    let compat: Compat =
      from_str(include_str!("./arrow_functions.json")).unwrap();
    Self {
      cache: Vec::new(),
      parent_stack: Vec::new(),
      _phantom: PhantomData {},
      compat: compat,
    }
  }
}

impl<'a> Visit<'a> for ArrowFunctionsVisitor<'a> {
  fn enter_node(&mut self, kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.push(kind);
  }

  fn leave_node(&mut self, _kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.pop();
  }

  fn visit_arrow_function_expression(
    &mut self,
    it: &oxc_ast::ast::ArrowFunctionExpression<'a>,
  ) {
    self.cache.push(CompatBox::new(&it.span, &self.compat));
    walk::walk_arrow_function_expression(self, it);
  }
}

#[cfg(test)]
mod tests {
  use std::{env::current_dir, fs};

  use oxc_allocator::Allocator;

  use crate::syntax::utils::t;

  use super::*;

  #[test]
  fn should_ok_when_arguments_global_undeclared() {
    let source_code = vec![
      r##"
      function fn() {
        var foo = () => {
          return arguments;
        };
      }
      var bar = () => arguments;
      var baz = () => () => arguments;
    "##,
    ];

    source_code.iter().for_each(|x| {
      let allocator = Allocator::default();
      let x = t(x, &allocator, ArrowFunctionsVisitor::default);
      assert!(x.iter().any(|c| c.compat.name == "arrow_functions"));
    });
  }

  #[test]
  fn should_ok_when_arguments_global_var() {
    let source_code = vec![
      r##"
      var arguments = 1;
      function fn() {
        var foo = () => {
          return arguments;
        };
      }

      var bar = () => arguments;

      var baz = () => () => arguments;
    "##,
    ];

    source_code.iter().for_each(|x| {
      let allocator = Allocator::default();
      let x = t(x, &allocator, ArrowFunctionsVisitor::default);
      assert!(x.iter().any(|c| c.compat.name == "arrow_functions"));
    });
  }

  #[test]
  fn should_ok_when_arguments() {
    let source_code = vec![
      r##"
        function one() {
          var inner = () => arguments;
          return [].slice.call(inner());
        }
      "##,
      r##"
        function two() {
        var inner = () => arguments;

        var another = function () {
          var inner2 = () => arguments;
        };

        return [].slice.call(inner());
      }
      "##,
      r##"
        function three() {
          var fn = () => arguments[0] + "bar";
          return fn();
        }
      "##,
      r##"
        function four() {
          var fn = () => arguments[0].foo + "bar";
          return fn();
        }
      "##,
      r##"
        function five(obj) {
          var fn = () => obj.arguments[0].foo + "bar";
          return fn();
        }
      "##,
      r##"
        function six(obj) {
          var fn = () => {
            var fn2 = function () {
              return arguments[0];
            };
            return fn2("foobar");
          };
          return fn();
        }
      "##,
      r##"
        var seven = () => {
            var arguments = 1;
            return arguments;
        };
      "##,
      r##"
        var eight = () => {
            var arguments = 1;
            return () => arguments;
        };
      "##,
      r##"
        function nine() {
            var arguments = 1;
            var foo = () => {
              return arguments;
            };
        }
      "##,
      r##"
        var eleven = () => {
          var arguments = 2;
          return function () {
            return () => arguments;
          }
        };
      "##,
      r##"
        var twelve = () => {
          var arguments = 2;
          return class {
            m() { return () => arguments; }
          }
        };
      "##,
    ];

    source_code.iter().for_each(|x| {
      let allocator = Allocator::default();
      let x = t(x, &allocator, ArrowFunctionsVisitor::default);
      assert!(x.iter().any(|c| c.compat.name == "arrow_functions"));
    });
  }

  #[test]
  fn should_ok_when_assumption_newable_arrow_functions_false_basic() {
    let source_code = vec![
      r##"
        function foo() {
          arr.map(x => x * x);
          var f = (x, y) => x * y;
          (function () {
            return () => this;
          })();
          return {
            g: () => this
          }
        }
    "##,
    ];

    source_code.iter().for_each(|x| {
      let allocator = Allocator::default();
      let x = t(x, &allocator, ArrowFunctionsVisitor::default);
      assert!(x.iter().any(|c| c.compat.name == "arrow_functions"));
    });
  }

  #[test]
  fn should_ok_when_assumption_newable_arrow_functions_false_naming() {
    let source_code = vec![
      r##"
        [].map(x => x);

        const f = x => x

        const o = { k: x => x }
    "##,
    ];

    source_code.iter().for_each(|x| {
      let allocator = Allocator::default();
      let x = t(x, &allocator, ArrowFunctionsVisitor::default);
      assert!(x.iter().any(|c| c.compat.name == "arrow_functions"));
    });
  }

  #[test]
  fn should_ok_when_assumption_newable_arrow_functions_false_self_referential()
  {
    let source_code = vec![
      r##"
        var fooCalls = [];
        var jumpTable = (name, arg) => {
          if (jumpTable[name]) {
            jumpTable[name](arg);
          }
        };
        Object.assign(jumpTable, {
          foo(arg) {
            fooCalls.push(arg);
          },
        });
        jumpTable("foo", "bar");

        expect(fooCalls[0]).toBe("bar");
    "##,
    ];

    source_code.iter().for_each(|x| {
      let allocator = Allocator::default();
      let x = t(x, &allocator, ArrowFunctionsVisitor::default);
      assert!(x.iter().any(|c| c.compat.name == "arrow_functions"));
    });
  }

  #[test]
  fn should_ok_when_default_parameters() {
    let source_code = vec![
      r##"
        var some = (count = "30") => {
          console.log("count", count);
        };

        var collect = (since = 0, userid) => {
          console.log(userid);
        };
    "##,
    ];

    source_code.iter().for_each(|x| {
      let allocator = Allocator::default();
      let x = t(x, &allocator, ArrowFunctionsVisitor::default);
      assert!(x.iter().any(|c| c.compat.name == "arrow_functions"));
    });
  }

  #[test]
  fn should_ok_when_destructuring_parameters() {
    let source_code = vec![
      r##"
        var a = ({ target }) => console.log(target);
        a({ target: "I am a target" });
    "##,
    ];

    source_code.iter().for_each(|x| {
      let allocator = Allocator::default();
      let x = t(x, &allocator, ArrowFunctionsVisitor::default);
      assert!(x.iter().any(|c| c.compat.name == "arrow_functions"));
    });
  }

  #[test]
  fn should_ok_when_empty_arguments() {
    let source_code = vec![
      r##"
        var t = () => 5 + 5;
    "##,
    ];

    source_code.iter().for_each(|x| {
      let allocator = Allocator::default();
      let x = t(x, &allocator, ArrowFunctionsVisitor::default);
      assert!(x.iter().any(|c| c.compat.name == "arrow_functions"));
    });
  }

  #[test]
  fn should_ok_when_empty_block() {
    let source_code = vec![
      r##"
        var t = () => {};
    "##,
    ];

    source_code.iter().for_each(|x| {
      let allocator = Allocator::default();
      let x = t(x, &allocator, ArrowFunctionsVisitor::default);
      assert!(x.iter().any(|c| c.compat.name == "arrow_functions"));
    });
  }

  #[test]
  fn should_ok_when_expression() {
    let source_code = vec![
      r##"
        arr.map((x) => x * x);
    "##,
    ];

    source_code.iter().for_each(|x| {
      let allocator = Allocator::default();
      let x = t(x, &allocator, ArrowFunctionsVisitor::default);
      assert!(x.iter().any(|c| c.compat.name == "arrow_functions"));
    });
  }

  #[test]
  fn should_ok_when_implicit_var_arguments() {
    let source_code = vec![
      r##"
        var arguments = [1, 2, 3];
        var arr = (n) => arguments[0];

        arr(4); // 1

        function foo(n) {
        var f = () => arguments[0] + n; // foo's implicit arguments binding. arguments[0] is n
        return f();
        }
    "##,
    ];

    source_code.iter().for_each(|x| {
      let allocator = Allocator::default();
      let x = t(x, &allocator, ArrowFunctionsVisitor::default);
      assert!(x.iter().any(|c| c.compat.name == "arrow_functions"));
    });
  }

  #[test]
  fn should_ok_when_inside_call() {
    let source_code = vec![
      r##"
        arr.map((i) => i + 1);
    "##,
    ];

    source_code.iter().for_each(|x| {
      let allocator = Allocator::default();
      let x = t(x, &allocator, ArrowFunctionsVisitor::default);
      assert!(x.iter().any(|c| c.compat.name == "arrow_functions"));
    });
  }

  #[test]
  fn should_ok_multiple_arguments() {
    let source_code = vec![
      r##"
        var t = (i, x) => i * x;
    "##,
    ];

    source_code.iter().for_each(|x| {
      let allocator = Allocator::default();
      let x = t(x, &allocator, ArrowFunctionsVisitor::default);
      assert!(x.iter().any(|c| c.compat.name == "arrow_functions"));
    });
  }

  #[test]
  fn should_ok_when_nested() {
    let source_code = vec![
      r##"
        module.exports = {
          init: function () {
            return new Promise((resolve, reject) => {
              MongoClient.connect(config.mongodb, (err, db) => {
                if (err) {
                  return reject(err);
                }
                this.db = db;
                resolve(this);
              });
            });
          },
        };
    "##,
    ];

    source_code.iter().for_each(|x| {
      let allocator = Allocator::default();
      let x = t(x, &allocator, ArrowFunctionsVisitor::default);
      assert!(x.iter().any(|c| c.compat.name == "arrow_functions"));
    });
  }

  #[test]
  fn should_ok_when_paran_insertion() {
    let source_code = vec![
      r##"
        var t = (i) => i * 5;
    "##,
    ];

    source_code.iter().for_each(|x| {
      let allocator = Allocator::default();
      let x = t(x, &allocator, ArrowFunctionsVisitor::default);
      assert!(x.iter().any(|c| c.compat.name == "arrow_functions"));
    });
  }

  #[test]
  fn should_ok_when_self_referential() {
    let source_code = vec![
      r##"
        var fooCalls = [];
        var jumpTable = (name, arg) => {
          if (jumpTable[name]) {
            jumpTable[name](arg);
          }
        };
        Object.assign(jumpTable, {
          foo(arg) {
            fooCalls.push(arg);
          },
        });
        jumpTable("foo", "bar");

        expect(fooCalls[0]).toBe("bar");
    "##,
    ];

    source_code.iter().for_each(|x| {
      let allocator = Allocator::default();
      let x = t(x, &allocator, ArrowFunctionsVisitor::default);
      assert!(x.iter().any(|c| c.compat.name == "arrow_functions"));
    });
  }

  #[test]
  fn should_ok_when_single_argument() {
    let source_code = vec![
      r##"
        var t = (i) => i * 5;
    "##,
    ];

    source_code.iter().for_each(|x| {
      let allocator = Allocator::default();
      let x = t(x, &allocator, ArrowFunctionsVisitor::default);
      assert!(x.iter().any(|c| c.compat.name == "arrow_functions"));
    });
  }

  #[test]
  fn should_ok_when_spec() {
    let source_code = vec![
      r##"
        function foo() {
          arr.map((x) => x * x);
          var f = (x, y) => x * y;
          (function () {
            return () => this;
          })();
          return {
            g: () => this,
          };
        }
    "##,
    ];

    source_code.iter().for_each(|x| {
      let allocator = Allocator::default();
      let x = t(x, &allocator, ArrowFunctionsVisitor::default);
      assert!(x.iter().any(|c| c.compat.name == "arrow_functions"));
    });
  }

  #[test]
  fn should_ok_when_statement() {
    let source_code = vec![
      r##"
        nums.forEach((v) => {
          if (v % 5 === 0) {
            fives.push(v);
          }
        });
    "##,
    ];

    source_code.iter().for_each(|x| {
      let allocator = Allocator::default();
      let x = t(x, &allocator, ArrowFunctionsVisitor::default);
      assert!(x.iter().any(|c| c.compat.name == "arrow_functions"));
    });
  }

  #[test]
  fn should_ok_when_super_call() {
    let source_code = vec![
      r##"
        class Bar extends Foo {
          constructor() {
              let f = () => super();
              f();
          }
        }
    "##,
    ];

    source_code.iter().for_each(|x| {
      let allocator = Allocator::default();
      let x = t(x, &allocator, ArrowFunctionsVisitor::default);
      assert!(x.iter().any(|c| c.compat.name == "arrow_functions"));
    });
  }

  #[test]
  fn should_ok_when_super_prop() {
    let source_code = vec![
      r##"
        class Bar extends Foo {
          constructor() {
              let f = () => super.x;
              f();
          }
        }

    "##,
    ];

    source_code.iter().for_each(|x| {
      let allocator = Allocator::default();
      let x = t(x, &allocator, ArrowFunctionsVisitor::default);
      assert!(x.iter().any(|c| c.compat.name == "arrow_functions"));
    });
  }

  #[test]
  fn should_ok_when_this() {
    let source_code = vec![
      r##"
        function b() {
          var t = (x) => this.x + x;
        }

        class Foo extends function () {} {
          constructor() {
            var foo = () => this;

            if (true) {
              console.log(super(), foo());
            } else {
              super();
              console.log(foo());
            }
          }
        }
    "##,
    ];

    source_code.iter().for_each(|x| {
      let allocator = Allocator::default();
      let x = t(x, &allocator, ArrowFunctionsVisitor::default);
      assert!(x.iter().any(|c| c.compat.name == "arrow_functions"));
    });
  }
}
