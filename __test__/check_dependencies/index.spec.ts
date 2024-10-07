import { expect, test } from "vitest";
import { fileURLToPath } from "node:url";
import path, { dirname } from "node:path";
import { checkDependencies, Cycle, DependencyNode } from "../../index.js";
import { normalizePaths } from "../utils/normalize-paths.js";

const __filename = fileURLToPath(import.meta.url);

test("The dependency file list of the specified file should be obtained normally.", () => {
	const cwd = path.resolve(dirname(__filename), "features", "normal");
	const response = checkDependencies(path.join(cwd, "c.js"), {
		cwd,
	});

	const normalizedPaths = normalizePaths(cwd, response);

	expect(normalizedPaths).toMatchSnapshot();
});

test("The dependency file list of the specified file should be obtained normally.", () => {
	const cwd = path.resolve(dirname(__filename), "features", "alias");
	const response = checkDependencies(path.join(cwd, "c.js"), {
		alias: {
			"@": [cwd],
		},
		cwd,
	});

	const normalizedPaths = normalizePaths(cwd, response);

	expect(normalizedPaths).toMatchSnapshot();
});
