# `shuttle-cron`

A service that prints out a log message at specified cron-style intervals.

This example uses the `apalis` framework to be able to carry out cronjobs. A struct is defined that has some given data in it, and is stored in the Apalis monitor. We also implement a struct that holds a `chrono::DateTime<chrono::Utc>`, which `apalis` uses internally for cron job streaming.

When the cron job is called, the data is grabbed from the `JobContext` and we then execute the job.

The actual function to be ran itself is stored in `job_fn()` in the main function, as a function pointer.

# Usage
Run `shuttle run` to spin up the service locally.

You can change the behavior of the cronjob by editing the `execute()` function for `CronjobData`.

Note that the run method doesn't have to be an implementation method for `CronjobData` - you can write your own!
