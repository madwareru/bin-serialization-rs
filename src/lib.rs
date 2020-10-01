use std::io::{Write, Read};

pub enum Endianness {
    BigEndian,
    LittleEndian
}

pub trait SerializationReflector: Sized {
    fn reflect_u8(&mut self, data: &mut u8) -> std::io::Result<()>;
    fn reflect_u16(&mut self, data: &mut u16) -> std::io::Result<()>;
    fn reflect_u32(&mut self, data: &mut u32) -> std::io::Result<()>;
    fn reflect_u64(&mut self, data: &mut u64) -> std::io::Result<()>;
    fn reflect_i8(&mut self, data: &mut i8) -> std::io::Result<()>;
    fn reflect_i16(&mut self, data: &mut i16) -> std::io::Result<()>;
    fn reflect_i32(&mut self, data: &mut i32) -> std::io::Result<()>;
    fn reflect_i64(&mut self, data: &mut i64) -> std::io::Result<()>;
    fn reflect_f32(&mut self, data: &mut f32) -> std::io::Result<()>;
    fn reflect_f64(&mut self, data: &mut f64) -> std::io::Result<()>;
    fn reflect_u8_array(&mut self, data: &mut Vec<u8>) -> std::io::Result<()> {
        reflect_vec_size(self, data)?;
        for i in 0..data.len(){
            self.reflect_u8(&mut data[i])?;
        }
        Ok(())
    }
    fn reflect_u16_array(&mut self, data: &mut Vec<u16>) -> std::io::Result<()> {
        reflect_vec_size(self, data)?;
        for i in 0..data.len(){
            self.reflect_u16(&mut data[i])?;
        }
        Ok(())
    }
    fn reflect_u32_array(&mut self, data: &mut Vec<u32>) -> std::io::Result<()> {
        reflect_vec_size(self, data)?;
        for i in 0..data.len(){
            self.reflect_u32(&mut data[i as usize])?;
        }
        Ok(())
    }
    fn reflect_u64_array(&mut self, data: &mut Vec<u64>) -> std::io::Result<()> {
        reflect_vec_size(self, data)?;
        for i in 0..data.len(){
            self.reflect_u64(&mut data[i as usize])?;
        }
        Ok(())
    }
    fn reflect_i8_array(&mut self, data: &mut Vec<i8>) -> std::io::Result<()> {
        reflect_vec_size(self, data)?;
        for i in 0..data.len(){
            self.reflect_i8(&mut data[i as usize])?;
        }
        Ok(())
    }
    fn reflect_i16_array(&mut self, data: &mut Vec<i16>) -> std::io::Result<()> {
        reflect_vec_size(self, data)?;
        for i in 0..data.len(){
            self.reflect_i16(&mut data[i as usize])?;
        }
        Ok(())
    }
    fn reflect_i32_array(&mut self, data: &mut Vec<i32>) -> std::io::Result<()> {
        reflect_vec_size(self, data)?;
        for i in 0..data.len(){
            self.reflect_i32(&mut data[i as usize])?;
        }
        Ok(())
    }
    fn reflect_i64_array(&mut self, data: &mut Vec<i64>) -> std::io::Result<()> {
        reflect_vec_size(self, data)?;
        for i in 0..data.len(){
            self.reflect_i64(&mut data[i as usize])?;
        }
        Ok(())
    }
    fn reflect_f32_array(&mut self, data: &mut Vec<f32>) -> std::io::Result<()> {
        reflect_vec_size(self, data)?;
        for i in 0..data.len(){
            self.reflect_f32(&mut data[i as usize])?;
        }
        Ok(())
    }
    fn reflect_f64_array(&mut self, data: &mut Vec<f64>) -> std::io::Result<()> {
        reflect_vec_size(self, data)?;
        for i in 0..data.len(){
            self.reflect_f64(&mut data[i as usize])?;
        }
        Ok(())
    }
    fn reflect_string(&mut self, string: &mut String) -> std::io::Result<()> {
        unsafe {
            let vec_repr = string.as_mut_vec();
            self.reflect_u8_array(vec_repr)?;
        }
        Ok(())
    }
    fn reflect_composite<R: Reflectable>(&mut self, composite: &mut R) -> std::io::Result<()> {
        composite.reflect(self)
    }
    fn reflect_bool(&mut self, data: &mut bool) -> std::io::Result<()> {
        let mut casted = if *data { 1 } else { 0 };
        self.reflect_u8(&mut casted)?;
        *data = casted != 0;
        Ok(())
    }
}

