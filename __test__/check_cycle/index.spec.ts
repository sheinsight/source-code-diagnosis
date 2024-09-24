import { expect, test } from 'vitest'
import { fileURLToPath } from "node:url";
import path, { dirname } from "node:path";
import { checkDependents,initLogger  , checkDetectCycle ,checkDependencies} from '../../index.js'

const __filename = fileURLToPath(import.meta.url);

test('Detect circular dependencies in the specified directory.', () => {
  const cwd = path.resolve(dirname(__filename),"features","cycle");  
  const response = checkDetectCycle({
    cwd,
  })


  for (const item of response) {
    expect(item.length).toBe(2)
  }
  

  expect(response.length).toBe(1)
})