import { expect, test } from "vitest";
import { checkPhantomDependencies } from "../../index";
import { fileURLToPath } from "node:url";
import path from "node:path";
import { dirname } from "node:path";

const __filename = fileURLToPath(import.meta.url);

test("No phantom dependencies", async () => {
	const cwd = path.resolve(dirname(__filename), "features", "demo1");
	const res = await checkPhantomDependencies(["react"], { cwd });
	expect(res.graph.length).toBe(0);
});

test("The existence of phantom dependencies", async () => {
	const cwd = path.resolve(dirname(__filename), "features", "demo2");
	const res = await checkPhantomDependencies(["react"], { cwd });

	expect(res.graph.length).toBe(1);
});
