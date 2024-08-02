import { fileURLToPath } from "node:url";
import { test } from "../common.mjs";

const __filename = fileURLToPath(import.meta.url);

test("spread_in_object_literals", __filename);
