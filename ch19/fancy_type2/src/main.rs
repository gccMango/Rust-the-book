use std::io::Error;
use std::fmt;

// pub trait Write {
//     fn Write(&mut self, buf: &[u8]) -> Result<usize,Error>;
//     fn flush(&mut self) -> Result<(),Error>;

//     fn write_all(&mut (&mut self, buf: &[u8])) -> Result<(),Error>;
//     fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(),Error>;
// }

type Result<T> = Result<T,std::io::Error>;

pub trait Write {
    fn Write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}

fn main() {
    println!("Hello, world!");
}
