module.exports = {
    plugins: {
        'postcss-preset-env': {
            stage: 1, // Use modern CSS features
            features: {
                'nesting-rules': true,
                'custom-properties': true,
                'custom-media-queries': true,
                'color-mix': true
            }
        },
        'postcss-nested': {}, // Enables Sass-like nesting
        'postcss-import': {},
        'autoprefixer': {} // Add vendor prefixes automatically
    }
};
