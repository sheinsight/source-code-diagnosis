import { expect, test } from "vitest";
import { checkOxlint } from "../../index";
import { fileURLToPath } from "node:url";
import path from "node:path";
import { dirname } from "node:path";

const __filename = fileURLToPath(import.meta.url);

test("No phantom dependencies", async () => {
	const cwd = path.resolve(dirname(__filename), "fixtures");
	const res = await checkOxlint(
		JSON.stringify({
			rules: {
				"unicorn/no-instanceof-array": "error",
			},
		}),
		{ cwd },
	);

	console.log(res.at(0)?.labels);
});
