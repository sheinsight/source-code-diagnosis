
import fs from "node:fs"
import { add_compat_code_str } from "./add_compat_code_str.mjs"




const data = [
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/addition.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/addition_assignment.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/assignment.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/async_function.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/async_generator_function.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/await.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/bitwise_and.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/bitwise_and_assignment.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/bitwise_not.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/bitwise_or.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/bitwise_or_assignment.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/bitwise_xor.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/bitwise_xor_assignment.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/class.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/comma.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/conditional.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/decrement.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/delete.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/destructuring.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/division.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/division_assignment.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/equality.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/exponentiation.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/exponentiation_assignment.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/function.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/generator_function.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/greater_than.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/greater_than_or_equal.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/grouping.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/import.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/import_meta.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/in.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/increment.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/inequality.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/instanceof.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/left_shift.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/left_shift_assignment.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/less_than.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/less_than_or_equal.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/logical_and.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/logical_and_assignment.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/logical_not.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/logical_or.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/logical_or_assignment.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/multiplication.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/multiplication_assignment.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/new.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/new_target.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/null.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/nullish_coalescing.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/nullish_coalescing_assignment.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/object_initializer.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/optional_chaining.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/property_accessors.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/remainder.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/remainder_assignment.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/right_shift.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/right_shift_assignment.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/spread.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/strict_equality.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/strict_inequality.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/subtraction.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/subtraction_assignment.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/super.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/this.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/typeof.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/unary_negation.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/unary_plus.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/unsigned_right_shift.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/unsigned_right_shift_assignment.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/void.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/yield.json",
  "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/yield_star.json",
]

export default async function generate_operators(params) {
  let struct = []
  let impl = []

  const name_mapper = {
    "await": "r#await",
    "in": "r#in",
    "super": "r#super_alias",
    "typeof": "r#typeof",
    "yield": "r#yield",
    "void": "r#void",
    "instanceof": "instance_of",
  }

  let response_list = await Promise.all(
    data.map(x => fetch(x).then(x => x.json()))
  )

  const operators = response_list.map(x => x.javascript.operators).reduce((acc, x) => {
    return Object.assign(acc, x)
  })

  Object.entries(operators).forEach(([key, value]) => {
    add_compat_code_str(struct,impl,name_mapper,null,key,value);
  });

  fs.writeFileSync("../src/syntax/operators.rs", `
use super::{compat::Compat, support::Support};

pub struct Operators<'a>{
    ${struct.join("")}
}

pub const OPERATORS: Operators = Operators { 
    ${impl.join("\n")}
};
    `)
}

 