use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::Duration,
};

use cron_job::{CronJob, Job};

struct NewJob {
    sender: Sender<i32>,
}

impl Job for NewJob {
    fn run(&mut self) {
        self.sender.send(1).unwrap();
    }
}

#[test]
fn test() {
    let total_run = 5;
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    thread::spawn(move || {
        let mut cron = CronJob::default();
        let new_job = NewJob { sender: tx };
        cron.new_job("* * * * * *", new_job);
        cron.start().unwrap();
    });

    thread::sleep(Duration::from_millis(5100));

    let mut count = 0;
    while rx.try_recv().is_ok() {
        count += 1;
    }
    assert_eq!(count, total_run);
}
#[test]
fn test_with_offset() {
    let total_run = 5;
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    thread::spawn(move || {
        let mut cron = CronJob::new(None, 2000);
        let new_job = NewJob { sender: tx };
        cron.new_job("* * * * * *", new_job);
        cron.start().unwrap();
    });

    thread::sleep(Duration::from_millis(10100));

    let mut count = 0;
    while rx.try_recv().is_ok() {
        count += 1;
    }
    assert_eq!(count, total_run);
}
