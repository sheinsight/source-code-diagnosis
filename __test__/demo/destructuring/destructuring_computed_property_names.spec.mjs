import { fileURLToPath } from "node:url";
import { test } from "../common.mjs";

const __filename = fileURLToPath(import.meta.url);
test("computed_property_names", __filename);
