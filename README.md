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

## getUsageOfDangerStrings

Analyze the dangerous strings in the source code, usually used for third-party CDN detection.

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

Analyze the usage rate of module members, generally used to analyze the number of times the exported members of a third-party package are used.

```ts
const response = getModuleMemberUsage(["antd"], {
  cwd: "/Users/Pikachu/project",
});
```

## checkSupportBrowser

```ts
const response = checkBrowserSupported("chrome 100", {
  cwd: "/Users/Pikachu/project",
});
```
