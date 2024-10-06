import { expect, test } from 'vitest'
import { fileURLToPath } from "node:url";
import path, { dirname } from "node:path";
import {  checkCycle , Cycle} from '../../index.js'


function normalizePaths(cwd:string,node:  Array<Array<Cycle>>): string {
  return node.map(
    item => 
      item.map(
        x => ({
          ...x,
          source:x.source.replace(cwd,"").replace(/^\\/,"/"),
          target:x.target.replace(cwd,"").replace(/^\\/,"/"),
        })
      ).map(x => `${x.source}${x.target}`).join("-")
  ).join("::")
}

const __filename = fileURLToPath(import.meta.url);

test('Detect circular dependencies in the specified directory.', () => {
  const cwd = path.resolve(dirname(__filename),"features","cycle");  
  const response = checkCycle({
    cwd,
  })

  const normalizedPaths = normalizePaths(cwd,response);

  expect(normalizedPaths).toMatchSnapshot()

})