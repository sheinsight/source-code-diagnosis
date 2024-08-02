import ava_test from "ava";
import { checkBrowserSupportedByFilePath } from "../../index.js";
import { performance } from "node:perf_hooks";

export function test(name, filename) {
	ava_test(`should to has ${name}`, (t) => {
		const start = performance.now();

		const res = checkBrowserSupportedByFilePath("", filename.replace(/\.spec\.mjs$/, ".js"));

		const end = performance.now();

		t.truthy(res.some((item) => item.compat.name === name));
	});
}
