# source-code-diagnosis

这是一个基于 `Rust` 工具类的杂货铺，主要用于对源码进行各种唯独的分析，支持多线程。

## 如何安装

```bash
# pnpm
pnpm i @shined/source-code-diagnosis -D

# npm
npm i @shined/source-code-diagnosis -D

# yarn
yarn add @shined/source-code-diagnosis -D
```

## getUsageOfDangerStrings

分析源码中存在的危险字符串，一般用于第三方 CDN 检测

```ts
import { getDangerStringsUsage } from "@shined/source-code-diagnosis";

let response = getDangerStringsUsage(
  ["bootcss.com", "bootcdn.com", "polyfill.com", "polyfill.io"],
  {
    cwd: "/Users/Pikachu/project",
    concurrency: 1,
  }
);
```

## getModuleMemberUsage

分析模块成员的使用率，一般用于分析一个第三方包的导出成员被使用的次数

```ts
const response = getModuleMemberUsage(["antd"], {
  cwd: "/Users/Pikachu/project",
});
```
