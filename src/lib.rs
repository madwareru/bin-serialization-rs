use std::io::{Write, Read};

#[derive(Copy, Clone, PartialEq)]
pub enum Endianness {
    BigEndian,
    LittleEndian
}

#[derive(Copy, Clone, PartialEq)]
pub enum SizePolicy {
    U8,
    U16,
    U32,
    U64
}

fn check(cond: bool, errText: &str) -> std::io::Result<()> {
    if cond {
        Some(())
    } else {
        None
    }.ok_or(std::io::Error::new(std::io::ErrorKind::Other, errText))
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
    fn reflect_cp866_string(&mut self, string: &mut String) -> std::io::Result<()>;
    fn reflect_cp866_zstring(&mut self, string: &mut String) -> std::io::Result<()>;
    fn reflect_cp866_zstring_ext(&mut self, length: usize, string: &mut String) -> std::io::Result<()>;
    fn reflect_u8_array(&mut self, data: &mut Vec<u8>) -> std::io::Result<()> {
        reflect_vec_size(self, data)?;
        for i in 0..data.len(){
            self.reflect_u8(&mut data[i])?;
        }
        Ok(())
    }
    fn reflect_u8_array_ext(&mut self, data: &mut Vec<u8>, size_policy: SizePolicy) -> std::io::Result<()> {
        reflect_vec_size_ext(self, data, size_policy)?;
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
    fn reflect_u16_array_ext(&mut self, data: &mut Vec<u16>, size_policy: SizePolicy) -> std::io::Result<()> {
        reflect_vec_size_ext(self, data, size_policy)?;
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
    fn reflect_u32_array_ext(&mut self, data: &mut Vec<u32>, size_policy: SizePolicy) -> std::io::Result<()> {
        reflect_vec_size_ext(self, data, size_policy)?;
        for i in 0..data.len(){
            self.reflect_u32(&mut data[i])?;
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
    fn reflect_u64_array_ext(&mut self, data: &mut Vec<u64>, size_policy: SizePolicy) -> std::io::Result<()> {
        reflect_vec_size_ext(self, data, size_policy)?;
        for i in 0..data.len(){
            self.reflect_u64(&mut data[i])?;
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
    fn reflect_i8_array_ext(&mut self, data: &mut Vec<i8>, size_policy: SizePolicy) -> std::io::Result<()> {
        reflect_vec_size_ext(self, data, size_policy)?;
        for i in 0..data.len(){
            self.reflect_i8(&mut data[i])?;
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
    fn reflect_i16_array_ext(&mut self, data: &mut Vec<i16>, size_policy: SizePolicy) -> std::io::Result<()> {
        reflect_vec_size_ext(self, data, size_policy)?;
        for i in 0..data.len(){
            self.reflect_i16(&mut data[i])?;
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
    fn reflect_i32_array_ext(&mut self, data: &mut Vec<i32>, size_policy: SizePolicy) -> std::io::Result<()> {
        reflect_vec_size_ext(self, data, size_policy)?;
        for i in 0..data.len(){
            self.reflect_i32(&mut data[i])?;
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
    fn reflect_i64_array_ext(&mut self, data: &mut Vec<i64>, size_policy: SizePolicy) -> std::io::Result<()> {
        reflect_vec_size_ext(self, data, size_policy)?;
        for i in 0..data.len(){
            self.reflect_i64(&mut data[i])?;
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
    fn reflect_f32_array_ext(&mut self, data: &mut Vec<f32>, size_policy: SizePolicy) -> std::io::Result<()> {
        reflect_vec_size_ext(self, data, size_policy)?;
        for i in 0..data.len(){
            self.reflect_f32(&mut data[i])?;
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
    fn reflect_f64_array_ext(&mut self, data: &mut Vec<f64>, size_policy: SizePolicy) -> std::io::Result<()> {
        reflect_vec_size_ext(self, data, size_policy)?;
        for i in 0..data.len(){
            self.reflect_f64(&mut data[i])?;
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
    fn reflect_string_ext(&mut self, string: &mut String, size_policy: SizePolicy) -> std::io::Result<()> {
        unsafe {
            let vec_repr = string.as_mut_vec();
            self.reflect_u8_array_ext(vec_repr, size_policy)?;
        }
        Ok(())
    }
    fn reflect_bool(&mut self, data: &mut bool) -> std::io::Result<()> {
        let mut casted = if *data { 1 } else { 0 };
        self.reflect_u8(&mut casted)?;
        *data = casted != 0;
        Ok(())
    }
    fn reflect_bool_array(&mut self, data: &mut Vec<bool>) -> std::io::Result<()> {
        reflect_vec_size(self, data)?;
        for i in 0..data.len(){
            self.reflect_bool(&mut data[i])?;
        }
        Ok(())
    }
    fn reflect_bool_array_ext(&mut self, data: &mut Vec<bool>, size_policy: SizePolicy) -> std::io::Result<()> {
        reflect_vec_size_ext(self, data, size_policy)?;
        for i in 0..data.len(){
            self.reflect_bool(&mut data[i])?;
        }
        Ok(())
    }
    fn reflect_composite<R: Reflectable>(&mut self, composite: &mut R) -> std::io::Result<()> {
        composite.reflect(self)
    }
    fn reflect_array_of_composites<R: Reflectable>(&mut self, data: &mut Vec<R>) -> std::io::Result<()> {
        reflect_vec_size(self, data)?;
        for i in 0..data.len(){
            self.reflect_composite(&mut data[i])?;
        }
        Ok(())
    }
    fn reflect_array_of_composites_ext<R: Reflectable>(
        &mut self,
        data: &mut Vec<R>,
        size_policy: SizePolicy
    ) -> std::io::Result<()> {
        reflect_vec_size_ext(self, data, size_policy)?;
        for i in 0..data.len(){
            self.reflect_composite(&mut data[i])?;
        }
        Ok(())
    }
    fn reflect_tagged_composite<R: TaggedReflectable>(
        &mut self,
        composite: &mut R
    ) -> std::io::Result<()> {
        composite.reflect(self)
    }
    fn reflect_array_of_tagged_composites<R: TaggedReflectable>(
        &mut self,
        data: &mut Vec<R>
    ) -> std::io::Result<()> {
        reflect_vec_size(self, data)?;
        for i in 0..data.len(){
            self.reflect_tagged_composite(&mut data[i])?;
        }
        Ok(())
    }
    fn reflect_array_of_tagged_composites_ext<R: TaggedReflectable>(
        &mut self,
        data: &mut Vec<R>,
        size_policy: SizePolicy
    ) -> std::io::Result<()> {
        reflect_vec_size_ext(self, data, size_policy)?;
        for i in 0..data.len(){
            self.reflect_tagged_composite(&mut data[i])?;
        }
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

fn reflect_size_ext<R: SerializationReflector>(
    r: &mut R, s:
    &mut usize,
    size_policy: SizePolicy
) -> std::io::Result<()> {
    let mut size = *s;
    let mut tag = match size_policy {
        SizePolicy::U8 => 1,
        SizePolicy::U16 => 2,
        SizePolicy::U32 => 4,
        SizePolicy::U64 => 8
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

fn reflect_vec_size_ext<R: SerializationReflector, T: Default+Clone>(
    r: &mut R,
    v: &mut Vec<T>,
    size_policy: SizePolicy
) -> std::io::Result<()> {
    let mut size = v.len();
    reflect_size_ext(r, &mut size, size_policy)?;
    if v.len() != size {
        v.resize(size, Default::default());
    }
    Ok(())
}

pub trait TaggedReflectable: Default+Clone {
    fn get_size_policy() -> SizePolicy;
    fn get_tag(&self) -> u64;
    fn reflect_tagged<TSerializationReflector: SerializationReflector>(
        &mut self,
        tag: u64,
        reflector: &mut TSerializationReflector
    ) -> std::io::Result<()>;
    fn reflect<TSerializationReflector: SerializationReflector>(
        &mut self,
        reflector: &mut TSerializationReflector
    ) -> std::io::Result<()> {
        let tag = match Self::get_size_policy() {
            SizePolicy::U8 => {
                let mut tag = self.get_tag() as u8;
                reflector.reflect_u8(&mut tag)?;
                tag as u64
            }
            SizePolicy::U16 => {
                let mut tag = self.get_tag() as u16;
                reflector.reflect_u16(&mut tag)?;
                tag as u64
            }
            SizePolicy::U32 => {
                let mut tag = self.get_tag() as u32;
                reflector.reflect_u32(&mut tag)?;
                tag as u64
            }
            SizePolicy::U64 => {
                let mut tag = self.get_tag();
                reflector.reflect_u64(&mut tag)?;
                tag as u64
            }
        };
        self.reflect_tagged(tag, reflector)
    }
    fn serialize<TStream: Write>(
        &mut self,
        stream: &mut TStream,
        endianness: Endianness
    ) -> std::io::Result<()> {
        match endianness {
            Endianness::BigEndian => serialize_tagged_to_stream_be(self, stream),
            Endianness::LittleEndian => serialize_tagged_to_stream_le(self, stream)
        }
    }
    fn deserialize<TStream: Read>(
        stream: &mut TStream,
        endianness: Endianness
    ) -> std::io::Result<Self> {
        let mut data = Default::default();
        match endianness {
            Endianness::BigEndian => deserialize_tagged_from_stream_be(&mut data, stream),
            Endianness::LittleEndian => deserialize_tagged_from_stream_le(&mut data, stream)
        }?;
        Ok(data)
    }
}

pub trait Reflectable: Default+Clone {
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
fn serialize_tagged_to_stream_be<'a, T, TStream>(
    data: &'a mut T,
    stream: &'a mut TStream
) -> std::io::Result<()>
    where T: TaggedReflectable, TStream: Write
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
fn serialize_tagged_to_stream_le<'a, T, TStream>(
    data: &'a mut T,
    stream: &'a mut TStream
) -> std::io::Result<()>
    where T: TaggedReflectable, TStream: Write
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
fn deserialize_tagged_from_stream_be<'a, T, TStream>(
    data: &'a mut T,
    stream: &'a mut TStream
) -> std::io::Result<()>
    where T: TaggedReflectable, TStream: Read
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
fn deserialize_tagged_from_stream_le<'a, T, TStream>(
    data: &'a mut T,
    stream: &'a mut TStream
) -> std::io::Result<()>
    where T: TaggedReflectable, TStream: Read
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
        let bytes_written = self.stream.write(&data.to_be_bytes())?;
        check(bytes_written == 1, "failed to write one byte")
    }

    fn reflect_u16(&mut self, data: &mut u16) -> std::io::Result<()> {
        let bytes_written = self.stream.write(&data.to_be_bytes())?;
        check(bytes_written == 2, "failed to write two bytes")
    }

    fn reflect_u32(&mut self, data: &mut u32) -> std::io::Result<()> {
        let bytes_written = self.stream.write(&data.to_be_bytes())?;
        check(bytes_written == 4, "failed to write four bytes")
    }

    fn reflect_u64(&mut self, data: &mut u64) -> std::io::Result<()> {
        let bytes_written = self.stream.write(&data.to_be_bytes())?;
        check(bytes_written == 8, "failed to write eight bytes")
    }

    fn reflect_i8(&mut self, data: &mut i8) -> std::io::Result<()> {
        let bytes_written = self.stream.write(&data.to_be_bytes())?;
        check(bytes_written == 1, "failed to write one byte")
    }

    fn reflect_i16(&mut self, data: &mut i16) -> std::io::Result<()> {
        let bytes_written = self.stream.write(&data.to_be_bytes())?;
        check(bytes_written == 2, "failed to write two bytes")
    }

    fn reflect_i32(&mut self, data: &mut i32) -> std::io::Result<()> {
        let bytes_written = self.stream.write(&data.to_be_bytes())?;
        check(bytes_written == 4, "failed to write four bytes")
    }

    fn reflect_i64(&mut self, data: &mut i64) -> std::io::Result<()>{
        let bytes_written = self.stream.write(&data.to_be_bytes())?;
        check(bytes_written == 8, "failed to write eight bytes")
    }

    fn reflect_f32(&mut self, data: &mut f32) -> std::io::Result<()> {
        let bytes_written = self.stream.write(&data.to_be_bytes())?;
        check(bytes_written == 4, "failed to write four bytes")
    }

    fn reflect_f64(&mut self, data: &mut f64) -> std::io::Result<()> {
        let bytes_written = self.stream.write(&data.to_be_bytes())?;
        check(bytes_written == 8, "failed to write eight bytes")
    }

    fn reflect_cp866_string(&mut self, string: &mut String) -> std::io::Result<()> {
        let mut size = string.len() as u8;
        self.reflect_u8(&mut size)?;
        for ch in string.chars() {
            let mut chr_id= if ch.is_ascii() {
                ch as u8
            } else {
                b' '
            };
            self.reflect_u8(&mut chr_id)?;
        };
        Ok(())
    }

    fn reflect_cp866_zstring(&mut self, string: &mut String) -> std::io::Result<()> {
        for ch in string.chars() {
            let mut chr_id= if ch.is_ascii() {
                ch as u8
            } else {
                b' '
            };
            self.reflect_u8(&mut chr_id)?;
        };
        let mut zero = 0;
        self.reflect_u8(&mut zero)
    }

    fn reflect_cp866_zstring_ext(&mut self, length: usize, string: &mut String) -> std::io::Result<()> {
        for ch in string.chars() {
            let mut chr_id= if ch.is_ascii() {
                ch as u8
            } else {
                b' '
            };
            self.reflect_u8(&mut chr_id)?;
        };
        let mut zero = 0;
        self.reflect_u8(&mut zero)
    }
}

impl<'a, TStream: Write> SerializationReflector for BinaryWriterLittleEndian<'a, TStream> {
    fn reflect_u8(&mut self, data: &mut u8) -> std::io::Result<()> {
        let bytes_written = self.stream.write(&data.to_le_bytes())?;
        check(bytes_written == 1, "failed to write one byte")
    }

    fn reflect_u16(&mut self, data: &mut u16) -> std::io::Result<()> {
        let bytes_written = self.stream.write(&data.to_le_bytes())?;
        check(bytes_written == 2, "failed to write two bytes")
    }

    fn reflect_u32(&mut self, data: &mut u32) -> std::io::Result<()> {
        let bytes_written = self.stream.write(&data.to_le_bytes())?;
        check(bytes_written == 4, "failed to write four bytes")
    }

    fn reflect_u64(&mut self, data: &mut u64) -> std::io::Result<()> {
        let bytes_written = self.stream.write(&data.to_le_bytes())?;
        check(bytes_written == 8, "failed to write eight bytes")
    }

    fn reflect_i8(&mut self, data: &mut i8) -> std::io::Result<()> {
        let bytes_written = self.stream.write(&data.to_le_bytes())?;
        check(bytes_written == 1, "failed to write one byte")
    }

    fn reflect_i16(&mut self, data: &mut i16) -> std::io::Result<()> {
        let bytes_written = self.stream.write(&data.to_le_bytes())?;
        check(bytes_written == 2, "failed to write two bytes")
    }

    fn reflect_i32(&mut self, data: &mut i32) -> std::io::Result<()> {
        let bytes_written = self.stream.write(&data.to_le_bytes())?;
        check(bytes_written == 4, "failed to write four bytes")
    }

    fn reflect_i64(&mut self, data: &mut i64) -> std::io::Result<()>{
        let bytes_written = self.stream.write(&data.to_le_bytes())?;
        check(bytes_written == 8, "failed to write eight bytes")
    }

    fn reflect_f32(&mut self, data: &mut f32) -> std::io::Result<()> {
        let bytes_written = self.stream.write(&data.to_le_bytes())?;
        check(bytes_written == 4, "failed to write four bytes")
    }

    fn reflect_f64(&mut self, data: &mut f64) -> std::io::Result<()> {
        let bytes_written = self.stream.write(&data.to_le_bytes())?;
        check(bytes_written == 8, "failed to write eight bytes")
    }

    fn reflect_cp866_string(&mut self, string: &mut String) -> std::io::Result<()> {
        let mut size = string.len() as u8;
        self.reflect_u8(&mut size)?;
        for ch in string.chars() {
            let mut chr_id= if ch.is_ascii() {
                ch as u8
            } else {
                b' '
            };
            self.reflect_u8(&mut chr_id)?;
        };
        Ok(())
    }

    fn reflect_cp866_zstring(&mut self, string: &mut String) -> std::io::Result<()> {
        for ch in string.chars() {
            let mut chr_id= if ch.is_ascii() {
                ch as u8
            } else {
                b' '
            };
            self.reflect_u8(&mut chr_id)?;
        };
        let mut zero = 0;
        self.reflect_u8(&mut zero)
    }

    fn reflect_cp866_zstring_ext(&mut self, length: usize, string: &mut String) -> std::io::Result<()> {
        for ch in string.chars() {
            let mut chr_id= if ch.is_ascii() {
                ch as u8
            } else {
                b' '
            };
            self.reflect_u8(&mut chr_id)?;
        };
        let mut zero = 0;
        self.reflect_u8(&mut zero)
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
        let d = &mut [0; 1];
        let size_read = self.stream.read(d)?;
        *data = u8::from_be_bytes(*d);
        check(size_read == 1, "failed to read one byte")
    }

    fn reflect_u16(&mut self, data: &mut u16) -> std::io::Result<()> {
        let d = &mut [0; 2];
        let size_read = self.stream.read(d)?;
        *data = u16::from_be_bytes(*d);
        check(size_read == 2, "failed to read two bytes")
    }

    fn reflect_u32(&mut self, data: &mut u32) -> std::io::Result<()> {
        let d = &mut [0; 4];
        let size_read = self.stream.read(d)?;
        *data = u32::from_be_bytes(*d);
        check(size_read == 4, "failed to read four bytes")
    }

    fn reflect_u64(&mut self, data: &mut u64) -> std::io::Result<()> {
        let d = &mut [0; 8];
        let size_read = self.stream.read(d)?;
        *data = u64::from_be_bytes(*d);
        check(size_read == 8, "failed to read eight bytes")
    }

    fn reflect_i8(&mut self, data: &mut i8) -> std::io::Result<()> {
        let d = &mut [0; 1];
        let size_read = self.stream.read(d)?;
        *data = i8::from_be_bytes(*d);
        check(size_read == 1, "failed to read one byte")
    }

    fn reflect_i16(&mut self, data: &mut i16) -> std::io::Result<()> {
        let d = &mut [0; 2];
        let size_read = self.stream.read(d)?;
        *data = i16::from_be_bytes(*d);
        check(size_read == 2, "failed to read two bytes")
    }

    fn reflect_i32(&mut self, data: &mut i32) -> std::io::Result<()> {
        let d = &mut [0; 4];
        let size_read = self.stream.read(d)?;
        *data = i32::from_be_bytes(*d);
        check(size_read == 4, "failed to read four bytes")
    }

    fn reflect_i64(&mut self, data: &mut i64) -> std::io::Result<()> {
        let d = &mut [0; 8];
        let size_read = self.stream.read(d)?;
        *data = i64::from_be_bytes(*d);
        check(size_read == 8, "failed to read eight bytes")
    }

    fn reflect_f32(&mut self, data: &mut f32) -> std::io::Result<()> {
        let d = &mut [0; 4];
        let size_read = self.stream.read(d)?;
        *data = f32::from_be_bytes(*d);
        check(size_read == 4, "failed to read four bytes")
    }

    fn reflect_f64(&mut self, data: &mut f64) -> std::io::Result<()> {
        let d = &mut [0; 8];
        let size_read = self.stream.read(d)?;
        *data = f64::from_be_bytes(*d);
        check(size_read == 8, "failed to read eight bytes")
    }

    fn reflect_cp866_string(&mut self, string: &mut String) -> std::io::Result<()> {
        let mut size = string.len() as u8;
        self.reflect_u8(&mut size)?;
        string.clear();
        for _ in 0..size {
            let mut chr_id = 0u8;
            self.reflect_u8(&mut chr_id)?;
            string.push(cp866_rs::decode_byte(chr_id));
        }
        Ok(())
    }

    fn reflect_cp866_zstring(&mut self, string: &mut String) -> std::io::Result<()> {
        let mut chr_id = 255;
        string.clear();
        while chr_id != 0 {
            self.reflect_u8(&mut chr_id)?;
            string.push(cp866_rs::decode_byte(chr_id));
        }
        Ok(())
    }

    fn reflect_cp866_zstring_ext(&mut self, length: usize, string: &mut String) -> std::io::Result<()> {
        let mut chr_id = 255;
        string.clear();
        let mut offset = 0;
        let mut terminator_found = false;
        for _ in 0..offset {
            self.reflect_u8(&mut chr_id)?;
            if chr_id == 0 { terminator_found = true; }
            if !terminator_found {
                string.push(cp866_rs::decode_byte(chr_id));
            }
        }
        Ok(())
    }
}

impl<'a, TStream: Read> SerializationReflector for BinaryReaderLittleEndian<'a, TStream> {
    fn reflect_u8(&mut self, data: &mut u8) -> std::io::Result<()> {
        let d = &mut [0; 1];
        let size_read = self.stream.read(d)?;
        *data = u8::from_le_bytes(*d);
        check(size_read == 1, "failed to read one byte")
    }

    fn reflect_u16(&mut self, data: &mut u16) -> std::io::Result<()> {
        let d = &mut [0; 2];
        let size_read = self.stream.read(d)?;
        *data = u16::from_le_bytes(*d);
        check(size_read == 2, "failed to read two bytes")
    }

    fn reflect_u32(&mut self, data: &mut u32) -> std::io::Result<()> {
        let d = &mut [0; 4];
        let size_read = self.stream.read(d)?;
        *data = u32::from_le_bytes(*d);
        check(size_read == 4, "failed to read four bytes")
    }

    fn reflect_u64(&mut self, data: &mut u64) -> std::io::Result<()> {
        let d = &mut [0; 8];
        let size_read = self.stream.read(d)?;
        *data = u64::from_le_bytes(*d);
        check(size_read == 8, "failed to read eight bytes")
    }

    fn reflect_i8(&mut self, data: &mut i8) -> std::io::Result<()> {
        let d = &mut [0; 1];
        let size_read = self.stream.read(d)?;
        *data = i8::from_le_bytes(*d);
        check(size_read == 1, "failed to read one byte")
    }

    fn reflect_i16(&mut self, data: &mut i16) -> std::io::Result<()> {
        let d = &mut [0; 2];
        let size_read = self.stream.read(d)?;
        *data = i16::from_le_bytes(*d);
        check(size_read == 2, "failed to read two bytes")
    }

    fn reflect_i32(&mut self, data: &mut i32) -> std::io::Result<()> {
        let d = &mut [0; 4];
        let size_read = self.stream.read(d)?;
        *data = i32::from_le_bytes(*d);
        check(size_read == 4, "failed to read four bytes")
    }

    fn reflect_i64(&mut self, data: &mut i64) -> std::io::Result<()> {
        let d = &mut [0; 8];
        let size_read = self.stream.read(d)?;
        *data = i64::from_le_bytes(*d);
        check(size_read == 8, "failed to read eight bytes")
    }

    fn reflect_f32(&mut self, data: &mut f32) -> std::io::Result<()> {
        let d = &mut [0; 4];
        let size_read = self.stream.read(d)?;
        *data = f32::from_le_bytes(*d);
        check(size_read == 4, "failed to read four bytes")
    }

    fn reflect_f64(&mut self, data: &mut f64) -> std::io::Result<()> {
        let d = &mut [0; 8];
        let size_read = self.stream.read(d)?;
        *data = f64::from_le_bytes(*d);
        check(size_read == 8, "failed to read eight bytes")
    }

    fn reflect_cp866_string(&mut self, string: &mut String) -> std::io::Result<()> {
        let mut size = string.len() as u8;
        self.reflect_u8(&mut size)?;
        string.clear();
        for _ in 0..size {
            let mut chr_id = 0u8;
            self.reflect_u8(&mut chr_id)?;
            string.push(cp866_rs::decode_byte(chr_id));
        }
        Ok(())
    }

    fn reflect_cp866_zstring(&mut self, string: &mut String) -> std::io::Result<()> {
        let mut chr_id = 255;
        string.clear();
        while chr_id != 0 {
            self.reflect_u8(&mut chr_id)?;
            string.push(cp866_rs::decode_byte(chr_id));
        }
        Ok(())
    }

    fn reflect_cp866_zstring_ext(&mut self, length: usize, string: &mut String) -> std::io::Result<()> {
        let mut chr_id = 255;
        string.clear();
        let mut offset = 0;
        let mut terminator_found = false;
        for _ in 0..offset {
            self.reflect_u8(&mut chr_id)?;
            if chr_id == 0 { terminator_found = true; }
            if !terminator_found {
                string.push(cp866_rs::decode_byte(chr_id));
            }
        }
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
    fn test_incorrect_length() {
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