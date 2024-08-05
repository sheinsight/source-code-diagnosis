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

- [ ] babel-plugin-transform-arrow-functions
- [ ] babel-plugin-transform-async-generator-functions
- [ ] babel-plugin-transform-async-to-generator
- [ ] babel-plugin-transform-block-scoped-functions
- [ ] babel-plugin-transform-block-scoping
- [ ] babel-plugin-transform-class-properties
- [ ] babel-plugin-transform-class-static-block
- [ ] babel-plugin-transform-classes
- [ ] babel-plugin-transform-computed-properties
- [ ] babel-plugin-transform-destructuring
- [ ] babel-plugin-transform-dotall-regex
- [ ] babel-plugin-transform-duplicate-keys
- [ ] babel-plugin-transform-dynamic-import
- [ ] babel-plugin-transform-exponentiation-operator
- [ ] babel-plugin-transform-export-namespace-from
- [ ] babel-plugin-transform-flow-comments
- [ ] babel-plugin-transform-flow-strip-types
- [ ] babel-plugin-transform-for-of
- [ ] babel-plugin-transform-function-name
- [ ] babel-plugin-transform-instanceof
- [ ] babel-plugin-transform-jscript
- [ ] babel-plugin-transform-json-strings
- [ ] babel-plugin-transform-literals
- [ ] babel-plugin-transform-logical-assignment-operators
- [ ] babel-plugin-transform-member-expression-literals
- [ ] babel-plugin-transform-modules-amd
- [ ] babel-plugin-transform-modules-commonjs
- [ ] babel-plugin-transform-modules-systemjs
- [ ] babel-plugin-transform-modules-umd
- [ ] babel-plugin-transform-named-capturing-groups-regex
- [ ] babel-plugin-transform-new-target
- [ ] babel-plugin-transform-nullish-coalescing-operator
- [ ] babel-plugin-transform-numeric-separator
- [ ] babel-plugin-transform-object-assign
- [ ] babel-plugin-transform-object-rest-spread
- [ ] babel-plugin-transform-object-set-prototype-of-to-assign
- [ ] babel-plugin-transform-object-super
- [ ] babel-plugin-transform-optional-catch-binding
- [ ] babel-plugin-transform-optional-chaining
- [ ] babel-plugin-transform-parameters
- [ ] babel-plugin-transform-private-methods
- [ ] babel-plugin-transform-private-property-in-object
- [ ] babel-plugin-transform-property-literals
- [ ] babel-plugin-transform-property-mutators
- [ ] babel-plugin-transform-proto-to-assign
- [ ] babel-plugin-transform-react-constant-elements
- [ ] babel-plugin-transform-react-display-name
- [ ] babel-plugin-transform-react-inline-elements
- [ ] babel-plugin-transform-react-jsx
- [ ] babel-plugin-transform-react-jsx-compat
- [ ] babel-plugin-transform-react-jsx-development
- [ ] babel-plugin-transform-react-jsx-self
- [ ] babel-plugin-transform-react-jsx-source
- [ ] babel-plugin-transform-react-pure-annotations
- [ ] babel-plugin-transform-regenerator
- [ ] babel-plugin-transform-reserved-words
- [ ] babel-plugin-transform-runtime
- [ ] babel-plugin-transform-shorthand-properties
- [ ] babel-plugin-transform-spread
- [ ] babel-plugin-transform-sticky-regex
- [ ] babel-plugin-transform-strict-mode
- [ ] babel-plugin-transform-template-literals
- [ ] babel-plugin-transform-typeof-symbol
- [ ] babel-plugin-transform-typescript
- [ ] babel-plugin-transform-unicode-escapes
- [ ] babel-plugin-transform-unicode-property-regex
- [ ] babel-plugin-transform-unicode-regex
- [ ] babel-plugin-transform-unicode-sets-regex
