import { expect, test } from "vitest";
import { fileURLToPath } from "node:url";
import path, { dirname } from "node:path";
import { checkBrowserSupportedWithSourceCode, checkBrowserSupported } from "../../index.js";

const __dirname = dirname(fileURLToPath(import.meta.url));

test("There are 4 syntaxes under normal that are incompatible under chrome 40", async () => {
	const cwd = path.resolve(__dirname, "fixtures", "normal");

	const response = await checkBrowserSupported({ chrome: "40" }, { cwd });

	expect(response.length).toBe(4);
});

test("There are 3 syntaxes under normal that are incompatible under chrome 45", async () => {
	const cwd = path.resolve(__dirname, "fixtures", "normal");

	const response = await checkBrowserSupported({ chrome: "45" }, { cwd });

	expect(response.length).toBe(3);
});

test("Multiple occurrences of the same syntax results in multiple calculations.", async () => {
	const response1 = await checkBrowserSupportedWithSourceCode({ chrome: "70" }, "const b = cc ?? 3;", "test.ts");
	expect(response1.length).toBe(1);
	expect(response1).toMatchSnapshot();

	const response2 = await checkBrowserSupportedWithSourceCode(
		{ chrome: "70" },
		"const b = cc ?? 3; const c = cc ?? 3;",
		"test.ts",
	);
	expect(response2.length).toBe(2);
	expect(response2).toMatchSnapshot();
});
