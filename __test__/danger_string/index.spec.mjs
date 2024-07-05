import test from "ava";
import { getDangerStringsUsage } from "../../index.js";
import { fileURLToPath } from "node:url";
import { dirname } from "node:path";

// 获取当前文件的路径
const __filename = fileURLToPath(import.meta.url);

test("getUsageOfDangerStrings", (t) => {
	const response = getDangerStringsUsage(["bootcss.com", "bootcdn.com", "polyfill.com", "polyfill.io"], {
		cwd: dirname(__filename),
	});
	t.snapshot(response.sort((x) => x.filePath));
});
