/// StackVec was created to hide every array operations in one file
/// It is briefly tested and very poor, but I believe it is temporary solution,
/// because after adding heap, we could probably write our allocator
/// to use/reuse/adapt the existing Vector or probably Linked lists or something else
/// that requires memory allocation

/// 5 days after writing previous comment block:
/// The shit should be 100% rewritten
#[derive(Debug, Copy, Clone)]
pub struct StackVec<T, const N: usize> {
    data: [T; N],
    default: T,
    len: usize,
}

#[allow(dead_code)]
pub enum IteratorType {
    Len,
    Capacity,
}

#[allow(dead_code)]
impl<T: Copy, const N: usize> StackVec<T, N> {
    pub fn new(default: T) -> Self {
        Self {
            data: [default; N],
            default,
            len: 0,
        }
    }

    pub fn push(&mut self, value: T) -> bool {
        if self.len < self.capacity() {
            self.data[self.len] = value;
            self.len += 1;
            true
        } else {
            false
        }
    }

    pub fn push_at(&mut self, index: usize, value: T) -> bool {
        if index <= self.len && self.len < self.capacity() {
            for i in (index..self.len).rev() {
                self.data[i + 1] = self.data[i];
            }
            self.data[index] = value;
            self.len += 1;
            true
        } else {
            false
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len > 0 {
            let value = self.data[self.len - 1];
            self.data[self.len - 1] = self.default;
            self.len -= 1;
            Some(value)
        } else {
            None
        }
    }

    pub fn pop_at(&mut self, index: usize) -> Option<T> {
        if index < self.len {
            let value = self.data[index];
            for i in index..self.len - 1 {
                self.data[i] = self.data[i + 1];
            }
            self.data[self.len - 1] = self.default;
            self.len -= 1;
            Some(value)
        } else {
            None
        }
    }

    pub fn set_at(&mut self, index: usize, value: T) -> bool {
        if index < self.capacity() {
            self.data[index] = value;
            if index >= self.len {
                self.len = index + 1;
            }
            true
        } else {
            false
        }
    }

    pub fn data(&self) -> &[T] {
        &self.data[..self.len]
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            Some(&self.data[index])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.len {
            Some(&mut self.data[index])
        } else {
            None
        }
    }

    pub fn slice(&self, range: core::ops::Range<usize>) -> &[T] {
        &self.data[range.start..range.end]
    }

    pub fn get_unsafe(&self, index: usize) -> &T {
        &self.data[index]
    }

    pub fn get_mut_unsafe(&mut self, index: usize) -> &mut T {
        &mut self.data[index]
    }

    pub fn clear(&mut self) {
        while let Some(_) = self.pop() {}
    }

    pub fn default(&self) -> T {
        self.default
    }

    pub fn iter(&self, it: IteratorType) -> core::slice::Iter<'_, T> {
        match it {
            IteratorType::Len => self.data[..self.len].iter(),
            IteratorType::Capacity => self.data[..self.capacity()].iter(),
        }
    }

    pub fn iter_mut(&mut self, it: IteratorType) -> core::slice::IterMut<'_, T> {
        let capacity = self.capacity();
        match it {
            IteratorType::Len => self.data[..self.len].iter_mut(),
            IteratorType::Capacity => self.data[..capacity].iter_mut(),
        }
    }

    pub fn capacity(&self) -> usize {
        N
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn copy(&self) -> ([T; N], usize) {
        (self.data, self.len)
    }

    pub fn copy_from(&mut self, data: &[T; N], len: usize) {
        for i in 0..len {
            self.data[i] = data[i];
        }

        for i in len..self.len {
            self.data[i] = self.default;
        }

        self.len = len;
    }
}
