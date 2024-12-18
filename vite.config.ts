import { defineConfig } from "vitest/config";

export default defineConfig({
	test: {
		include: ["__test__/**/*.spec.ts"],
		env: {
			SHINED_LOG: "debug",
		},
		testTimeout: 1_000_000,
	},
});
