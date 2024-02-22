
// Define a struct to represent a byte reader
// It will be used to parse the actual binary into a proper Manifest.

use std::ffi::CString;

use widestring::U16String;

use crate::{error::ParseError, ParseResult};

#[derive(Debug)]
pub struct ByteReader {
    data: Vec<u8>,
    position: usize
}

impl ByteReader {
    pub fn new(data: Vec<u8>) -> ByteReader {
        ByteReader {
            data,
            position: 0
        }
    }

    pub fn read_bytes(&mut self, size: usize) -> ParseResult<Vec<u8>> {
        if self.position + size > self.data.len() {
            return Err(ParseError::Overflow);
        }
        
        let mut result = Vec::with_capacity(size);
        for i in 0..size {
            result.push(self.data[self.position + i]);
        }
        self.position += size;
        
        Ok(result)
    }

    pub fn read<T:ByteReadable>(&mut self) -> ParseResult<T>
    {
        T::read(self)
    }

    pub fn tell(&self) -> usize {
        self.position
    }

    pub fn length(&self) -> usize {
        self.data.len()
    }

    pub fn seek(&mut self, position: usize) {
        self.position = position;
    }

    //may panic if the offset is too large
    pub fn offset(&mut self, offset: isize) {
        let pos = self.position as isize;

        //avoid underflow i guess
        if pos - offset < 0 {
            self.position = 0;
        } else {
            self.position = (pos + offset) as usize;
        }
    }

    pub fn read_array<T>(&mut self, mut read_item:impl FnMut(&mut Self) -> ParseResult<T>) -> ParseResult<Vec<T>> {
        let count = self.read::<u32>()? as usize;

        if count == 0 {
            return Ok(vec![]);
        } else {
            let mut result = Vec::with_capacity(count);
            for _ in 0..count {
                result.push(read_item(self)?);
            }
            Ok(result)
        }
    }
}


//TODO : Implement a trait to make the reader more generic
 pub trait ByteReadable: Sized {
     fn read(reader: &mut ByteReader) -> ParseResult<Self>;
 }

 impl ByteReadable for u64 {
    fn read(reader: &mut ByteReader) -> ParseResult<Self> {
       let result = u64::from_le_bytes(reader.read_bytes(8)?.try_into().map_err(|_| ParseError::InvalidData)?);
       Ok(result)
    }
}

impl ByteReadable for u32 {
     fn read(reader: &mut ByteReader) -> ParseResult<Self> {
        let result = u32::from_le_bytes(reader.read_bytes(4)?.try_into().map_err(|_| ParseError::InvalidData)?);
        Ok(result)
     }
}

impl ByteReadable for u16 {
     fn read(reader: &mut ByteReader) -> ParseResult<Self> {
        let result = u16::from_le_bytes(reader.read_bytes(2)?.try_into().map_err(|_| ParseError::InvalidData)?);
        Ok(result)
     }
}

impl ByteReadable for u8 {
     fn read(reader: &mut ByteReader) -> ParseResult<Self> {
        let result = u8::from_le_bytes(reader.read_bytes(1)?.try_into().map_err(|_| ParseError::InvalidData)?);
        Ok(result)
     }
}

impl ByteReadable for i64 {
     fn read(reader: &mut ByteReader) -> ParseResult<Self> {
        let result = i64::from_le_bytes(reader.read_bytes(8)?.try_into().map_err(|_| ParseError::InvalidData)?);
        Ok(result)
     }
}

impl ByteReadable for i32 {
     fn read(reader: &mut ByteReader) -> ParseResult<Self> {
        let result = i32::from_le_bytes(reader.read_bytes(4)?.try_into().map_err(|_| ParseError::InvalidData)?);
        Ok(result)
     }
}

impl ByteReadable for i16 {
     fn read(reader: &mut ByteReader) -> ParseResult<Self> {
        let result = i16::from_le_bytes(reader.read_bytes(2)?.try_into().map_err(|_| ParseError::InvalidData)?);
        Ok(result)
     }
}

impl ByteReadable for i8 {
     fn read(reader: &mut ByteReader) -> ParseResult<Self> {
        let result = i8::from_le_bytes(reader.read_bytes(1)?.try_into().map_err(|_| ParseError::InvalidData)?);
        Ok(result)
     }
}

impl ByteReadable for String {
    fn read(reader: &mut ByteReader) -> ParseResult<Self> {
        let length = reader.read::<i32>()?;

        if length == 0 {
            return Ok(String::new())
        }

        let utf_8 = length > 0;

        let string = if utf_8 {
            let c_string = CString::from_vec_with_nul(reader.read_bytes(length as usize)?).map_err(|_| ParseError::InvalidData)?;

            c_string.into_string().map_err(|_| ParseError::InvalidData)?
        } else {
            let length = (length * -2) as usize;
            let byte_data = reader.read_bytes(length)?;

            //shouldn't panic
            unsafe {
                let u16_string = U16String::from_ptr(byte_data.as_ptr() as *const u16, length.abs_diff(0));
                u16_string.to_string_lossy()
            }
        };

        Ok(string)
    }
}