fn reflect_size<R: SerializationReflector>(r: &mut R, s: &mut usize) -> std::io::Result<()> {
    let mut size = *s;
    let mut tag = if size <= 0xFF {
        1
    } else if size <= 0xFFFF {
        2
    } else if size <= 0xFFFFFFFF {
        4
    } else {
        8
    };
    r.reflect_u8(&mut tag)?;
    size = match tag {
        1 => {
            let mut size_u8 = size as u8;
            r.reflect_u8(&mut size_u8)?;
            size_u8 as usize
        },
        2 => {
            let mut size_u16 = size as u16;
            r.reflect_u16(&mut size_u16)?;
            size_u16 as usize
        },
        4 => {
            let mut size_u32 = size as u32;
            r.reflect_u32(&mut size_u32)?;
            size_u32 as usize
        },
        8 => {
            let mut size_u64 = size as u64;
            r.reflect_u64(&mut size_u64)?;
            size_u64 as usize
        },
        _ => unreachable!()
    };
    *s = size;
    Ok(())
}

fn reflect_vec_size<R: SerializationReflector, T: Default+Clone>(r: &mut R, v: &mut Vec<T>) -> std::io::Result<()> {
    let mut size = v.len();
    reflect_size(r, &mut size)?;
    if v.len() != size {
        v.resize(size, Default::default());
    }
    Ok(())
}

pub trait Reflectable: Sized+Default {
    fn reflect<TSerializationReflector: SerializationReflector>(
        &mut self,
        reflector: &mut TSerializationReflector
    ) -> std::io::Result<()>;
    fn serialize<TStream: Write>(
        &mut self,
        stream: &mut TStream,
        endianness: Endianness
    ) -> std::io::Result<()> {
        match endianness {
            Endianness::BigEndian => serialize_to_stream_be(self, stream),
            Endianness::LittleEndian => serialize_to_stream_le(self, stream)
        }
    }
    fn deserialize<TStream: Read>(
        stream: &mut TStream,
        endianness: Endianness
    ) -> std::io::Result<Self> {
        let mut data = Default::default();
        match endianness {
            Endianness::BigEndian => deserialize_from_stream_be(&mut data, stream),
            Endianness::LittleEndian => deserialize_from_stream_le(&mut data, stream)
        }?;
        Ok(data)
    }
}

fn serialize_to_stream_be<'a, T, TStream>(
    data: &'a mut T,
    stream: &'a mut TStream
) -> std::io::Result<()>
    where T: Reflectable, TStream: Write
{
    let mut serializer = BinaryWriterBigEndian { stream };
    data.reflect(&mut serializer)
}

fn serialize_to_stream_le<'a, T, TStream>(
    data: &'a mut T,
    stream: &'a mut TStream
) -> std::io::Result<()>
    where T: Reflectable, TStream: Write
{
    let mut serializer = BinaryWriterLittleEndian { stream };
    data.reflect(&mut serializer)
}

fn deserialize_from_stream_be<'a, T, TStream>(
    data: &'a mut T,
    stream: &'a mut TStream
) -> std::io::Result<()>
    where T: Reflectable, TStream: Read
{
    let mut serializer = BinaryReaderBigEndian { stream };
    data.reflect(&mut serializer)
}

fn deserialize_from_stream_le<'a, T, TStream>(
    data: &'a mut T,
    stream: &'a mut TStream
) -> std::io::Result<()>
    where T: Reflectable, TStream: Read
{
    let mut serializer = BinaryReaderLittleEndian { stream };
    data.reflect(&mut serializer)
}

struct BinaryWriterBigEndian<'a, TStream: Write> {
    stream: &'a mut TStream
}

struct BinaryWriterLittleEndian<'a, TStream: Write> {
    stream: &'a mut TStream
}

