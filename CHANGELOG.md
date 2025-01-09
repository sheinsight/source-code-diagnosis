# 0.0.90

## 🐞 fix

fix: module_member_usage

```jsx
import { Message } from "antd";
export default () => {
  return [
    {
      render: (d) => {
        if (d.status === 2 && d.filePath) {
          return (
            <a
              onClick={async () => {
                Message.success("xxx", 5);
              }}
            >
              hello
            </a>
          );
        }
        return <div>{d.exportName}</div>;
      },
    },
  ];
};
```

before because the `Message` is in `<a>` , so the result member name is `a`.

but now because the `Message` is in `event handler` , so the result member name is `Message`;

# 0.0.89

## 🐞 fix

fix: module_member_usage

```jsx
import { Table as STable } from "antd";
class View extends React.Component {
  render() {
    return (
      <div className={style.wrap}>
        <STable
          keygen="id"
          columns={columns}
          data={modalInfo.details}
          style={{ minHeight: 100, maxHeight: 400 }}
          size="small"
        />
      </div>
    );
  }
}
```

before the result member name is `STable` , but now is `Table`;

# 0.0.88

## 🐞 fix

fix: module_member_usage

```jsx
import React, { useState } from "react";
import { Upload as Up } from "antd";
export function Upload({ getRes, image, ...props }) {
  const [value, setValue] = useState("");
  const Comp = image ? Up.Image : Up;
  const children = image ? null : (
    <ButtonWithIcon name="upload">{t("上传")}</ButtonWithIcon>
  );
  return <Comp>{children}</Comp>;
}
```

before the result member name is `Up` , but now is `Updoad`;

## 🔥 refactor

other

`is_in` remove max_depth args, i think it's ok.

# 0.0.87

## 🐞 fix

### fix: may be node_modules

the local_patterns before is `[".", "../", "/","node_modules", "@/"]`, but now it is `[".", "../", "/", "@/"]`.

remove `node_modules` from local_patterns.

the `TargetMetadata.may_be` is `true` when the target is in `node_modules` .

.e.g

- "node_modules/antd/lib/Button" => true
- "@babel/core/lib/something" => true
- "@/src/index.ts" => false
- "lodash/cloneDeep" => false
- "demo/node_modules/antd/lib/Button" => true

### fix: main_module_name

from import source, get npm module name.

.e.g

- "@babel/core/lib/something" => "@babel/core"
- "lodash/cloneDeep" => "lodash"
- "node_modules/antd/lib/Button" => "antd"

### fix: get_graph

where `get_graph` used to return only `import` deps , it now returns `import` and `redirect export` deps

- add `export *` to graph.
- add `export * as namespace` to graph.
