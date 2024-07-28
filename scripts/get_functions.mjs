
import fs from "node:fs"
import { add_compat_code_str } from "./add_compat_code_str.mjs"



export default async function generate_functions(params) {
  let struct = []
  let impl = []

  const name_mapper = {
    "@@iterator":"iterator",
  }
  const response = await (await fetch('https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/functions.json')).json();
  const remote_functions = response.javascript;
  Object.entries(remote_functions).forEach(([key, value]) => {
    add_compat_code_str(struct,impl,name_mapper,null,key,value);
  });
  fs.writeFileSync("../src/syntax/functions.rs", `
use super::{compat::Compat, support::Support};

pub struct Functions<'a>{
    ${struct.join("")}
}

pub const FUNCTIONS: Functions = Functions { 
    ${impl.join("\n")}
};
    `)
}
 