impl<'a, TStream: Write> SerializationReflector for BinaryWriterBigEndian<'a, TStream> {
    fn reflect_u8(&mut self, data: &mut u8) -> std::io::Result<()> {
        let bytes_written = self.stream.write(&[*data])?;
        if bytes_written == 1 {
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "failed to write one byte"
            ))
        }
    }

    fn reflect_u16(&mut self, data: &mut u16) -> std::io::Result<()> {
        let mut d = data.to_be();
        let b0 = (d & 0xFF) as u8; d /= 0x100;
        let b1 = (d & 0xFF) as u8;
        let bytes_written = self.stream.write(&[b0, b1])?;
        if bytes_written == 2 {
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "failed to write two bytes"
            ))
        }
    }

    fn reflect_u32(&mut self, data: &mut u32) -> std::io::Result<()> {
        let mut d = data.to_be();
        let b0 = (d & 0xFF) as u8; d /= 0x100;
        let b1 = (d & 0xFF) as u8; d /= 0x100;
        let b2 = (d & 0xFF) as u8; d /= 0x100;
        let b3 = (d & 0xFF) as u8;
        let bytes_written = self.stream.write(&[b0, b1, b2, b3])?;
        if bytes_written == 4 {
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "failed to write four bytes"
            ))
        }
    }

    fn reflect_u64(&mut self, data: &mut u64) -> std::io::Result<()> {
        let mut d = data.to_be();
        let b0 = (d & 0xFF) as u8; d /= 0x100;
        let b1 = (d & 0xFF) as u8; d /= 0x100;
        let b2 = (d & 0xFF) as u8; d /= 0x100;
        let b3 = (d & 0xFF) as u8; d /= 0x100;
        let b4 = (d & 0xFF) as u8; d /= 0x100;
        let b5 = (d & 0xFF) as u8; d /= 0x100;
        let b6 = (d & 0xFF) as u8; d /= 0x100;
        let b7 = (d & 0xFF) as u8;
        let bytes_written = self.stream.write(&[b0, b1, b2, b3, b4, b5, b6, b7])?;
        if bytes_written == 8 {
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "failed to write eight bytes"
            ))
        }
    }

    fn reflect_i8(&mut self, data: &mut i8) -> std::io::Result<()> {
        let mut data = unsafe {
            let x  = &[*data] as *const i8;
            let x = x as *const u8;
            *x
        };
        self.reflect_u8(&mut data)
    }

    fn reflect_i16(&mut self, data: &mut i16) -> std::io::Result<()> {
        let mut data = unsafe {
            let x  = &[*data] as *const i16;
            let x = x as *const u16;
            *x
        };
        self.reflect_u16(&mut data)
    }

    fn reflect_i32(&mut self, data: &mut i32) -> std::io::Result<()> {
        let mut data = unsafe {
            let x  = &[*data] as *const i32;
            let x = x as *const u32;
            *x
        };
        self.reflect_u32(&mut data)
    }

    fn reflect_i64(&mut self, data: &mut i64) -> std::io::Result<()>{
        let mut data = unsafe {
            let x  = &[*data] as *const i64;
            let x = x as *const u64;
            *x
        };
        self.reflect_u64(&mut data)
    }

    fn reflect_f32(&mut self, data: &mut f32) -> std::io::Result<()> {
        let mut data = unsafe {
            let x  = &[*data] as *const f32;
            let x = x as *const u32;
            *x
        };
        self.reflect_u32(&mut data)
    }

    fn reflect_f64(&mut self, data: &mut f64) -> std::io::Result<()> {
        let mut data = unsafe {
            let x  = &[*data] as *const f64;
            let x = x as *const u64;
            *x
        };
        self.reflect_u64(&mut data)
    }
}

