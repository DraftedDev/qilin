#[derive(Clone, Debug)]
pub struct Buffer {
    pub(crate) buffer: Vec<u32>,
    pub(crate) width: usize,
    pub(crate) height: usize,
}

impl Buffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            buffer: vec![0; width * height],
            width,
            height,
        }
    }
    pub fn get_width(&self) -> usize { self.width }

    pub fn get_height(&self) -> usize { self.height }

    pub fn as_slice(&self) -> &[u32] { self.buffer.as_slice() }
}

impl IntoIterator for Buffer {
    type Item = u32;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter { self.buffer.into_iter() }
}

#[cfg(feature = "parallel")]
impl rayon::iter::IntoParallelIterator for Buffer {
    type Iter = rayon::vec::IntoIter<Self::Item>;
    type Item = u32;

    fn into_par_iter(self) -> Self::Iter { self.buffer.into_par_iter() }
}
