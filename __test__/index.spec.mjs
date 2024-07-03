import test from 'ava'
import { getUsageOfDangerStrings } from '../index.js'
import { fileURLToPath } from 'url';
import { dirname } from 'path';

// 获取当前文件的路径
const __filename = fileURLToPath(import.meta.url);

test('getUsageOfDangerStrings', (t) => {
  let response = getUsageOfDangerStrings(["bootcss.com","bootcdn.com","polyfill.com","polyfill.io"],{
    cwd: dirname(__filename),
  });
  t.is(response.length,7)
})