impl<'a, TStream: Write> SerializationReflector for BinaryWriterLittleEndian<'a, TStream> {
    fn reflect_u8(&mut self, data: &mut u8) -> std::io::Result<()> {
        let bytes_written = self.stream.write(&[*data])?;
        if bytes_written == 1 {
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "failed to write two bytes"
            ))
        }
    }

    fn reflect_u16(&mut self, data: &mut u16) -> std::io::Result<()> {
        let mut d = data.to_le();
        let b0 = (d & 0xFF) as u8; d /= 0x100;
        let b1 = (d & 0xFF) as u8;
        let bytes_written = self.stream.write(&[b0, b1])?;
        if bytes_written == 2 {
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "failed to write two bytes"
            ))
        }
    }

    fn reflect_u32(&mut self, data: &mut u32) -> std::io::Result<()> {
        let mut d = data.to_le();
        let b0 = (d & 0xFF) as u8; d /= 0x100;
        let b1 = (d & 0xFF) as u8; d /= 0x100;
        let b2 = (d & 0xFF) as u8; d /= 0x100;
        let b3 = (d & 0xFF) as u8;
        let bytes_written = self.stream.write(&[b0, b1, b2, b3])?;
        if bytes_written == 4 {
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "failed to write four bytes"
            ))
        }
    }

    fn reflect_u64(&mut self, data: &mut u64) -> std::io::Result<()> {
        let mut d = data.to_le();
        let b0 = (d & 0xFF) as u8; d /= 0x100;
        let b1 = (d & 0xFF) as u8; d /= 0x100;
        let b2 = (d & 0xFF) as u8; d /= 0x100;
        let b3 = (d & 0xFF) as u8; d /= 0x100;
        let b4 = (d & 0xFF) as u8; d /= 0x100;
        let b5 = (d & 0xFF) as u8; d /= 0x100;
        let b6 = (d & 0xFF) as u8; d /= 0x100;
        let b7 = (d & 0xFF) as u8;
        let bytes_written = self.stream.write(&[b0, b1, b2, b3, b4, b5, b6, b7])?;
        if bytes_written == 8 {
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "failed to write eight bytes"
            ))
        }
    }

    fn reflect_i8(&mut self, data: &mut i8) -> std::io::Result<()> {
        let mut data = unsafe {
            let x  = &[*data] as *const i8;
            let x = x as *const u8;
            *x
        };
        self.reflect_u8(&mut data)
    }

    fn reflect_i16(&mut self, data: &mut i16) -> std::io::Result<()> {
        let mut data = unsafe {
            let x  = &[*data] as *const i16;
            let x = x as *const u16;
            *x
        };
        self.reflect_u16(&mut data)
    }

    fn reflect_i32(&mut self, data: &mut i32) -> std::io::Result<()> {
        let mut data = unsafe {
            let x  = &[*data] as *const i32;
            let x = x as *const u32;
            *x
        };
        self.reflect_u32(&mut data)
    }

    fn reflect_i64(&mut self, data: &mut i64) -> std::io::Result<()>{
        let mut data = unsafe {
            let x  = &[*data] as *const i64;
            let x = x as *const u64;
            *x
        };
        self.reflect_u64(&mut data)
    }

    fn reflect_f32(&mut self, data: &mut f32) -> std::io::Result<()> {
        let mut data = unsafe {
            let x  = &[*data] as *const f32;
            let x = x as *const u32;
            *x
        };
        self.reflect_u32(&mut data)
    }

    fn reflect_f64(&mut self, data: &mut f64) -> std::io::Result<()> {
        let mut data = unsafe {
            let x  = &[*data] as *const f64;
            let x = x as *const u64;
            *x
        };
        self.reflect_u64(&mut data)
    }
}

struct BinaryReaderBigEndian<'a, TStream: Read> {
    stream: &'a mut TStream
}

struct BinaryReaderLittleEndian<'a, TStream: Read> {
    stream: &'a mut TStream
}

