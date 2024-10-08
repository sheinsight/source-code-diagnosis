import { expect, test } from "vitest";
import { fileURLToPath } from "node:url";
import path, { dirname } from "node:path";
import { checkDependents, type Graphics } from "../../index.js";

const __filename = fileURLToPath(import.meta.url);

function normalizePaths(graphics: Graphics): string {
	const dictionaries = graphics.dictionaries;

	const graph = graphics.graph
		.map((x) => `${dictionaries[x.source]} -> ${dictionaries[x.target]}`)
		.sort()
		.join(" :: ");

	return graph;
}

test("Get which files depend on the specified file", () => {
	const cwd = path.resolve(dirname(__filename), "features", "normal");

	const response = checkDependents("utils.js", {
		cwd,
	});

	const normalizedPaths = normalizePaths(response);

	expect(normalizedPaths).toMatchSnapshot();
});

test("Get which files depend on the specified file with alias", () => {
	const cwd = path.resolve(dirname(__filename), "features", "alias");
	const response = checkDependents("utils.js", {
		alias: {
			"@": [cwd],
		},
		cwd,
	});
	const normalizedPaths = normalizePaths(response);

	expect(normalizedPaths).toMatchSnapshot();
});
