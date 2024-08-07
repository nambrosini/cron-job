# cron-job [![crates.io](https://github.com/nambrosini/cron-job/actions/workflows/.github/workflows/continuous-rust.yaml/badge.svg)](https://github.com/nambrosini/cron-job/actions) [![crates.io](https://img.shields.io/crates/v/cron-job.svg)](https://crates.io/crates/cron-job) [![docs](https://docs.rs/cron-job/badge.svg)](https://docs.rs/cron-job)

The `cron-job` library lets you create cronjobs. This is basically
an implementation of the `cron` library.

## Getting started

Add this to your project:

```
[dependencies]
cron_job = "0.2.0"
```

## Examples

You can schedule your own functions as jobs.

``` rust
extern crate cron_job;
use cron_job::CronJob;

fn main() {
    // Create CronJob
    let mut cron = CronJob::default();
    // Add the function
    cron.new_job("* * * * * *", run_on_cron);
    // Start job
    cron.start();
}

// The function to be executed.
fn run_on_cron() {
    println!("Executed function");
}
```

Multiple functions with different cron expression can also be added.

``` rust
extern crate cron_job;
use cron_job::CronJob;

fn main() {
    // Create CronJob
    let mut cron = CronJob::default();
    // Add the function to be run every second
    cron.new_job("* * * * * *", run_every_second);
    // Add the function to be run every 5 seconds
    cron.new_job("*/5 * * * * *", run_every_five_seconds);
    // Start jobs
    cron.start();
}

// The function to be executed every second.
fn run_every_second() {
    println!("1 second");
}

// The function to be executed every 5 seconds.
fn run_every_five_seconds() {
    println!("5 seconds");
}
```

### Running jobs implementing Job trait

Since the function used as job cannot have any parameters, the
`Job` trait is available to be implemented to structs. This way
if any parameter needs to be passed to the function, can be
passed as the struct property.

``` rust
extern crate cron_job;
use cron_job::CronJob;

fn main() {
    // Create HelloJob
    let helloJob = HelloJob{ name: "John" };
    // Create CronJob
    let mut cron = CronJob::default();
    // Say hello every second
    cron.new_job("* * * * * *", helloJob);
    // Start jobs
    cron.start();
}

// The job to be executed
struct HelloJob {
    name: String
}

impl Job for HelloJob {
    fn run(&self) {
        println!("Hello, {}!", self.name);
    }
}
```

Functions and job can also be mixed together.

``` rust
extern crate cron_job;
use cron_job::CronJob;

fn main() {
    // Create HelloJob
    let helloJob = HelloJob{ name: "John" };
    // Create CronJob
    let mut cron = CronJob::default();
    // Run function every second
    cron.new_job("* * * * * *", run_every_second);
    // Say hello every second
    cron.new_job("* * * * * *", helloJob);
    // Start jobs
    cron.start();
}
// The function to be executed every second.
fn run_every_second() {
    println!("1 second");
}

// The job to be executed
struct HelloJob {
    name: String
}

// Very important, implement the Job trait and its functions.
impl Job for HelloJob {
    fn run(&self) {
        println!("Hello, {}!", self.name);
    }
}
```
