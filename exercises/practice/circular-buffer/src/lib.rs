//use std::marker::PhantomData;

pub struct CircularBuffer<T> {
    // This field is here to make the template compile and not to
    // complain about unused type parameter 'T'. Once you start
    // solving the exercise, delete this field and the 'std::marker::PhantomData'
    // import.
    //field: PhantomData<T>,
    data: Vec<Option<T>>,
    read_index: usize,
    write_index: usize,

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
            read_index: 0,
            write_index: 0
        }
    }

    pub fn write(&mut self, _element: T) -> Result<(), Error> {
        if self.data[self.write_index].is_some() {
            return Err(Error::FullBuffer)
        }
        self.data[self.write_index] = Some(_element);
        self.write_index = self.increase_index(self.write_index);
        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        self.data[self.read_index].take().ok_or(Error::EmptyBuffer)
            .map(|value|{
                self.read_index = self.increase_index(self.read_index);
                value
            })
    }

    pub fn clear(&mut self) {
        self.read_index = 0;
        self.write_index = 0;
        self.data.fill(None);
    }

    pub fn overwrite(&mut self, _element: T) {
        if self.data[self.write_index].is_some() {
            self.read_index = self.increase_index(self.read_index);
        }
        self.data[self.write_index] = Some(_element);
        self.write_index = self.increase_index(self.write_index);
    }

    fn increase_index(&mut self, index: usize) -> usize {
        (index + 1) % (self.data.capacity())
    }

}
