use num_traits::FromBytes;

pub struct FastCursor<'a> {
    code: &'a [u8],
    pos: usize,
}

impl<'a> FastCursor<'a> {
    pub fn new(code: &'a [u8]) -> Self {
        Self { code, pos: 0 }
    }

    pub fn set_position(&mut self, pos: usize) {
        self.pos = pos;
    }

    #[inline]
    pub fn get<T, const N: usize>(&mut self) -> T
    where
        T: FromBytes<Bytes = [u8; N]>,
    {
        unsafe {
            let mut addr = self.code as *const [u8];
            addr = addr.byte_add(self.pos);
            let addr = addr as *const [u8; N];
            self.pos += N;
            T::from_le_bytes(&*addr)
        }
    }

    #[inline]
    pub fn get_i64_le(&mut self) -> i64 {
        self.get()
    }

    #[inline]
    pub fn get_u8(&mut self) -> u8 {
        self.get()
    }
}
