#[cfg(test)]
mod tests {
    use ruststreams::Stream;
    use std::io::{Result};
    use crate::{VarWrite, VarRead, get_var_int_size, get_var_long_size};

    #[test]
    fn test() {
        let mut stream = Stream::new();
        stream.write_var_int(42).expect("write");
        stream.write_var_int(69).expect("write");

        assert_eq!(stream.read_var_int().expect("read"), 42);
        assert_eq!(stream.read_var_int().expect("read"), 69);

        assert_eq!(get_var_int_size(10), 1);
        assert_eq!(get_var_int_size(256), 2);

        assert_eq!(get_var_int_size(10), get_var_long_size(10));
        assert_eq!(get_var_int_size(256), get_var_long_size(256));
        assert_eq!(get_var_int_size(70000), get_var_long_size(70000));
    }
}

use std::io::{Read, Write, Error, Result, ErrorKind};

pub fn get_var_int_size(var_int: i32) -> usize
{
    get_var_long_size(var_int as i64)
}

pub fn get_var_long_size(var_long: i64) -> usize
{
    let mut u_val = var_long as u64;
    let mut count = 0;
    loop {
        u_val >>= 7;
        count += 1;
        if u_val == 0 { break; }
    }
    count
}

pub trait VarWrite
{
    fn write_var_int(&mut self, var_int: i32) -> Result<usize>;
    fn write_var_long(&mut self, var_long: i64) -> Result<usize>;
}

pub trait VarRead
{
    fn read_var_int(&mut self) -> Result<i32>;
    fn read_var_long(&mut self) -> Result<i64>;
}

impl<T> VarWrite for T
    where T: Write
{
    fn write_var_int(&mut self, var_int: i32) -> Result<usize> {
        self.write_var_long(var_int as i64)
    }

    fn write_var_long(&mut self, var_long: i64) -> Result<usize> {
        let mut u_val = var_long as u64;
        let mut count = 0;

        loop {
            let mut temp = (u_val & 0b01111111) as u8;
            u_val >>= 7;
            if u_val != 0 {
                temp |= 0b10000000;
            }
            let r = self.write(&[temp as u8]);
            if !r.is_ok() {
                return Err(Error::from(ErrorKind::Other));
            } else {
                count += r.unwrap();
            }

            if u_val == 0 { break; }
        };
        Ok(count)
    }
}

impl<T> VarRead for T
    where T: Read
{
    fn read_var_int(&mut self) -> Result<i32> {
        let mut num_read: usize = 0;
        let mut result: i32 = 0;
        let read = &mut [0];

        loop {
            let r = self.read(read);
            if !r.is_ok() {
                return Err(Error::from(ErrorKind::Other));
            } else {
                num_read += r.unwrap();
            }
            let value: i32 = read[0] as i32 & 0b01111111;
            result |= value << (7 * num_read);

            if num_read > 5 {
                return Err(Error::from(ErrorKind::Other));
            }
            if (read[0] as i32 & 0b10000000) == 0 { break; }
        }
        Ok(result)
    }

    fn read_var_long(&mut self) -> Result<i64> {
        let mut num_read: usize = 0;
        let mut result: i64 = 0;
        let read = &mut [0];

        loop {
            let r = self.read(read);
            if !r.is_ok() {
                return Err(Error::from(ErrorKind::Other));
            } else {
                num_read += r.unwrap();
            }

            let value: i64 = read[0] as i64 & 0b01111111;
            result |= value << (7 * num_read);

            if num_read > 10 {
                return Err(Error::from(ErrorKind::Other));
            }
            if (read[0] as i32 & 0b10000000) == 0 { break; }
        }
        Ok(result)
    }
}
