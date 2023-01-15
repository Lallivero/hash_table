use crate::Hashable;

impl Hashable for usize {
    fn hash(&self) -> usize {
        *self as usize
    }
}
impl Hashable for u8 {
    fn hash(&self) -> usize {
        *self as usize
    }
}

impl Hashable for u16 {
    fn hash(&self) -> usize {
        *self as usize
    }
}

impl Hashable for u32 {
    fn hash(&self) -> usize {
        *self as usize
    }
}

impl Hashable for u64 {
    fn hash(&self) -> usize {
        *self as usize
    }
}

impl Hashable for u128 {
    fn hash(&self) -> usize {
        *self as usize
    }
}

impl Hashable for i8 {
    fn hash(&self) -> usize {
        *self as usize
    }
}

impl Hashable for i16 {
    fn hash(&self) -> usize {
        *self as usize
    }
}

impl Hashable for i32 {
    fn hash(&self) -> usize {
        *self as usize
    }
}

impl Hashable for i64 {
    fn hash(&self) -> usize {
        *self as usize
    }
}

impl Hashable for i128 {
    fn hash(&self) -> usize {
        *self as usize
    }
}

impl Hashable for f32 {
    fn hash(&self) -> usize {
        *self as usize
    }
}

impl Hashable for f64 {
    fn hash(&self) -> usize {
        *self as usize
    }
}

impl Hashable for &str {
    fn hash(&self) -> usize {
        hash_string_like_types(self)
    }
}
impl Hashable for str {
    fn hash(&self) -> usize {
        hash_string_like_types(self)
    }
}
impl Hashable for String {
    fn hash(&self) -> usize {
        hash_string_like_types(self)
    }
}

fn hash_string_like_types(input: &str) -> usize {
    let mut sum = 0;
    input.chars().for_each(|c| sum += c as usize);
    sum
}
