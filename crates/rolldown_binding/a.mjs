import {Bundler} from './rolldown.wasi.mjs'
const a = new Bundler({
  input: [ {
    import: "./test.js"
  }],
  plugins: [],
  cwd: "./"
})
console.log('something')
a.generate({}).then(res => {
  console.log(res)
})
