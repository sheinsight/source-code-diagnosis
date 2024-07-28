
import fs from "node:fs"
import { add_compat_code_str } from "./add_compat_code_str.mjs"
 

 export default async function generate_grammar(params) {
  let struct = []
  let impl = []

  const name_mapper = {
    hashbang_comments: "hash_bang_comments",
  }
  const response = await (await fetch('https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/grammar.json')).json();
  const remote_functions = response.javascript.grammar;
  Object.entries(remote_functions).forEach(([key, value]) => {
    add_compat_code_str(struct,impl,name_mapper,null,key,value);
  });
  fs.writeFileSync("../src/syntax/grammar.rs", `
use super::{compat::Compat, support::Support};

pub struct Grammar<'a>{
    ${struct.join("")}
}

pub const GRAMMAR: Grammar = Grammar { 
    ${impl.join("\n")}
};
    `)
 }