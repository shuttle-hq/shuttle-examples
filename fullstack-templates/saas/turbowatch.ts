import { defineConfig } from 'turbowatch';
import { ChildProcess, exec, spawn } from 'child_process';
import path from 'path';
import os from 'os';
import { execSync } from 'node:child_process';

let clearConsoleOnCommand: boolean = true;

/**
 * Executes a shell command asynchronously and returns a promise.
 * @param {string | string[]} command - The command to execute.
 * @param {AbortSignal} [abortSignal] - Optional abort signal to terminate the command.
 * @returns {Promise<void>} - A promise that resolves when the command execution completes.
 */
function executeCommand(command: string | string[], abortSignal?: AbortSignal): Promise<void> {
    return new Promise((resolve, reject) => {
        let process: ChildProcess;

        // Clear the console before executing a new command, if enabled
        if (clearConsoleOnCommand) {
            console.clear();
        }

        const [cmd, ...args] = typeof command === 'string' ? command.split(' ') : command;
        process = spawn(cmd, args, {
            cwd: __dirname,
            stdio: 'inherit',
            shell: true,
        });

        process.on('close', (code) => {
            if (code !== 0) {
                reject(new Error(`Command "${cmd}" exited with code ${code}`));
            } else {
                resolve();
            }
        });

        if (abortSignal) {
            abortSignal.addEventListener('abort', () => {
                process.kill();
            });
        }
    });
}

let shuttleCliAvailable = false;

/**
 * Checks if the Shuttle CLI is available by running `cargo shuttle --version`.
 */
function checkShuttleCli(): Promise<void> {
    return new Promise((resolve, reject) => {
        exec('cargo shuttle --version', (error, stdout, _stderr) => {
            if (error) {
                console.error('Shuttle check failed:', error.message);
                return reject(error);
            }
            console.log('Shuttle CLI Version:', stdout.trim());
            shuttleCliAvailable = true;
            resolve();
        });
    });
}

interface ShuttleRelease {
    tag_name?: string;
}

/**
 * Retrieves the latest Shuttle CLI version from the GitHub API.
 */
async function getLatestShuttleVersion(): Promise<string> {
    try {
        const response = execSync('curl -s https://api.github.com/repos/shuttle-hq/shuttle/releases/latest');
        const data: ShuttleRelease = JSON.parse(response.toString());
        return data.tag_name || '';
    } catch (error) {
        console.error('Error retrieving latest Shuttle version:', error);
        return '';
    }
}

/**
 * Updates the Shuttle CLI if a newer version is available.
 */
async function updateShuttleIfNeeded(): Promise<void> {
    let latestVersion = await getLatestShuttleVersion();
    if (!latestVersion) return;
    latestVersion = latestVersion.startsWith('v') ? latestVersion.substring(1) : latestVersion;

    const localVersionResult = execSync('cargo shuttle --version').toString().trim();
    let localVersion = localVersionResult.split(' ')[1];
    localVersion = localVersion.startsWith('v') ? localVersion.substring(1) : localVersion;

    if (localVersion !== latestVersion) {
        console.log(`Updating Shuttle CLI from ${localVersion} to ${latestVersion}...`);
        if (os.platform() === 'win32') {
            execSync('iwr "https://www.shuttle.rs/install-win" | iex', { stdio: 'inherit' });
        } else {
            execSync('curl -sSfL https://www.shuttle.rs/install | bash', { stdio: 'inherit' });
        }
    } else {
        console.log('Shuttle CLI is up to date');
    }
}

/**
 * Main function that initializes the TurboWatch project.
 */
async function main(): Promise<void> {
    try {
        // Check if Shuttle CLI is available
        await checkShuttleCli();
        console.log('Shuttle CLI is available');

        // Update Shuttle CLI if needed
        await updateShuttleIfNeeded();
        console.log('Starting Next.js development server...');

        // Start the Next.js development server
        const nextCmd = os.platform() === 'win32' ? 'next.cmd' : 'next';
        const nextPath = path.join(__dirname, 'node_modules', '.bin', nextCmd);
        await executeCommand(`${nextPath} dev`);

        console.log('Related triggers are being enabled');
    } catch (error) {
        console.error('Error during initialization:', error instanceof Error ? error.message : 'Unknown error during initialization.');
    }
}

main().then(() => console.log('Initialization complete'));

export default defineConfig({
    project: __dirname,
    triggers: [
        {
            // Watch for changes in backend files, excluding certain directories
            expression: ['allof', ['dirname', 'backend'], ['anyof', ['match', '*.rs', 'basename'], ['match', '*.toml', 'basename']]],
            name: 'backend',
            // If {interruptible: true}, then AbortSignal will abort the current onChange routine
            interruptible: true,
            // Routine that is executed when file changes are detected
            onChange: async ({ abortSignal }) => {
                if (shuttleCliAvailable) {
                    await executeCommand(['cargo', 'shuttle', 'run'], abortSignal);
                } else {
                    console.error('Shuttle not available, skipping cargo shuttle run');
                }
            },
            // Retry settings
            retry: {
                retries: 0, // No retries
            },
        },
    ],
});
