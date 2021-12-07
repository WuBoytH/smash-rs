use std::ops::{Index, IndexMut, Range};

#[repr(C)]
pub struct Vector<T> {
    start: *mut T,
    end: *mut T,
    eos: *mut T
}

impl<T> Vector<T> {
    pub fn capacity(&self) -> usize {
        if self.start.is_null() || self.eos.is_null() { 0 }
        else {
            unsafe {
                self.eos.offset_from(self.start) as usize
            }
        }
    }

    pub fn len(&self) -> usize {
        if self.start.is_null() || self.end.is_null() { 0 }
        else {
            unsafe {
                self.end.offset_from(self.start) as usize
            }
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.as_slice()
            .map(|x| x.get(index))
            .flatten()
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.as_mut_slice()
            .map(|x| x.get_mut(index))
            .flatten()
    }

    pub fn as_ptr(&self) -> *const T {
        self.start
    }

    pub fn as_mut_ptr(&self) -> *mut T {
        self.start
    }

    pub fn as_slice(&self) -> Option<&[T]> {
        if self.start.is_null() {
            None
        } else {
            unsafe {
                Some(std::slice::from_raw_parts(self.start, self.len()))
            }
        }
    }

    pub fn as_mut_slice(&mut self) -> Option<&mut [T]> {
        if self.start.is_null() {
            None
        } else {
            unsafe {
                Some(std::slice::from_raw_parts_mut(self.start, self.len()))
            }
        }
    }
}

impl<T> Index<usize> for Vector<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.as_slice().expect("Attempted to index null vector!")[index]
    }
}

impl<T> IndexMut<usize> for Vector<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut_slice().expect("Attempted to index null vector!")[index]
    }
}

impl<T> Index<Range<usize>> for Vector<T> {
    type Output = [T];

    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.as_slice().expect("Attempted to index null vector!")[index]
    }
}

impl<T> IndexMut<Range<usize>> for Vector<T> {
    fn index_mut(&mut self, index: Range<usize>) -> &mut Self::Output {
        &mut self.as_mut_slice().expect("Attempted to index null vector!")[index]
    }
}

pub struct VectorIter<'a, T> {
    vector: &'a Vector<T>,
    index: usize
}

impl<'a, T> Iterator for VectorIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.vector.get(self.index);
        self.index += 1;
        result
    }
}

pub struct VectorIterMut<'a, T> {
    vector: &'a mut Vector<T>,
    index: usize
}

impl<'a, T> Iterator for VectorIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.vector.get_mut(self.index);
        self.index += 1;
        unsafe { // hacky workaround, if you have a better idea please
            std::mem::transmute::<Option<&mut T>, Option<&'a mut T>>(result)
        }
    }
}