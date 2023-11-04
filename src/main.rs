mod fifo {
    struct Fifo {
        size: usize,
        capacity: usize,
        data: Box<[i32]>,
    }

    impl Fifo {
        fn new() -> Self {
            Self {
                size: 0,
                capacity: 1,
                data: Box::new([0]),
            }
        }

        fn size(&self) -> usize {
            self.size
        }

        #[cfg(test)]
        fn capacity(&self) -> usize {
            self.capacity
        }

        fn push(&mut self, value: i32) {
            if self.size == self.capacity {
                self.capacity *= 2;
                let mut new_data = vec![0; self.capacity].into_boxed_slice();
                for i in 0..self.size {
                    new_data[i] = self.data[i];
                }
                self.data = new_data;
            }

            self.data[self.size] = value;
            self.size += 1;
        }

        fn pop(&mut self) -> i32 {
            let ret = self.data[0];
            for i in 1..self.size {
                self.data[i - 1] = self.data[i];
            }
            self.size -= 1;
            ret
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn size_increments_capacity_doubles() {
            let mut fifo = Fifo::new();
            assert_eq!(0, fifo.size());
            assert_eq!(1, fifo.capacity());

            fifo.push(5);
            assert_eq!(1, fifo.size());
            assert_eq!(1, fifo.capacity());

            fifo.push(5);
            assert_eq!(2, fifo.size());
            assert_eq!(2, fifo.capacity());

            fifo.push(5);
            assert_eq!(3, fifo.size());
            assert_eq!(4, fifo.capacity());

            fifo.push(5);
            assert_eq!(4, fifo.size());
            assert_eq!(4, fifo.capacity());

            fifo.push(5);
            assert_eq!(5, fifo.size());
            assert_eq!(8, fifo.capacity());
        }

        #[test]
        fn push_pop() {
            let mut fifo = Fifo::new();
            fifo.push(5);
            assert_eq!(5, fifo.pop());
            assert_eq!(0, fifo.size())
        }
    }
}

fn main() {
    println!("Hello, world!");
}
