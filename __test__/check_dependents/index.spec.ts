import { expect, test } from 'vitest'
import { fileURLToPath } from "node:url";
import path, { dirname } from "node:path";
import { checkDependents, Cycle, DependencyNode} from '../../index.js'


function normalizePaths(cwd:string,node:  Array<Array<Cycle>>):  Array<Array<Cycle>> {
  return node.map(
    item => 
      item.map(
        x => ({
          ...x,
          source:x.source.replace(cwd,""),
          target:x.target.replace(cwd,""),
        })
      )
  );
}

const __filename = fileURLToPath(import.meta.url);
 
test('Get which files depend on the specified file', () => {
  const cwd = path.resolve(dirname(__filename),"features","normal");

  const response = checkDependents(path.join(cwd,"utils.js"),{
    cwd,
  })

  const normalizedPaths = normalizePaths(cwd,response);

  expect(normalizedPaths).toMatchSnapshot()

})



test('Get which files depend on the specified file with alias', () => {
  const cwd = path.resolve(dirname(__filename),"features","alias");  
  const response = checkDependents(path.join(cwd,"utils.js"),{
    alias:{
      "@":[cwd]
    },
    cwd,
  })
  const normalizedPaths = normalizePaths(cwd,response);

  expect(normalizedPaths).toMatchSnapshot()
})
 