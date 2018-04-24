const path = require('path');
module.exports = {
    entry: {
        'app': './ts/app.tsx'
    },
    output: {
        path: path.join(__dirname, 'dist', 'js'),
        filename: '[name].js'
    },
    module: {
        rules: [
            {
                test: /\.tsx?$/,
                use: 'ts-loader'
            }
        ]
    },
    mode: 'development'
}