import { WASI as __nodeWASI } from '@tybys/wasm-util'
import { instantiateNapiModule as __emnapiInstantiateNapiModule } from '@emnapi/core'
import { getDefaultContext as __emnapiGetDefaultContext } from '@emnapi/runtime'
import { Volume, createFsFromVolume } from 'memfs-browser'

const fs = createFsFromVolume(Volume.fromJSON({ /* ... */ }))

const __wasi = new __nodeWASI({
  fs,
})


const __emnapiContext = __emnapiGetDefaultContext()

const __sharedMemory = new WebAssembly.Memory({
  initial: 1024,
  maximum: 1024,
  // shared: true,
})

const { instance: __napiInstance, module: __wasiModule, napiModule: __napiModule } = await __emnapiInstantiateNapiModule(fetch('./test.wasm'), {
  context: __emnapiContext,
  // asyncWorkPoolSize: 4,
  wasi: __wasi,
  onCreateWorker() {
  },
  overwriteImports(importObject) {
    debugger
    importObject.env = {
      ...importObject.env,
      ...importObject.napi,
      ...importObject.emnapi,
    }
  },
  beforeInit({ instance }) {
    __napi_rs_initialize_modules(instance)
  }
})

function __napi_rs_initialize_modules(__napiInstance) {
  __napiInstance.exports['__napi_register__Bundler_struct_0']()
  __napiInstance.exports['__napi_register__Bundler_impl_1']()
}

const binding = __napiModule.exports
const { Bundler } = binding
export {
  Bundler
}

