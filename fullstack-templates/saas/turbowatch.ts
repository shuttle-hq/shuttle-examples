import { defineConfig } from 'turbowatch';
import { ChildProcess, exec, spawn } from 'child_process';
import path from 'path';
import os from 'os';

// Function to execute a shell command and return a promise
function executeCommand(command: string | string[], abortSignal?: AbortSignal): Promise<void> {
    return new Promise((resolve, reject) => {
        let process: ChildProcess;

        // Destructure the command into the command and its arguments
        const [cmd, ...args] = typeof command === 'string' ? command.split(' ') : command;

        // Start the process
        process = spawn(cmd, args, {
            cwd: __dirname,
            stdio: 'inherit',
        });

        // When the process exits, resolve or reject the promise
        process.on('close', (code) => {
            if (code !== 0) {
                reject(new Error(`${[cmd, ...args].join(' ')} exited with code ${code}`));
            } else {
                resolve();
            }
        });

        // If an abort signal is provided, kill the process if the watcher is aborted
        if (abortSignal) {
            abortSignal.addEventListener('abort', () => {
                process.kill();
            });
        }
    });
}

let shuttleCliAvailable = false;

// Function to check if cargo shuttle is installed
function checkShuttleCli(): Promise<void> {
    return new Promise((resolve, reject) => {
        exec('cargo shuttle --version', (error, stdout, _stderr) => {
            if (error) {
                console.error(`Shuttle check failed: ${error}`);
                return reject(error);
            }
            console.log(`Version: ${stdout}`);
            shuttleCliAvailable = true; // Set the shuttle availability flag to true
            resolve();
        });
    });
}

// Check for cargo shuttle once at the start
(async () => {
    try {
        console.log('Checking for cargo shuttle...');
        await checkShuttleCli();
    } catch (error) {
        console.error('Failed to check cargo shuttle:', error);
        shuttleCliAvailable = false;
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
            // If {interruptible: true}, then AbortSignal will abort the current onChange routine
            interruptible: true,
            // Routine that is executed when file changes are detected
            onChange: async ({abortSignal}) => {
                // Build the Next.js project
                const nextCmd = os.platform() === 'win32' ? 'next.cmd' : 'next';
                const nextPath = path.join(__dirname, 'node_modules', '.bin', nextCmd);
                await executeCommand(`${nextPath} build`, abortSignal);
            },
            // Retry a task if it fails. Otherwise, watch program will throw an error if trigger fails.
            retry: {
                retries: 0, // Setting retries to 0 to ensure that console logs, especially errors, are not overwritten by automatic retries
            },
        },
        {
            // Watch for changes in backend files, excluding certain directories
            expression: ['allof', ['dirname', 'backend'], ['anyof', ['match', '*.rs', 'basename'], ['match', '*.toml', 'basename']]],
            name: 'backend',
            // If {interruptible: true}, then AbortSignal will abort the current onChange routine
            interruptible: true,
            // Routine that is executed when file changes are detected
            onChange: async ({abortSignal}) => {
                if (shuttleCliAvailable) {
                    await executeCommand(['cargo', 'shuttle', 'run'], abortSignal);
                } else {
                    console.error('Shuttle not available, skipping cargo shuttle run');
                }
            },
            // Retry a task if it fails. Otherwise, watch program will throw an error if trigger fails.
            retry: {
                retries: 0, // Setting retries to 0 to ensure that console logs, especially errors, are not overwritten by automatic retries
            },
        },
    ],
});