impl<'a, TStream: Read> SerializationReflector for BinaryReaderBigEndian<'a, TStream> {
    fn reflect_u8(&mut self, data: &mut u8) -> std::io::Result<()> {
        let d = &mut [0];
        let size_read = self.stream.read(d)?;
        *data = d[0];
        if size_read == 1 {
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "failed to read one byte"
            ))
        }
    }

    fn reflect_u16(&mut self, data: &mut u16) -> std::io::Result<()> {
        let d = &mut [0; 2];
        let size_read = self.stream.read(d)?;
        *data = u16::from_be(d[0] as u16 + d[1] as u16 * 0x100);
        if size_read == 2 {
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "failed to read two bytes"
            ))
        }
    }

    fn reflect_u32(&mut self, data: &mut u32) -> std::io::Result<()> {
        let d = &mut [0; 4];
        let size_read = self.stream.read(d)?;
        *data = u32::from_be(d[0] as u32 +
            d[1] as u32 * 0x100 +
            d[2] as u32 * 0x10000 +
            d[3] as u32 * 0x1000000
        );
        if size_read == 4 {
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "failed to read four bytes"
            ))
        }
    }

    fn reflect_u64(&mut self, data: &mut u64) -> std::io::Result<()> {
        let d = &mut [0; 8];
        let size_read = self.stream.read(d)?;
        *data = u64::from_be(d[0] as u64 +
            d[1] as u64 * 0x100 +
            d[2] as u64 * 0x10000 +
            d[3] as u64 * 0x1000000 +
            d[4] as u64 * 0x100000000 +
            d[5] as u64 * 0x10000000000 +
            d[6] as u64 * 0x1000000000000 +
            d[7] as u64 * 0x100000000000000
        );
        if size_read == 8 {
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "failed to read eight bytes"
            ))
        }
    }

    fn reflect_i8(&mut self, data: &mut i8) -> std::io::Result<()> {
        let mut d = 0u8;
        self.reflect_u8(&mut d)?;
        *data = unsafe {
            let x = &[d] as *const u8;
            let x = x as *const i8;
            *x
        };
        Ok(())
    }

    fn reflect_i16(&mut self, data: &mut i16) -> std::io::Result<()> {
        let mut d = 0u16;
        self.reflect_u16(&mut d)?;
        *data = unsafe {
            let x = &[d] as *const u16;
            let x = x as *const i16;
            *x
        };
        Ok(())
    }

    fn reflect_i32(&mut self, data: &mut i32) -> std::io::Result<()> {
        let mut d = 0u32;
        self.reflect_u32(&mut d)?;
        *data = unsafe {
            let x = &[d] as *const u32;
            let x = x as *const i32;
            *x
        };
        Ok(())
    }

    fn reflect_i64(&mut self, data: &mut i64) -> std::io::Result<()> {
        let mut d = 0u64;
        self.reflect_u64(&mut d)?;
        *data = unsafe {
            let x = &[d] as *const u64;
            let x = x as *const i64;
            *x
        };
        Ok(())
    }

    fn reflect_f32(&mut self, data: &mut f32) -> std::io::Result<()> {
        let mut d = 0u32;
        self.reflect_u32(&mut d)?;
        *data = unsafe {
            let x = &[d] as *const u32;
            let x = x as *const f32;
            *x
        };
        Ok(())
    }

    fn reflect_f64(&mut self, data: &mut f64) -> std::io::Result<()> {
        let mut d = 0u64;
        self.reflect_u64(&mut d)?;
        *data = unsafe {
            let x  = &[d] as *const u64;
            let x = x as *const f64;
            *x
        };
        Ok(())
    }
}

