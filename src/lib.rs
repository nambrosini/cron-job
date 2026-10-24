//! # cron-job
//!
//! The `cron-job` library lets you create cronjobs. This is basically
//! an implementation of the `cron` library.
//!
//! ## Getting started
//!
//! ### Running functions
//!
//! You can schedule your own functions as jobs.
//!
//! ```rust,no_run
//! use cron_job::CronJob;
//!
//! fn main() {
//!     // Create CronJob
//!     let mut cron = CronJob::default();
//!     // Add the function
//!     cron.new_job("* * * * * *", run_on_cron).unwrap();
//!     // Start job
//!     cron.start();
//! }
//!
//! // The function to be executed.
//! fn run_on_cron() {
//!     println!("Executed function");
//! }
//! ```
//!
//! Multiple functions with different cron expression can also be added.
//!
//! ```rust,no_run
//! use cron_job::CronJob;
//!
//! fn main() {
//!     // Create CronJob
//!     let mut cron = CronJob::default();
//!     // Add the function to be run every second
//!     cron.new_job("* * * * * *", run_every_second).unwrap();
//!     // Add the function to be run every 5 seconds
//!     cron.new_job("*/5 * * * * *", run_every_five_seconds).unwrap();
//!     // Start jobs
//!     cron.start();
//! }
//!
//! // The function to be executed every second.
//! fn run_every_second() {
//!     println!("1 second");
//! }
//!
//! // The function to be executed every 5 seconds.
//! fn run_every_five_seconds() {
//!     println!("5 seconds");
//! }
//! ```
//!
//! ## Running jobs implementing Job trait
//!
//! Since the function used as job cannot have any parameters, the
//! `Job` trait is available to be implemented to structs. This way
//! if any parameter needs to be passed to the function, can be
//! passed as the struct property.
//!
//! ```rust,no_run
//! use cron_job::{CronJob, Job};
//!
//! fn main() {
//!     // Create HelloJob
//!     let hello_job = HelloJob{ name: "John".to_string() };
//!     // Create CronJob
//!     let mut cron = CronJob::default();
//!     // Say hello every second
//!     cron.new_job("* * * * * *", hello_job).unwrap();
//!     // Start jobs
//!     cron.start();
//! }
//!
//! // The job to be executed
//! struct HelloJob {
//!     name: String
//! }
//!
//! impl Job for HelloJob {
//!     fn run(&mut self) {
//!         println!("Hello, {}!", self.name);
//!     }
//! }
//! ```
//!
//! Functions and job can be mixed together.
//!
//! ```rust,no_run
//! use cron_job::{CronJob, Job};
//!
//! fn main() {
//!     // Create HelloJob
//!     let hello_job = HelloJob{ name: "John".to_string() };
//!     // Create CronJob
//!     let mut cron = CronJob::default();
//!     // Run function every second
//!     cron.new_job("* * * * * *", run_every_second).unwrap();
//!     // Say hello every second
//!     cron.new_job("* * * * * *", hello_job).unwrap();
//!     // Start jobs
//!     cron.start();
//! }
//! // The function to be executed every second.
//! fn run_every_second() {
//!     println!("1 second");
//! }
//!
//! // The job to be executed
//! struct HelloJob {
//!     name: String
//! }
//!
//! // Very important, implement the Job trait and its functions.
//! impl Job for HelloJob {
//!     fn run(&mut self) {
//!         println!("Hello, {}!", self.name);
//!     }
//! }
//! ```

mod cronjob;
mod job;

pub use crate::cronjob::{CronJob, StopHandle};
pub use job::Job;

impl<T: Send + 'static + FnMut()> Job for T {
    fn run(&mut self) {
        self();
    }
}
