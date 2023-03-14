/// The Job trait, allows structs to be run as cronjobs.
pub trait Job: Sync + Send + 'static {
    fn run(&mut self);
}
