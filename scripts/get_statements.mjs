

import fs from "node:fs"
import { add_compat_code_str } from "./add_compat_code_str.mjs"
 

export default async function generate_statements(params) {
  let impl = []
  let struct = []
  const name_mapper = {
    "break":"r#break",
    "class":"r#class",
    "const":"r#const",
    "continue":"r#continue",
    "debugger":"r#debugger",
    "export":"r#export",
    "for":"r#for",
    "function":"r#function",
    "import":"r#import",
    "label":"r#label",
    "let":"r#let",
    "return":"r#return",
    "switch":"r#switch",
    "throw":"r#throw",
    "var":"r#var",
    "while":"r#while",
    "with":"r#with",
    "IteratorResult_object":"iterator_result_object",
  }
  const response = await (await fetch('https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/statements.json')).json();
  const remote_functions = response.javascript.statements;
  Object.entries(remote_functions).forEach(([key, value]) => {
    add_compat_code_str(struct,impl,name_mapper,null,key,value);
  });
  fs.writeFileSync("../src/syntax/statements.rs", `
use super::{compat::Compat, support::Support};

pub struct Statements<'a> {
  ${struct.join("")}
}

pub const STATEMENTS: Statements = Statements {
    ${impl.join("")}
};`)
}
 