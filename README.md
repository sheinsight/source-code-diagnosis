# source-code-diagnosis

## getUsageOfDangerStrings

```ts
import { getUsageOfDangerStrings } from "@shined/source-code-diagnosis";

let response = getUsageOfDangerStrings(
  ["bootcss.com", "bootcdn.com", "polyfill.com", "polyfill.io"],
  {
    cwd: "/Users/Pikachu/project",
    concurrency: 1,
  }
);
```
