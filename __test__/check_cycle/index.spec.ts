import { expect, test } from "vitest";
import { fileURLToPath } from "node:url";
import path, { dirname } from "node:path";
import { checkCycle } from "../../index.js";
import { normalizePaths } from "../utils/normalize-paths.js";

const __filename = fileURLToPath(import.meta.url);

test("Detect circular dependencies in the specified directory.", () => {
	const cwd = path.resolve(dirname(__filename), "features", "cycle");
	const response = checkCycle({
		cwd,
	});

	const normalizedPaths = normalizePaths(cwd, response);

	expect(normalizedPaths).toMatchSnapshot();
});
