import type { Cycle } from "../../index.js";

export function normalizePaths(cwd: string, node: Array<Array<Cycle>>): string {
	return node
		.map((item) =>
			item
				.map((x) => ({
					...x,
					source: x.source.replace(cwd, "").replace(/^(\\|\/)/, ""),
					target: x.target.replace(cwd, "").replace(/^(\\|\/)/, ""),
				}))
				.map((x) => `${x.source} -> ${x.target}`)
				.join(" , "),
		)
		.sort()
		.join(" :: ");
}
