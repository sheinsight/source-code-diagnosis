# source-code-diagnosis

This is a grocery store based on the `Rust` tool class, mainly used for various unique analysis of source code, supporting multi-threading.

## How to install

```bash
# pnpm
pnpm i @shined/source-code-diagnosis -D

# npm
npm i @shined/source-code-diagnosis -D

# yarn
yarn add @shined/source-code-diagnosis -D
```

## checkDangerStrings

Analyze the dangerous strings in the source code, usually used for third-party CDN detection.

```ts
import { checkDangerStrings } from "@shined/source-code-diagnosis";

let response = checkDangerStrings(
  ["bootcss.com", "bootcdn.com", "polyfill.com", "polyfill.io"],
  {
    cwd: "/Users/Pikachu/project",
    concurrency: 1,
  }
);
```

## checkModuleMemberUsage

Analyze the usage rate of module members, generally used to analyze the number of times the exported members of a third-party package are used.

```ts
import { checkModuleMemberUsage } from "@shined/source-code-diagnosis";
const response = checkModuleMemberUsage(["antd"], {
  cwd: "/Users/Pikachu/project",
});
```

## checkBrowserSupported

```ts
import { checkBrowserSupported } from "@shined/source-code-diagnosis";
const response = checkBrowserSupported("chrome >= 100", {
  cwd: "/Users/Pikachu/project",
});
```

## checkBrowserSupportedWithSourceCode

```ts
import { checkBrowserSupportedWithSourceCode } from "@shined/source-code-diagnosis";
const response = checkBrowserSupportedWithSourceCode(
  "chrome >= 100",
  "function hello(){}"
);
```

## checkDependents

Detecting dependencies on a file

```ts
const response = checkDependents("/Users/Pikachu/project/src/utils/index.ts", {
  cwd: "/Users/Pikachu/project",
});
```

Of course you can also configure alias and modules

```ts
const response = checkDependents("/Users/Pikachu/project/src/utils/index.ts", {
  cwd: "/Users/Pikachu/project",
  alias: {
    "@": "/Users/Pikachu/project/src",
  },
  modules: ["node_modules", "web_modules"],
});
```

## checkDetectCycle

Detecting circular dependencies in the project

```ts
import { checkDetectCycle } from "@shined/source-code-diagnosis";
const response = checkDetectCycle({
  cwd: "/Users/Pikachu/project",
});
```
