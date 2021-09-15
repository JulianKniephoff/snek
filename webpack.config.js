const HtmlWebpackPlugin = require('html-webpack-plugin');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');

module.exports = {
    mode: 'none',
    plugins: [
        new WasmPackPlugin({
            crateDirectory: __dirname,
        }),
        new HtmlWebpackPlugin({
            title: 'This is Snek!',
        }),
    ],
    experiments: {
        asyncWebAssembly: true,
    },
};
