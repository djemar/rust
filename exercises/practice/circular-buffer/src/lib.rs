//use std::marker::PhantomData;

pub struct CircularBuffer<T> {
    // This field is here to make the template compile and not to
    // complain about unused type parameter 'T'. Once you start
    // solving the exercise, delete this field and the 'std::marker::PhantomData'
    // import.
    //field: PhantomData<T>,
    data: Vec<Option<T>>,
    head: usize,
    tail: usize,

}


#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T: Clone> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        CircularBuffer {
            data: vec![None; capacity],
            head: 0,
            tail: 0
        }
    }

    pub fn write(&mut self, _element: T) -> Result<(), Error> {
        if self.data[self.tail].is_some() {
            return Err(Error::FullBuffer)
        }
        self.data[self.tail] = Some(_element);
        self.tail = self.increase_index(self.tail);
        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        self.data[self.head].take().ok_or(Error::EmptyBuffer)
            .map(|value|{
                self.head = self.increase_index(self.head);
                value
            })
    }

    pub fn clear(&mut self) {
        self.head = 0;
        self.tail = 0;
        self.data.fill(None);
    }

    pub fn overwrite(&mut self, _element: T) {
        if self.data[self.tail].is_some() {
            self.head = self.increase_index(self.head);
        }
        self.data[self.tail] = Some(_element);
        self.tail = self.increase_index(self.tail);
    }

    fn increase_index(&mut self, index: usize) -> usize {
        (index + 1) % (self.data.capacity())
    }

}
