const path = require('path')
const nodeExternals = require('webpack-node-externals')

module.exports = {
  entry: {
    'api/build': './src/api/build.ts',
    'api/dev': './src/api/dev.ts',
    'api/init': './src/api/init.ts',
    'api/tauricon': './src/api/tauricon.ts',
    'helpers/tauri-config': './src/helpers/tauri-config.ts',
    'api/info': './src/api/info.ts'
  },
  mode: process.env.NODE_ENV || 'development',
  devtool: 'source-map',
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: 'ts-loader',
        exclude: /node_modules/
      },
      {
        test: /tauri\.js$/i,
        loader: 'raw-loader'
      }
    ]
  },
  node: false,
  resolve: {
    extensions: ['.ts', '.js']
  },
  output: {
    library: 'tauri',
    libraryTarget: 'umd',
    filename: '[name].js',
    path: path.resolve(__dirname, 'dist')
  },
  externals: [
    nodeExternals()
    // {
    //   'fs-extra': 'Deno'
    // }
  ],
  target: 'node'
}
