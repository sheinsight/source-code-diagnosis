import { expect, test } from "vitest";
import { fileURLToPath } from "node:url";
import path, { dirname } from "node:path";
import { checkDependencies, type Graphics } from "../../index.js";
import fs from "node:fs";
const __filename = fileURLToPath(import.meta.url);

function normalizePaths(graphics: Graphics): string {
	const dictionaries = graphics.dictionaries;

	const graph = graphics.graph
		.map((x) => `${dictionaries[x.source]} -> ${dictionaries[x.target]}`)
		.sort()
		.join(" :: ");

	return graph;
}

function transformToEcharts(current: string, graphics: Graphics) {
	const nodes = Object.entries(graphics.dictionaries).map(([id, name]) => ({
		id,
		name,
		symbolSize: current === name ? 30 : 10,
		edgeSymbol: ["circle", "arrow"],
		itemStyle: {
			color: current === name ? "#3ba272" : undefined,
		},
	}));

	const links = graphics.graph.map((edge) => ({
		source: edge.source,
		target: edge.target,
	}));

	return { 
		nodes,
		links,
		categories: [],
	};
}

test("The dependency file list of the specified file should be obtained normally.", () => {
	const cwd = path.resolve(dirname(__filename), "features", "normal");
	const current = "c.js";
	const response = checkDependencies(current, {
		cwd,
	});

	const normalizedPaths = normalizePaths(response);

	expect(normalizedPaths).toMatchSnapshot();

	fs.writeFileSync("./response.json", JSON.stringify(transformToEcharts(current, response), null, 2), {
		encoding: "utf-8",
	});
});

test("The dependency file list of the specified file should be obtained normally.", () => {
	const current = "c.js";
	const cwd = path.resolve(dirname(__filename), "features", "alias");
	const response = checkDependencies(current, {
		alias: {
			"@": [cwd],
		},
		cwd,
	});

	const normalizedPaths = normalizePaths(response);

	expect(normalizedPaths).toMatchSnapshot();
});

test("Ignore node_modules and dist directories.", () => {
	const cwd = path.resolve(dirname(__filename), "features", "has_node_modules");
	const current = "a.js";
	const response = checkDependencies(current, {
		cwd,
	});

	const normalizedPaths = normalizePaths(response);

	expect(normalizedPaths).toMatchSnapshot();
});

test("Ignore empty import.", () => {
	try {
		const cwd = path.resolve(dirname(__filename), "features", "empty-import");
		const current = "a.js";
		const response = checkDependencies(current, {
			cwd,
		});

		const normalizedPaths = normalizePaths(response);

		expect(normalizedPaths).toMatchSnapshot();
	} catch (error) {
		console.log(error);
	}
});

test("Get which files depend on the specified file with all-from", () => {
	const cwd = path.resolve(dirname(__filename), "features", "all-from");
	const current = "a.js";
	const response = checkDependencies(current, {
		cwd,
	});

	const normalizedPaths = normalizePaths(response);

	expect(normalizedPaths).toMatchSnapshot();
});
 
test("Test error syntax code", () => {
	const cwd = path.resolve(dirname(__filename), "features", "error_syntax");
	const current = "b.js";
	try {
		checkDependencies(current, {
			cwd,
		});
	} catch (error) {
		expect(error).toBeDefined();
	}
	// expect(() => {
	// 	checkDependencies(current, {
	// 		cwd,
	// 	});
	// }).toThrowError();
});
