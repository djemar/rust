// iterative
pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let mut sliced_array = array;
    let mut len= array.len();
    let mut value : i32;
    let mut index: usize = 0;

    while len > 0 {
        len = sliced_array.len() / 2;
        value = sliced_array[len];
        if value == key {
            index += len;
            return Some(index)
        } else  {
            let (l_array, r_array) = sliced_array.split_at(len);
            if value < key {
                index += len;
                sliced_array = r_array;
            } else { sliced_array = l_array; }
        }
    }

    None
}
