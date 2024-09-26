import { expect, test, } from 'vitest'
import { fileURLToPath } from "node:url";
import path, { dirname } from "node:path";
import { checkDependencies, DependencyNode} from '../../index.js'



const __filename = fileURLToPath(import.meta.url);

function normalizePaths(node: DependencyNode): DependencyNode {
  const basePath = dirname(__filename);
  return {
    name: path.relative(basePath, node.name).replace(/\\/g, '/'),
    children: node.children.map(normalizePaths),
    astNode: node.astNode
  };
}


test('The dependency file list of the specified file should be obtained normally.', () => {
  const cwd = path.resolve(dirname(__filename),"features","normal");
  const response = checkDependencies(path.join(cwd,"c.js"),{
    cwd,
  })

  const normalizedPaths = normalizePaths(response);

  expect(normalizedPaths).toMatchSnapshot()
 
})

test('The dependency file list of the specified file should be obtained normally.', () => {
  const cwd = path.resolve(dirname(__filename),"features","alias");
  const response = checkDependencies(path.join(cwd,"c.js"),{
    alias:{
      "@":[cwd]
    },
    cwd,
  })

  const normalizedPaths = normalizePaths(response);

  expect(normalizedPaths).toMatchSnapshot()
 


})