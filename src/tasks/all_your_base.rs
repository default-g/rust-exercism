use std::{u32, vec};

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}


pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {

    if to_base <= 1 {
        return Err(Error::InvalidOutputBase);
    }

   for x in number {
        if *x >= from_base {
            return Err(Error::InvalidDigit(*x));
        }
   }

   let mut to_decimal = 0;
   for (i, x) in number.iter().rev().enumerate() {
    to_decimal += x * from_base.pow(i as u32);
   }

   if to_base == 10 {
       return Ok(
        to_decimal.to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect()
    );
   }

   let mut digits = vec![];
   while to_decimal > 0{
    let remainder = to_decimal % to_base;
    digits.push(remainder);

    to_decimal /= to_base;
   }

    Ok(digits)
}
