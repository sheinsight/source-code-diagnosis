import test from "ava";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { checkBrowserSupportedByFilePath } from "../../../index.js";
import { performance } from "node:perf_hooks";

const __filename = fileURLToPath(import.meta.url);

test("should to has exponentiation_assignment", (t) => {
	const start = performance.now();
	const cwd = dirname(__filename);

	const res = checkBrowserSupportedByFilePath("", __filename.replace(/\.spec\.mjs$/, ".js"));

	const end = performance.now();

	console.log(`Execution time: ${end - start}ms`);

	t.truthy(res.some((item) => item.compat.name === "exponentiation_assignment"));
});
