#[cfg(test)]
mod tests {
    use ruststreams::Stream;
    use crate::{VarWrite, VarRead};


    #[test]
    fn test() {
        let mut stream = Stream::new();
        stream.write_var_int(42);
        stream.write_var_int(69);
        assert_eq!(stream.read_var_int(), 42);
        assert_eq!(stream.read_var_int(), 69);
    }
}

use std::io::{Read, Write};

macro_rules! write_integer {
    ($self:ident, $var:ident) => {
        loop {
            let mut temp = ($var & 0b01111111) as u8;
            $var >>= 7;
            if $var != 0 {
                temp |= 0b10000000;
            }
            $self.write(&[temp as u8]).expect("Write error");

            if $var == 0 { break; }
        }
    }
}

pub trait VarWrite
{
    fn write_var_int(&mut self, var_int: i32);
    fn write_var_long(&mut self, var_int: i64);
}

pub trait VarRead
{
    fn read_var_int(&mut self) -> i32;
    fn read_var_long(&mut self) -> i64;
}

impl<T> VarWrite for T
    where T: Write
{
    fn write_var_int(&mut self, var_int: i32) {
        let mut u_val = var_int as u32;
        write_integer!(self, u_val);
    }

    fn write_var_long(&mut self, var_int: i64) {
        let mut u_val = var_int as u64;
        write_integer!(self, u_val);
    }
}

impl<T> VarRead for T
    where T: Read
{
    fn read_var_int(&mut self) -> i32 {
        let mut num_read: i32 = 0;
        let mut result: i32 = 0;
        let read = &mut [0];

        loop {
            self.read(read).expect("Unexpected end of stream");
            let value: i32 = read[0] as i32 & 0b01111111;
            result |= value << (7 * num_read);

            num_read += 1;
            if num_read > 5 {
                panic!()
            }
            if (read[0] as i32 & 0b10000000) == 0 { break; }
        }
        result
    }

    fn read_var_long(&mut self) -> i64 {
        let mut num_read: i32 = 0;
        let mut result: i64 = 0;
        let read = &mut [0];

        loop {
            self.read(read).expect("Unexpected end of stream");
            let value: i64 = read[0] as i64 & 0b01111111;
            result |= value << (7 * num_read);

            num_read += 1;
            if num_read > 10 {
                panic!()
            }
            if (read[0] as i32 & 0b10000000) == 0 { break; }
        }
        result
    }
}
