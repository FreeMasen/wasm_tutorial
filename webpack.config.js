const path = require('path');
module.exports = function(env) {
    mode = env == 'prod' || process.env.production ? 'production' : 'development';
    devtool = mode == 'development' ? 'source-map' : null;
    return {
        entry: {
            'app': './ts/app.ts'
        },
        output: {
            path: path.resolve(__dirname, 'dist', 'js'),
            filename: '[name].js',
        },
        resolve: {
            extensions: ['.ts', '.js', '.wasm']
        },
        module: {
            rules: [
                {
                    test: /\.ts$/,
                    use: 'awesome-typescript-loader'
                }
            ]
        },
        devtool,
        mode,
    }
}