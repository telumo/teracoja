const path = require('path')
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin')
const CopyWebpackPlugin = require('copy-webpack-plugin')
const MiniCssExtractPlugin = require('mini-css-extract-plugin')

const distPath = path.resolve(__dirname, 'dist')
module.exports = (env, argv) => {
  return {
    devServer: {
      contentBase: distPath,
      compress: argv.mode === 'production',
      port: 8000,
    },
    entry: './bootstrap.js',
    output: {
      path: distPath,
      filename: 'teracoja.js',
      webassemblyModuleFilename: 'teracoja.wasm',
    },
    module: {
      rules: [
        {
          test: /\.css$/,
          exclude: /node_modules/,
          use: [
            MiniCssExtractPlugin.loader,
            { loader: 'css-loader', options: { importLoaders: 1 } },
            { loader: 'postcss-loader' },
          ],
        },
      ],
    },
    plugins: [
      new MiniCssExtractPlugin({
        filename: 'styles.css',
      }),
      new CopyWebpackPlugin({
        patterns: [{ from: './static', to: distPath }],
      }),
      new WasmPackPlugin({
        crateDirectory: '.',
        extraArgs: '--no-typescript',
      }),
    ],
    watch: argv.mode !== 'production',
  }
}
