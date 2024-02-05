import { defineConfig } from 'turbowatch';

export default defineConfig({
    project: __dirname,
    triggers: [
        {
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
            onChange: async ({ spawn }) => {
                await spawn`next build`;
            },
        },
        {
            expression: [
                'allof',
                ['not', ['dirname', '.cargo']],
                ['not', ['dirname', 'target']],
                ['anyof', ['match', '*.rs', 'basename'], ['match', '*.toml', 'basename']],
            ],
            name: 'backend',
            onChange: async ({ spawn }) => {
                await spawn`cargo shuttle run --working-directory backend`;
            },
        },
    ],
});
