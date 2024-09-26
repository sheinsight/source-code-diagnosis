import { expect, test } from 'vitest'
import { fileURLToPath } from "node:url";
import path, { dirname } from "node:path";
import { checkDependents, DependencyNode} from '../../index.js'


function normalizePaths(node: DependencyNode): DependencyNode {
  const basePath = dirname(__filename);
  let name = path.relative(basePath, node.name).replace(/\\/g, '/');
  let children = node.children.map(normalizePaths).sort((a, b) => a.name.localeCompare(b.name));
  return {
    name,
    children,
    astNode: node.astNode
  };
}

const __filename = fileURLToPath(import.meta.url);
 
test('Get which files depend on the specified file', () => {
  const cwd = path.resolve(dirname(__filename),"features","normal");

  const response = checkDependents(path.join(cwd,"utils.js"),{
    cwd,
  })

  const normalizedPaths = normalizePaths(response);

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

  const normalizedPaths = normalizePaths(response);

  console.log(JSON.stringify(normalizedPaths, null, 2));
  

  expect(normalizedPaths).toMatchSnapshot()
})
 