import test from "ava";
import { getModuleMemberUsage } from "../../index.js";
import { fileURLToPath } from "node:url";
import { dirname, posix } from "node:path";

// 获取当前文件的路径
const __filename = fileURLToPath(import.meta.url);

test("getModuleMemberUsage", (t) => {
	const cwd = dirname(__filename);
	const response = getModuleMemberUsage(["antd"], {
		cwd,
	});
	t.is(response.length, 18);
});
