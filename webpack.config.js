const HtmlWebpackPlugin = require('html-webpack-plugin');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');

module.exports = {
    plugins: [
        new WasmPackPlugin({
            crateDirectory: '.',
        }),
        new HtmlWebpackPlugin({
            title: 'This is Snek!',
        }),
    ],
    mode: 'development',
};
