import path from "node:path";
import { checkCloc } from "../../index";
import { expect, test } from "vitest";

test("check_cloc", async () => {
	const res = await checkCloc({
		cwd: path.resolve(__dirname, "fixtures"),
		ignore: [],
	});

	expect(res.length).toBe(2);

	expect(res).toMatchSnapshot();
});
