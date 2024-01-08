import {Bundler, setupWasmPanicHook} from './index.js'

setupWasmPanicHook()

const a = new Bundler({
  input: [ {
    import: "./test.js"
  }],
  plugins: [],
  cwd: "./"
})

a.generate({}).then(res => {
  console.log(res)
})
