import test from "ava";
import { demo } from "../../index.js";
import { fileURLToPath } from "node:url";
import { dirname, posix } from "node:path";
import { performance } from "node:perf_hooks";

// 获取当前文件的路径
const __filename = fileURLToPath(import.meta.url);

test("getModuleMemberUsage", (t) => {
	const start = performance.now();
	const cwd = dirname(__filename);
	demo(["antd"], {
		cwd,
		concurrency:2
	});
	const end = performance.now();
	console.log("Execution time: " + (end - start) + "ms");
	t.is(1, 1);
});
