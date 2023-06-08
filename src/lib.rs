use std::io;


pub trait ReadTrait {
    fn read_vec(&mut self, size: isize) -> Result<Vec<u8>, std::io::Error>;
}

impl<R: io::Read> ReadTrait for R {
    fn read_vec(&mut self, size: isize) -> Result<Vec<u8>, std::io::Error> {
        let mut vec:Vec<u8> = Vec::new();
        if size < 0 {
            self.read_to_end(&mut vec)?;
            }
        else if size > 0 {
            vec = Vec::with_capacity(size as usize);
            unsafe {vec.set_len(size as usize);}
            let mut buf = &mut vec[..];
            let mut readed = 0;
            while !buf.is_empty() {
                match self.read(buf) {
                    Ok(0) => break,
                    Ok(n) => {
                        readed += n;
                        let tmp = buf;
                        buf = &mut tmp[n..];
                        }
                    Err(e) => return Err(e),
                    }
                }
            unsafe {vec.set_len(readed);}
            }
        return Ok(vec);
    }
}
