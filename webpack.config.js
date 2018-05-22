const path = require('path');
module.exports = function(env) {
    return {
        entry: {
            'app': './ts/app.ts'
        },
        output: {
            path: path.resolve(__dirname, 'dist', 'js'),
            filename: '[name].js'
        },
        resolve: {
            extensions: ['.ts', '.js']
        },
        module: {
            rules: [
                {
                    test: /\.ts$/,
                    use: 'awesome-typescript-loader'
                }
            ]
        },
        mode: env == 'prod' || process.env.production ? 'production' : 'development'
    }
}