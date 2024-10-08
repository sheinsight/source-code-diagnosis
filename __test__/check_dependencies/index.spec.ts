import { expect, test } from "vitest";
import { fileURLToPath } from "node:url";
import path, { dirname } from "node:path";
import { checkDependencies, type Graphics } from "../../index.js";

const __filename = fileURLToPath(import.meta.url);

function normalizePaths(graphics: Graphics): string {
	const dictionaries = graphics.dictionaries;

	const graph = graphics.graph
		.map((x) => `${dictionaries[x.source]} -> ${dictionaries[x.target]}`)
		.sort()
		.join(" :: ");

	return graph;
}

test("The dependency file list of the specified file should be obtained normally.", () => {
	const cwd = path.resolve(dirname(__filename), "features", "normal");
	const response = checkDependencies("c.js", {
		cwd,
	});

	const normalizedPaths = normalizePaths(response);

	expect(normalizedPaths).toMatchSnapshot();
});

test("The dependency file list of the specified file should be obtained normally.", () => {
	const cwd = path.resolve(dirname(__filename), "features", "alias");
	const response = checkDependencies("c.js", {
		alias: {
			"@": [cwd],
		},
		cwd,
	});

	const normalizedPaths = normalizePaths(response);

	expect(normalizedPaths).toMatchSnapshot();
});
