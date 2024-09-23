import { Bundler } from './binding'
import type { OutputOptions } from './options/output-options'
import { transformToRollupOutput } from './utils/transform-to-rollup-output'
import { createBundler } from './utils/create-bundler'

import type { RolldownOutput } from './types/rolldown-output'
import type { HasProperty, TypeAssert } from './utils/type-assert'
import type { InputOptions } from './options/input-options'
import { AssetSource, transformAssetSource } from './utils/asset-source'

export class RolldownBuild {
  #inputOptions: InputOptions
  #bundler?: Bundler
  #stopWorkers?: () => Promise<void>

  constructor(inputOptions: InputOptions) {
    // TODO: Check if `inputOptions.output` is set. If so, throw an warning that it is ignored.
    this.#inputOptions = inputOptions
  }

  async #getBundler(outputOptions: OutputOptions): Promise<Bundler> {
    if (typeof this.#bundler === 'undefined') {
      const { bundler, stopWorkers } = await createBundler(
        this.#inputOptions,
        outputOptions,
      )
      this.#bundler = bundler
      this.#stopWorkers = stopWorkers
    }
    return this.#bundler
  }

  async generate(outputOptions: OutputOptions = {}): Promise<RolldownOutput> {
    const bundler = await this.#getBundler(outputOptions)
    const output = await bundler.generate()
    return transformToRollupOutput(output)
  }

  async write(outputOptions: OutputOptions = {}): Promise<RolldownOutput> {
    const bundler = await this.#getBundler(outputOptions)
    const output = await bundler.write()
    return transformToRollupOutput(output)
  }

  async experimental_hmr_rebuild(
    changedFiles: string[],
  ): Promise<[string, AssetSource]> {
    const output = await this.#bundler!.hmrRebuild(changedFiles)
    return [
      output.assets[0].fileName,
      transformAssetSource(output.assets[0].source),
    ]
  }

  async close(): Promise<void> {
    const bundler = await this.#getBundler({})
    await this.#stopWorkers?.()
    await bundler.close()
  }
}

function _assert() {
  type _ = TypeAssert<HasProperty<RolldownBuild, 'generate' | 'write'>>
}
