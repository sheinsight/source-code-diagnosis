
import fs from "node:fs"
import { add_compat_code_str } from "./add_compat_code_str.mjs"




export default async function generate_classes(params) {
  let struct = []
  let impl = []

  const name_mapper = {
    static: "r#static",
    constructor: "r#constructor",
  }
  const response = await (await fetch('https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/classes.json')).json();
  const remote_functions = response.javascript.classes;
  Object.entries(remote_functions).forEach(([key, value]) => {
    add_compat_code_str(struct,impl,name_mapper,null,key,value);
  });
  fs.writeFileSync("../src/syntax/classes.rs", `
use super::{compat::Compat, support::Support};

pub struct Classes<'a>{
    ${struct.join("")}
}

pub const CLASSES: Classes = Classes { 
    ${impl.join("\n")}
};
    `)
}

 