impl<'a, TStream: Read> SerializationReflector for BinaryReaderLittleEndian<'a, TStream> {
    fn reflect_u8(&mut self, data: &mut u8) -> std::io::Result<()> {
        let d = &mut [0];
        let size_read = self.stream.read(d)?;
        *data = d[0];
        if size_read == 1 {
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "failed to read one byte"
            ))
        }
    }

    fn reflect_u16(&mut self, data: &mut u16) -> std::io::Result<()> {
        let d = &mut [0; 2];
        let size_read = self.stream.read(d)?;
        *data = u16::from_le(d[0] as u16 + d[1] as u16 * 0x100);
        if size_read == 2 {
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "failed to read two bytes"
            ))
        }
    }

    fn reflect_u32(&mut self, data: &mut u32) -> std::io::Result<()> {
        let d = &mut [0; 4];
        let size_read = self.stream.read(d)?;
        *data = u32::from_le(d[0] as u32 +
            d[1] as u32 * 0x100 +
            d[2] as u32 * 0x10000 +
            d[3] as u32 * 0x1000000
        );
        if size_read == 4 {
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "failed to read four bytes"
            ))
        }
    }

    fn reflect_u64(&mut self, data: &mut u64) -> std::io::Result<()> {
        let d = &mut [0; 8];
        let size_read = self.stream.read(d)?;
        *data = u64::from_le(d[0] as u64 +
            d[1] as u64 * 0x100 +
            d[2] as u64 * 0x10000 +
            d[3] as u64 * 0x1000000 +
            d[4] as u64 * 0x100000000 +
            d[5] as u64 * 0x10000000000 +
            d[6] as u64 * 0x1000000000000 +
            d[7] as u64 * 0x100000000000000
        );
        if size_read == 8 {
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "failed to read eight bytes"
            ))
        }
    }

    fn reflect_i8(&mut self, data: &mut i8) -> std::io::Result<()> {
        let mut d = 0u8;
        self.reflect_u8(&mut d)?;
        *data = unsafe {
            let x = &[d] as *const u8;
            let x = x as *const i8;
            *x
        };
        Ok(())
    }

    fn reflect_i16(&mut self, data: &mut i16) -> std::io::Result<()> {
        let mut d = 0u16;
        self.reflect_u16(&mut d)?;
        *data = unsafe {
            let x = &[d] as *const u16;
            let x = x as *const i16;
            *x
        };
        Ok(())
    }

    fn reflect_i32(&mut self, data: &mut i32) -> std::io::Result<()> {
        let mut d = 0u32;
        self.reflect_u32(&mut d)?;
        *data = unsafe {
            let x = &[d] as *const u32;
            let x = x as *const i32;
            *x
        };
        Ok(())
    }

    fn reflect_i64(&mut self, data: &mut i64) -> std::io::Result<()> {
        let mut d = 0u64;
        self.reflect_u64(&mut d)?;
        *data = unsafe {
            let x = &[d] as *const u64;
            let x = x as *const i64;
            *x
        };
        Ok(())
    }

    fn reflect_f32(&mut self, data: &mut f32) -> std::io::Result<()> {
        let mut d = 0u32;
        self.reflect_u32(&mut d)?;
        *data = unsafe {
            let x = &[d] as *const u32;
            let x = x as *const f32;
            *x
        };
        Ok(())
    }

    fn reflect_f64(&mut self, data: &mut f64) -> std::io::Result<()> {
        let mut d = 0u64;
        self.reflect_u64(&mut d)?;
        *data = unsafe {
            let x  = &[d] as *const u64;
            let x = x as *const f64;
            *x
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Cursor, Seek, SeekFrom};
    use crate::{Reflectable, SerializationReflector, Endianness, BinaryReaderLittleEndian, BinaryReaderBigEndian, BinaryWriterLittleEndian};

    #[derive(Default, Debug, Copy, Clone)]
    struct TestStruct {
        a: u32,
        b: u16,
        c: u32,
        d: u64,
        e: u8,
        f: u8
    }

    impl Reflectable for TestStruct {
        fn reflect<TSerializationReflector: SerializationReflector>(
            &mut self, reflector:
            &mut TSerializationReflector
        ) -> std::io::Result<()> {
            reflector.reflect_u32(&mut self.a)?;
            reflector.reflect_u16(&mut self.b)?;
            reflector.reflect_u32(&mut self.c)?;
            reflector.reflect_u64(&mut self.d)?;
            reflector.reflect_u8(&mut self.e)?;
            reflector.reflect_u8(&mut self.f)
        }
    }

    #[test]
    fn test_trivial_reads() {
        let test_set = &[
            0, 0, 1, 0, 1, 0, 1, 0, 0, 0,
            1, 0, 0, 0, 0, 0, 0, 0, 1, 1
        ];
        let mut cursor = Cursor::new(test_set);
        let le = TestStruct::deserialize(
            &mut cursor,
            Endianness::LittleEndian
        ).unwrap();
        cursor.seek(SeekFrom::Start(0)).unwrap();
        let be = TestStruct::deserialize(
            &mut cursor,
            Endianness::BigEndian
        ).unwrap();

        assert_eq!(le.a, 0x00010000);
        assert_eq!(le.b, 1);
        assert_eq!(le.c, 1);
        assert_eq!(le.d, 1);
        assert_eq!(le.e, 1);
        assert_eq!(le.f, 1);

        assert_eq!(be.a, 0x00000100);
        assert_eq!(be.b, 0x0100);
        assert_eq!(be.c, 0x01000000);
        assert_eq!(be.d, 0x0100000000000000);
        assert_eq!(be.e, 1);
        assert_eq!(be.f, 1);
    }

    #[test]
    fn test_array_read() {
        let test_set = &[
             1, 20,
             1,  2,  3,  4,  5,  6,  7,  8,
             9, 10, 11, 12, 13, 14, 15, 16,
            17, 18, 19, 20
        ];
        let mut deserializer = BinaryReaderLittleEndian{stream: &mut &test_set[..]};
        let mut vec = vec![0u8; 20];
        deserializer.reflect_u8_array(&mut vec).unwrap();
        assert_eq!(vec, vec![
            1,  2,  3,  4,  5,  6,  7,  8,
            9, 10, 11, 12, 13, 14, 15, 16,
            17, 18, 19, 20
        ]);
        let test_set = &[
            1, 20,
            1,  2,  3,  4,  5,  6,  7,  8,
            9, 10, 11, 12, 13, 14, 15, 16,
            17, 18, 19, 20
        ];
        let mut deserializer = BinaryReaderBigEndian{stream: &mut &test_set[..]};
        let mut vec = vec![0u8; 20];
        deserializer.reflect_u8_array(&mut vec).unwrap();
        assert_eq!(vec, vec![
            1,  2,  3,  4,  5,  6,  7,  8,
            9, 10, 11, 12, 13, 14, 15, 16,
            17, 18, 19, 20
        ]);
    }

    #[test]
    fn test_icorrect_length() {
        let test_set1 = &[
            0, 0, 1, 0, 1, 0, 1, 0, 0, 0,
            1, 0, 0, 0, 0, 0, 0, 0
        ];
        let mut cursor1 = Cursor::new(test_set1);
        let test_set2 = &[
            0, 0, 1, 0, 1, 0, 1, 0, 0, 0,
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
        ];
        let mut cursor2 = Cursor::new(test_set2);
        let deserialize_trial = TestStruct::deserialize(
            &mut cursor1,
            Endianness::LittleEndian
        );
        assert!(deserialize_trial.is_err());
        let deserialize_trial = TestStruct::deserialize(
            &mut cursor2,
            Endianness::LittleEndian
        );
        assert!(deserialize_trial.is_ok());
    }

    #[test]
    fn test_string_serialization() {
        let mut s = "Hey dude".to_string();
        let mut s2 = "Yo dude".to_string();
        let mut stream_vec = Vec::new();
        let mut serializer = BinaryWriterLittleEndian{ stream: &mut stream_vec };
        serializer.reflect_string(&mut s).unwrap();
        let mut deserializer = BinaryReaderLittleEndian{ stream: &mut &stream_vec[..] };
        deserializer.reflect_string(&mut s2).unwrap();
        assert_eq!(s, s2);
    }

    #[test]
    fn test_integers_and_floats_serialization() {
        let mut a = 13;
        let mut b = 3123;
        let mut c = -23123131;
        let mut d = 10.43345;
        let mut e = 17.1231231;

        let mut a1 = 0;
        let mut b1 = 0;
        let mut c1 = 0;
        let mut d1 = 0.0;
        let mut e1 = 0.0;

        let mut stream_vec = Vec::new();

        let mut serializer = BinaryWriterLittleEndian{ stream: &mut stream_vec };
        serializer.reflect_i8(&mut a).unwrap();
        serializer.reflect_i16(&mut b).unwrap();
        serializer.reflect_i32(&mut c).unwrap();
        serializer.reflect_f32(&mut d).unwrap();
        serializer.reflect_f64(&mut e).unwrap();

        let mut deserializer = BinaryReaderLittleEndian{ stream: &mut &stream_vec[..] };
        deserializer.reflect_i8(&mut a1).unwrap();
        deserializer.reflect_i16(&mut b1).unwrap();
        deserializer.reflect_i32(&mut c1).unwrap();
        deserializer.reflect_f32(&mut d1).unwrap();
        deserializer.reflect_f64(&mut e1).unwrap();

        assert_eq!(a, a1);
        assert_eq!(b, b1);
        assert_eq!(c, c1);
        assert_eq!(d, d1);
        assert_eq!(e, e1);
    }
}