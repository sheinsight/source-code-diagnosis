import { expect, test } from 'vitest'
import { fileURLToPath } from "node:url";
import path, { dirname } from "node:path";
import { checkModuleMemberUsage } from '../../index.js'

const __filename = fileURLToPath(import.meta.url);

test('should return empty array when module is only imported but not used', () => {
  const cwd = dirname(__filename);
  const response = checkModuleMemberUsage(["antd"],{cwd:path.resolve(cwd,'only-imported')})
  expect(response.length).toBe(0)
})

test('should return 1 result when module member used in statement', () => {
  const cwd = dirname(__filename);
  const response = checkModuleMemberUsage(["antd"],{cwd:path.resolve(cwd,'features/use-in-statement')})
  expect(response.length).toBe(1)
})


test('should return 1 result when module member used in tsx', () => {
  const cwd = dirname(__filename);
  const response = checkModuleMemberUsage(["antd"],{cwd:path.resolve(cwd,'features/use-in-tsx')})
  expect(response.length).toBe(1)
})

test('should return 1 result when module member used in tsx with self closed', () => {
  const cwd = dirname(__filename);
  const response = checkModuleMemberUsage(["antd"],{cwd:path.resolve(cwd,'features/use-in-tsx-with-self-closed')})
  expect(response.length).toBe(1)
})

test('should return 1 result when module member used in tsx with static member', () => {
  const cwd = dirname(__filename);
  const response = checkModuleMemberUsage(["antd"],{cwd:path.resolve(cwd,'features/use-in-tsx-with-static-member')})
  expect(response.length).toBe(1)
})

test('should return 1 result when module member used in tsx with static member self closed', () => {
  const cwd = dirname(__filename);
  const response = checkModuleMemberUsage(["antd"],{cwd:path.resolve(cwd,'features/use-in-tsx-with-static-member-self-closed')})
  expect(response.length).toBe(1)
})

test('should return 1 result when module member used in tsx with namespace import', () => {
  const cwd = dirname(__filename);
  const response = checkModuleMemberUsage(["antd"],{cwd:path.resolve(cwd,'features/use-in-tsx-with-namespace-import')})
  expect(response.length).toBe(1)
})

test('should return 1 result when module member used in tsx with multi library usage', () => {
  const cwd = dirname(__filename);
  const response = checkModuleMemberUsage(["antd","lodash"],{cwd:path.resolve(cwd,'features/multi-library-usage')})
  expect(response.length).toBe(2)
})

test('should return 1 result when module member used in tsx with no-impored-but-used', () => {
  const cwd = dirname(__filename);
  const response = checkModuleMemberUsage(["antd"],{cwd:path.resolve(cwd,'features/no-impored-but-used')})
  expect(response.length).toBe(1)
})


test('should return 1 result when module member used in tsx with use-in-call-expression', () => {
  const cwd = dirname(__filename);
  const response = checkModuleMemberUsage(["antd"],{cwd:path.resolve(cwd,'features/use-in-call-expression')})
  expect(response.length).toBe(1)
})

test('should return 1 result when module member used in tsx with use-in-jsx-alias', () => {
  const cwd = dirname(__filename);
  const response = checkModuleMemberUsage(["antd"],{cwd:path.resolve(cwd,'features/use-in-jsx-with-alias')})
  expect(response.length).toBe(3)
})

test('should return 1 result when module member used in tsx with tree-fold', () => {
  const cwd = dirname(__filename);
  const response = checkModuleMemberUsage(["antd"],{cwd:path.resolve(cwd,'features/tree-fold')})
  expect(response.length).toBe(4)
})


