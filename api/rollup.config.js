// rollup.config.js
import { terser } from 'rollup-plugin-terser'
import resolve from '@rollup/plugin-node-resolve'
import commonjs from '@rollup/plugin-commonjs'
import sucrase from '@rollup/plugin-sucrase'
import babel, { getBabelOutputPlugin } from '@rollup/plugin-babel'
import typescript from '@rollup/plugin-typescript'
import pkg from './package.json'

export default [
  {
    input: {
      fs: './src/fs.ts',
      path: './src/path.ts',
      dialog: './src/dialog.ts',
      event: './src/event.ts',
      http: './src/http.ts',
      index: './src/index.ts',
      shell: './src/shell.ts',
      tauri: './src/tauri.ts',
      window: './src/window.ts',
      cli: './src/cli.ts',
      notification: './src/notification.ts'
    },
    treeshake: true,
    perf: true,
    output: [
      {
        dir: 'dist/',
        entryFileNames: '[name].js',
        format: 'cjs',
        exports: 'named',
        globals: {}
      },
      {
        dir: 'dist/',
        entryFileNames: '[name].mjs',
        format: 'esm',
        exports: 'named',
        globals: {}
      }
    ],
    plugins: [
      commonjs({}),
      resolve({
        // pass custom options to the resolve plugin
        customResolveOptions: {
          moduleDirectory: 'node_modules'
        }
      }),
      typescript({
        tsconfig: './tsconfig.json'
      }),
      babel({
        configFile: false,
        presets: [['@babel/preset-env'], ['@babel/preset-typescript']]
      }),
      terser()
    ],
    external: [
      ...Object.keys(pkg.dependencies || {}),
      ...Object.keys(pkg.peerDependencies || {})
    ]
  },
  {
    input: {
      bundle: './src/bundle.ts'
    },
    output: [
      {
        name: '__TAURI__',
        dir: 'dist/', // if it needs to run in the browser
        entryFileNames: 'tauri.bundle.umd.js',
        format: 'umd',
        plugins: [
          getBabelOutputPlugin({
            presets: [['@babel/preset-env', { modules: 'umd' }]],
            allowAllFormats: true
          }),
          terser()
        ],
        globals: {}
      }
    ],
    plugins: [
      sucrase({
        exclude: ['node_modules'],
        transforms: ['typescript']
      }),
      resolve({
        // pass custom options to the resolve plugin
        customResolveOptions: {
          moduleDirectory: 'node_modules'
        }
      })
    ],
    external: [
      ...Object.keys(pkg.dependencies || {}),
      ...Object.keys(pkg.peerDependencies || {})
    ]
  }
]
