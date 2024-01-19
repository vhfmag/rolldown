import { rolldown } from './dist/index.mjs'

const output = await rolldown({
  input: process.cwd() + '/packages/node/test.js',
})

await output.generate({
  format: 'esm',
  file: 'test.mjs',
})
