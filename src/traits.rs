pub trait Runnable {
    fn run(&self) -> Result<(), std::io::Error>;
}