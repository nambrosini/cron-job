/// The Job trait, allows structs to be run as cronjobs.
pub trait Job: Send + 'static {
    fn run(&mut self);
}
