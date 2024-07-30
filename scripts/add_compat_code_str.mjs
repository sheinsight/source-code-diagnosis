
function getSupport(support) {
  if (Array.isArray(support)) {
    return support[0].version_added;
  } else if (typeof support === "object") {
    return support.version_added;
  } else {
    console.log(support);
    return "No"
    // throw new Error("support is not array or object");
  }
}




export function add_compat_code_str(struct,impl,name_mapper,parent_key,key,value) {
  const compat = value.__compat; 
  let p_k ;
  if (parent_key) {
    p_k = name_mapper[parent_key] || parent_key;
  }
  let name ;
  if (name_mapper.hasOwnProperty(key)) {
    name = name_mapper[key];
  } else {
      name = key;
  }
  if (parent_key) {
    name = `${p_k}_${name}`.toLocaleLowerCase();
  }
  if (compat) {
    const support = compat?.support;
    const chrome = getSupport(support.chrome);
    const edge = getSupport(support.edge);
    const firefox = getSupport(support.firefox);
    const ie = getSupport(support.ie);
    const node = getSupport(support.node);
    const safari = getSupport(support.safari);
    struct.push(`
      pub ${name}: Compat<'a>,
    `);
    impl.push(`
  ${name}: Compat {
    name: "${key}",
    description: r##"${compat.description}"##,
    mdn_url: "${compat.mdn_url}",
    spec_url: "${compat.spec_url}",
    support: Support {
      chrome: "${chrome}",
      edge: "${edge}",
      firefox: "${firefox}",
      ie: "${ie}",
      node: "${node}",
      safari: "${safari}",
    },
  },`);
  }


  if (Object.keys(value).length > 1) {
    Object.entries(value).filter(([key,_]) => key !== "__compat").forEach(([child_key,value]) => {
      add_compat_code_str(struct,impl,name_mapper,name,child_key,value);
    });
  }
 

}