import { defineConfig } from 'turbowatch';
import { ChildProcess, exec, spawn } from 'child_process';

// Function to execute a shell command and return a promise
function executeCommand(command: string | string[], abortSignal?: AbortSignal): Promise<void> {
    return new Promise((resolve, reject) => {
        let process: ChildProcess;

        if (typeof command === 'string') {
            // Execute a simple command
            process = exec(command, {cwd: __dirname}, (error, stdout, stderr) => {
                if (error) {
                    console.error(`exec error: ${error}`);
                    return reject(error);
                }
                console.log(stdout);
                console.error(stderr);
                resolve();
            });
        } else {
            // Execute a command with parameters (like 'cargo shuttle run')
            process = spawn(command[0], command.slice(1), {
                cwd: __dirname,
                stdio: 'inherit',
            });

            process.on('close', (code) => {
                if (code !== 0) {
                    reject(new Error(`${command.join(' ')} exited with code ${code}`));
                } else {
                    resolve();
                }
            });
        }

        if (abortSignal) {
            abortSignal.addEventListener('abort', () => {
                process.kill(); // Kill the process if the watcher is aborted
            });
        }
    });
}

let shuttleRuntimeAvailable = false;

// Function to check if the Shuttle runtime is installed
function checkShuttleRuntime(): Promise<void> {
    return new Promise((resolve, reject) => {
        exec('cargo shuttle --version', (error, stdout, _stderr) => {
            if (error) {
                console.error(`Shuttle runtime check failed: ${error}`);
                return reject(error);
            }
            console.log(`Version: ${stdout}`);
            shuttleRuntimeAvailable = true;
            resolve();
        });
    });
}

// Check for the Shuttle runtime once at the start
(async () => {
    try {
        console.log('Checking for cargo shuttle runtime...');
        await checkShuttleRuntime();
    } catch (error) {
        console.error('Failed to check cargo shuttle runtime:', error);
        shuttleRuntimeAvailable = false;
    }
})();

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
            onChange: async () => {
                // Build the Next.js project
                await executeCommand('next build');
            },
        },
        {
            // Watch for changes in backend files, excluding certain directories
            expression: ['allof', ['dirname', 'backend'], ['anyof', ['match', '*.rs', 'basename'], ['match', '*.toml', 'basename']]],
            name: 'backend',
            onChange: async ({abortSignal}) => {
                if (shuttleRuntimeAvailable) {
                    await executeCommand(['cargo', 'shuttle', 'run'], abortSignal);
                } else {
                    console.error('Shuttle runtime not available, skipping cargo shuttle run');
                }
            },
        },
    ],
});
