

import fs from "node:fs"
import { add_compat_code_str } from "./add_compat_code_str.mjs"
 

export default async function generate_regular_expressions(params) {
  let impl = []
  let struct = []
  const name_mapper = { 
  }
  const response = await (await fetch('https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/regular_expressions.json')).json();
  const remote_functions = response.javascript.regular_expressions;
  Object.entries(remote_functions).forEach(([key, value]) => {
    add_compat_code_str(struct,impl,name_mapper,null,key,value);
  });
  fs.writeFileSync("../src/syntax/regular_expressions.rs", `
use super::{compat::Compat, support::Support};

pub struct RegularExpressions<'a> {
  ${struct.join("")}
}

pub const REGULAR_EXPRESSIONS: RegularExpressions = RegularExpressions {
    ${impl.join("")}
};`)
}