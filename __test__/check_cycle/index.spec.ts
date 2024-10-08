import { expect, test } from "vitest";
import { fileURLToPath } from "node:url";
import path, { dirname } from "node:path";
import { checkCycle, type GroupGraphics } from "../../index.js";

const __filename = fileURLToPath(import.meta.url);

function normalizePaths(graphics: GroupGraphics): string {
	const dictionaries = graphics.dictionaries;

	const graph = graphics.graph
		.map((item) =>
			item
				.map((x) => `${dictionaries[x.source]} -> ${dictionaries[x.target]}`)
				.sort()
				.join(" , "),
		)
		.sort()
		.join(" :: ");

	return graph;
}

test("Detect circular dependencies in the specified directory.", () => {
	const cwd = path.resolve(dirname(__filename), "features", "cycle");
	const response = checkCycle({
		cwd,
	});

	const normalizedPaths = normalizePaths(response);

	expect(normalizedPaths).toMatchSnapshot();
});
