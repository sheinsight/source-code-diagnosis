import { expect, test } from 'vitest'
import { fileURLToPath } from "node:url";
import path, { dirname } from "node:path";
import { checkDangerStrings } from '../../index.js'

const __filename = fileURLToPath(import.meta.url);

test('should detect danger strings in specified directory', () => {
  const cwd = dirname(__filename);
  const response = checkDangerStrings(["bootcss.com", "bootcdn.com", "polyfill.com", "polyfill.io"],{cwd:path.resolve(cwd,'features')})
  expect(response.length).toBe(7)
})
 

