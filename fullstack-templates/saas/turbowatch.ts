import { defineConfig } from 'turbowatch';

export default defineConfig({
    project: __dirname,
    triggers: [
        {
            // Watch for changes in frontend files, excluding certain directories
            expression: [
                'allof',
                ['not', ['dirname', '.next']],
                ['not', ['dirname', 'node_modules']],
                ['not', ['dirname', 'dist']],
                ['not', ['dirname', 'public']],
                [
                    'anyof',
                    ['match', '*.ts', 'basename'],
                    ['match', '*.tsx', 'basename'],
                    ['match', '*.js', 'basename'],
                    ['match', '*.jsx', 'basename'],
                    ['match', '*.css', 'basename'],
                ],
            ],
            name: 'frontend',
            onChange: async ({spawn}) => {
                // Build the Next.js project
                await spawn`next build`;
            },
        },
        {
            // Watch for changes in backend files, excluding certain directories
            expression: [
                'allof',
                ['not', ['dirname', '.cargo']],
                ['not', ['dirname', 'target']],
                ['anyof', ['match', '*.rs', 'basename'], ['match', '*.toml', 'basename']],
            ],
            name: 'backend',
            onChange: async ({spawn}) => {
                // Run the Rust project with Cargo Shuttle
                await spawn`cargo shuttle run`;
            },
        },
    ],
});
