use std::io::{Write, Read};

pub enum Endianness {
    BigEndian,
    LittleEndian
}

pub trait SerializationReflector {
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
}

pub trait SerializationScheme: Sized+Default {
    fn process<TSerializationReflector: SerializationReflector>(
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
    where T: SerializationScheme, TStream: Write
{
    let mut serializer = BinaryWriterBigEndian { stream };
    data.process(&mut serializer)
}

fn serialize_to_stream_le<'a, T, TStream>(
    data: &'a mut T,
    stream: &'a mut TStream
) -> std::io::Result<()>
    where T: SerializationScheme, TStream: Write
{
    let mut serializer = BinaryWriterLittleEndian { stream };
    data.process(&mut serializer)
}

fn deserialize_from_stream_be<'a, T, TStream>(
    data: &'a mut T,
    stream: &'a mut TStream
) -> std::io::Result<()>
    where T: SerializationScheme, TStream: Read
{
    let mut serializer = BinaryReaderBigEndian { stream };
    data.process(&mut serializer)
}

fn deserialize_from_stream_le<'a, T, TStream>(
    data: &'a mut T,
    stream: &'a mut TStream
) -> std::io::Result<()>
    where T: SerializationScheme, TStream: Read
{
    let mut serializer = BinaryReaderLittleEndian { stream };
    data.process(&mut serializer)
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
    use crate::{SerializationScheme, SerializationReflector, Endianness};

    #[derive(Default, Debug, Copy, Clone)]
    struct TestStruct {
        a: u32,
        b: u16,
        c: u32,
        d: u64,
        e: u8,
        f: u8
    }

    impl SerializationScheme for TestStruct {
        fn process<TSerializationReflector: SerializationReflector>(
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
}