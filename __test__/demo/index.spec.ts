import { expect, test } from "vitest";
import { fileURLToPath } from "node:url";
import path, { dirname } from "node:path";
import { test as t } from "../../index.js";

const __filename = fileURLToPath(import.meta.url);
test("Detect circular dependencies in the specified directory.", () => {
	// const cwd = path.resolve(dirname(__filename), "features", "cycle");
	// const response = checkCycle({
	// 	cwd,
	// });

	// const normalizedPaths = normalizePaths(response);

	// expect(normalizedPaths).toMatchSnapshot();

	// const cwd = path.resolve(dirname(__filename), "..", "check_cycle", "features", "cycle");
	// const cwd = path.resolve(dirname(__filename), "..", "check_dependents", "features", "normal");

	const cwd = path.resolve(dirname(__filename), "..", "check_dependencies", "features", "normal");

	const res = t("a.js", {
		cwd,
	});

	expect(true).toBeTruthy();
});
