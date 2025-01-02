import { expect, test } from "vitest";
import { fileURLToPath } from "node:url";
import path, { dirname } from "node:path";
import { checkModuleMemberUsage } from "../../index.js";

const __filename = fileURLToPath(import.meta.url);

test("should return empty array when module is only imported but not used", async () => {
	const cwd = dirname(__filename);
	const response = await checkModuleMemberUsage(["antd"], { cwd: path.resolve(cwd, "only-imported") });
	expect(response.flatMap((item) => item.items).length).toBe(0);
});

test("should return 1 result when module member used in statement", async () => {
	const cwd = dirname(__filename);
	const response = await checkModuleMemberUsage(["antd"], { cwd: path.resolve(cwd, "fixtures/use-in-statement") });
	expect(response.flatMap((item) => item.items).length).toBe(1);
});

test("should return 1 result when module member used in tsx", async () => {
	const cwd = dirname(__filename);
	const response = await checkModuleMemberUsage(["antd"], { cwd: path.resolve(cwd, "fixtures/use-in-tsx") });
	expect(response.flatMap((item) => item.items).length).toBe(1);
});

test("should return 1 result when module member used in tsx with self closed", async () => {
	const cwd = dirname(__filename);
	const response = await checkModuleMemberUsage(["antd"], {
		cwd: path.resolve(cwd, "fixtures/use-in-tsx-with-self-closed"),
	});
	expect(response.flatMap((item) => item.items).length).toBe(1);
});

test("should return 1 result when module member used in tsx with static member", async () => {
	const cwd = dirname(__filename);
	const response = await checkModuleMemberUsage(["antd"], {
		cwd: path.resolve(cwd, "fixtures/use-in-tsx-with-static-member"),
	});
	expect(response.flatMap((item) => item.items).length).toBe(1);
});

test("should return 1 result when module member used in tsx with static member self closed", async () => {
	const cwd = dirname(__filename);
	const response = await checkModuleMemberUsage(["antd"], {
		cwd: path.resolve(cwd, "fixtures/use-in-tsx-with-static-member-self-closed"),
	});
	expect(response.flatMap((item) => item.items).length).toBe(1);
});

test("should return 1 result when module member used in tsx with namespace import", async () => {
	const cwd = dirname(__filename);
	const response = await checkModuleMemberUsage(["antd"], {
		cwd: path.resolve(cwd, "fixtures/use-in-tsx-with-namespace-import"),
	});
	expect(response.flatMap((item) => item.items).length).toBe(1);
});

test("should return 1 result when module member used in tsx with multi library usage", async () => {
	const cwd = dirname(__filename);
	const response = await checkModuleMemberUsage(["antd", "lodash"], {
		cwd: path.resolve(cwd, "fixtures/multi-library-usage"),
	});
	expect(response.flatMap((item) => item.items).length).toBe(2);
});

test("should return 1 result when module member used in tsx with no-impored-but-used", async () => {
	const cwd = dirname(__filename);
	const response = await checkModuleMemberUsage(["antd"], { cwd: path.resolve(cwd, "fixtures/no-impored-but-used") });
	expect(response.flatMap((item) => item.items).length).toBe(1);
});

test("should return 1 result when module member used in tsx with use-in-call-expression", async () => {
	const cwd = dirname(__filename);
	const response = await checkModuleMemberUsage(["antd"], {
		cwd: path.resolve(cwd, "fixtures/use-in-call-expression"),
	});
	expect(response.flatMap((item) => item.items).length).toBe(1);
});

test("should return 1 result when module member used in tsx with use-in-jsx-alias", async () => {
	const cwd = dirname(__filename);
	const response = await checkModuleMemberUsage(["antd"], { cwd: path.resolve(cwd, "fixtures/use-in-jsx-with-alias") });
	expect(response.flatMap((item) => item.items).length).toBe(3);
});

test("should return 1 result when module member used in tsx with tree-fold", async () => {
	const cwd = dirname(__filename);
	const response = await checkModuleMemberUsage(["antd"], { cwd: path.resolve(cwd, "fixtures/tree-fold") });

	expect(response.flatMap((item) => item.items).length).toBe(4);
});

test("check_static_member", async () => {
	const cwd = dirname(__filename);
	const response = await checkModuleMemberUsage(["antd"], {
		cwd: path.resolve(cwd, "fixtures/static_member_expr"),
	});

	expect(response.flatMap((item) => item.items)).toMatchSnapshot();
});
