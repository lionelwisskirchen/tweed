use crate::core::error::ExecutionError;
use alloc::vec::Vec;
pub type Result<T> = core::result::Result<T, ExecutionError>;

/// A fixed-capacity stack.
///
/// `BaseStack<T, N>` stores up to `N` elements of type `T` on the stack,
/// with *no* heap allocation. It mirrors the interface of [`Vec`] for common
/// operations but panics on overflow instead of reallocating.
///
/// # Type Parameters
/// - `T` — Element type. Must implement [`Default`] (used to initialize the
///   backing array) and [`Clone`] (used by [`pop`](BaseStack::pop) and
///   [`split_off`](BaseStack::split_off)).
/// - `N` — Maximum number of elements. Determined at compile time.
///
/// # Example
/// ```
/// use tweed::core::custom_types::base_stack::BaseStack;
/// let mut stack: BaseStack<i32, 8> = BaseStack::new();
/// stack.push(1);
/// stack.push(2);
/// assert_eq!(stack.pop(), Some(2));
/// assert_eq!(stack.len(), 1);
/// ```
#[derive(Clone, Debug)]
pub struct BaseStack<T: Default + Clone, const N: usize> {
    data: Box<[T; N]>,
    top: usize,
}

impl<T: Default + Clone, const N: usize> BaseStack<T, N> {
    /// Creates an empty `BaseStack`, initializing all slots to `T::default()`.
    pub fn new() -> Self {
        Self {
            data: Box::new(core::array::from_fn(|_| T::default())),
            top: 0,
        }
    }

    /// Pushes `val` onto the top of the stack.
    ///
    /// # Arguments
    /// * `val`: The value to push onto the stack.
    ///
    /// # Panics
    /// Panics if the stack is already at capacity (`len() == N`).
    pub fn push(&mut self, val: T) -> Result<()> {
        match self.data.get_mut(self.top) {
            Some(x) => {
                *x = val;
                self.top += 1;
            }
            None => return Err(ExecutionError::StackExhaustion),
        }
        Ok(())
    }

    /// Removes and returns the top element, or `None` if the stack is empty.
    pub fn pop(&mut self) -> Option<T> {
        if self.top == 0 {
            return None;
        }
        self.top -= 1;
        Some(self.data[self.top].clone())
    }

    /// Returns the number of elements currently on the stack.
    pub fn len(&self) -> usize {
        self.top
    }

    /// Removes all elements from the stack.
    ///
    /// The backing array is not zeroed; existing values remain in memory
    /// until overwritten by a subsequent [`push`](BaseStack::push).
    pub fn clear(&mut self) {
        self.top = 0;
    }

    /// Returns `true` if the stack contains no elements.
    pub fn is_empty(&self) -> bool {
        self.top == 0
    }

    /// Truncates the stack, keeping only elements before `range.start`.
    ///
    /// Equivalent to `self.top = range.start`. Elements beyond that point
    /// are not dropped but become inaccessible.
    pub fn drain(&mut self, range: core::ops::RangeFrom<usize>) {
        self.top = range.start;
    }

    /// Splits the stack at `at`, returning the elements in `at..len` as a `Vec`.
    ///
    /// After the call, `self` contains only the elements in `0..at`.
    /// The returned [`Vec`] holds the removed elements in bottom-to-top order.
    ///
    /// # Panics
    /// Panics if `at > self.len()`, mirroring the behaviour of [`Vec::split_off`].
    ///
    pub fn split_off(&mut self, at: usize) -> Vec<T> {
        // this error is the same as in Vec
        if self.top < at {
            panic!(
                "'at' split index (is {}) should be <= len (is {})",
                at, self.top
            );
        }
        let r: Vec<T> = self.data[at..self.top].to_vec();
        self.top = at;
        r
    }
}

impl<T: Default + Clone + Copy, const N: usize> Default for BaseStack<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_stack_push_pop() {
        let mut stack: BaseStack<i32, 100> = BaseStack::new();
        let mut v: Vec<i32> = Vec::new();

        for i in 0..100 {
            stack.push(i).unwrap();
            v.push(i);
        }

        for _ in 0..100 {
            assert_eq!(stack.pop().unwrap(), v.pop().unwrap());
        }
    }

    #[test]
    fn test_base_stack_len() {
        let mut stack: BaseStack<i32, 100> = BaseStack::new();
        let mut v: Vec<i32> = Vec::new();

        for i in 0..100 {
            stack.push(i).unwrap();
            v.push(i);
            assert_eq!(stack.len(), v.len())
        }
    }

    #[test]
    fn test_base_stack_clear() {
        let mut stack: BaseStack<i32, 100> = BaseStack::new();
        let mut v: Vec<i32> = Vec::new();

        for i in 0..100 {
            stack.push(i).unwrap();
            v.push(i);
        }

        stack.clear();
        v.clear();

        assert_eq!(stack.len(), v.len());
        assert!(stack.is_empty());
        assert!(v.is_empty());
    }

    #[test]
    fn test_base_stack_drain() {
        let mut stack: BaseStack<i32, 100> = BaseStack::new();
        let mut v: Vec<i32> = Vec::new();

        for i in 0..100 {
            stack.push(i).unwrap();
            v.push(i);
        }

        v.drain(50..);
        stack.drain(50..);

        for _ in 0..50 {
            assert_eq!(stack.pop().unwrap(), v.pop().unwrap());
        }
    }

    #[test]
    fn test_base_stack_split_off() {
        let mut stack: BaseStack<i32, 100> = BaseStack::new();
        let mut v: Vec<i32> = Vec::new();

        for i in 0..100 {
            stack.push(i).unwrap();
            v.push(i);
        }

        let r1 = v.split_off(50);
        let r2 = stack.split_off(50);

        println!("{:?}", r1);
        println!("{:?}", r2);

        assert_eq!(r1, r2);
        assert_eq!(stack.len(), v.len());
    }
}
