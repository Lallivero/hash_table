use std::fmt::Debug;
use std::fmt::Display;

#[derive(Debug)]
pub struct HashTable<T>
where
    T: Debug + Hashable + Clone + PartialEq,
{
    capacity: usize,
    growable: bool,
    cells: Vec<Option<T>>,
}

impl<T> Display for HashTable<T>
where
    T: Debug + Hashable + Clone + PartialEq,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[").expect("Error formatting.");
        for i in 0..self.cells.len() {
            if i == self.cells.len() - 1 {
                write!(f, "{:?} - {:?}]", i, self.cells[i]).expect("Error formatting.");
            } else {
                writeln!(f, "{:?} - {:?},", i, self.cells[i]).expect("Error formatting.");
            }
        }

        Ok(())
    }
}

//Public functions
impl<T> HashTable<T>
where
    T: Debug + Hashable + Clone + PartialEq,
{
    pub fn new(capacity: usize, growable: bool) -> Self {
        Self {
            capacity,
            growable,
            cells: vec![None; capacity],
        }
    }

    pub fn is_empty(&self) -> bool {
        let num_none = self.cells.iter().filter(|cell| cell.is_none()).count();
        num_none == self.capacity
    }

    pub fn insert(&mut self, input: T) -> Result<(), &str> {
        let input_hash = hash_function(&input, self.capacity);

        if self.size() == self.capacity {
            if self.growable {
                self.grow_cells().expect("Error growing cells.")
            } else {
                return Err("Capacity reached");
            }
        }

        match self
            .cells
            .get(input_hash)
            .expect("Unexpected error occured, found value None where Some(_) was expected.")
        {
            Some(_) => self.insert_next_index(input, input_hash),
            None => {
                self.cells[input_hash] = Some(input);

                Ok(())
            }
        }
    }

    pub fn get_index(&self, index: usize) -> Option<&T> {
        match self
            .cells
            .get(index)
            .expect("Expected Some(_) but found None")
        {
            Some(s) => Some(s),
            None => None,
        }
    }

    pub fn get(&self, input: &T) -> Option<&T> {
        let hash_index = hash_function(input, self.capacity);
        match self
            .cells
            .get(hash_index)
            .expect("Unexpected error occured, found value None where Some(_) was expected.")
        {
            Some(s) => {
                if s == input {
                    Some(s)
                } else {
                    self.get_next(input, hash_index)
                }
            }
            None => None,
        }
    }

    pub fn size(&self) -> usize {
        self.cells.iter().filter(|cell| cell.is_some()).count()
    }

    pub fn remove(&mut self, input: T) -> Option<T> {
        let hash_index = hash_function(&input, self.capacity);
        match self
            .cells
            .get(hash_index)
            .expect("Unexpected error occured, found value None where Some(_) was expected.")
        {
            Some(s) => {
                if *s == input {
                    self.cells.remove(hash_index)
                } else {
                    self.remove_next(input, hash_index)
                }
            }
            None => None,
        }
    }
}

//Private helper methods
impl<T> HashTable<T>
where
    T: Debug + Hashable + Clone + PartialEq,
{
    fn grow_cells(&mut self) -> Result<(), &str> {
        let new_capacity = self.capacity * 2;
        let new_cells: Vec<Option<T>> = vec![None; new_capacity];
        let old_cells = self.cells.clone();
        self.cells = new_cells;
        self.capacity = new_capacity;

        for i in 0..old_cells.len() {
            match old_cells.get(i).expect("Unexpected error.") {
                Some(s) => self.insert(s.clone()).expect("Insertion error"),
                None => continue,
            }
        }

        Ok(())
    }

    fn insert_next_index(&mut self, input: T, previous_index: usize) -> Result<(), &str> {
        let next_index = self.calculate_next_index_for_reccursion(previous_index);

        match self
            .cells
            .get(next_index)
            .expect("Unexpected error occured, found value None where Some(_) was expected.")
        {
            Some(_) => Ok(self.insert_next_index(input, next_index)?),
            None => {
                self.cells[next_index] = Some(input);
                Ok(())
            }
        }
    }

    fn get_next(&self, input: &T, previous_index: usize) -> Option<&T> {
        let next_index = self.calculate_next_index_for_reccursion(previous_index);

        if next_index == hash_function(input, self.capacity) {
            return None;
        }

        match self
            .cells
            .get(next_index)
            .expect("Unexpected error occured, found value None where Some(_) was expected.")
        {
            Some(s) => {
                if s == input {
                    Some(s)
                } else {
                    self.get_next(input, next_index)
                }
            }
            None => self.get_next(input, next_index),
        }
    }

    fn remove_next(&mut self, input: T, previous_index: usize) -> Option<T> {
        let next_index = self.calculate_next_index_for_reccursion(previous_index);

        if next_index == hash_function(&input, self.capacity) {
            return None;
        }

        match self
            .cells
            .get(next_index)
            .expect("Unexpected error occured, found value None where Some(_) was expected.")
        {
            Some(a) => {
                if *a == input {
                    self.cells.remove(next_index)
                } else {
                    self.remove_next(input, next_index)
                }
            }
            None => self.remove_next(input, next_index),
        }
    }

    fn calculate_next_index_for_reccursion(&self, previous_index: usize) -> usize {
        let next_index = previous_index + 1;
        if next_index == self.capacity {
            0
        } else {
            next_index
        }
    }
}

fn hash_function<T>(input: &T, table_size: usize) -> usize
where
    T: Hashable + ?Sized,
{
    input.hash() % table_size
    // sum_ascii_values(input_string) as usize % table_size
}

#[cfg(test)]
mod tests {

    use crate::*;
    const SIZE_10: usize = 10;
    const TEST_STR_1: &str = "abcd";
    const TEST_STR_2: &str = "efgh";

    // a = 7
    // k = 7
    const COLLISION_STR_1: &str = "a";
    const COLLISION_STR_2: &str = "k";

    #[test]
    fn test_hash_function_within_index() {
        let upper_bound = 10;
        let hash_code = hash_function("abcdefghijklmnopqrstuvwxyz", upper_bound);
        assert!(hash_code < upper_bound);
    }

    #[test]
    fn test_new_hash_table() {
        let test_hash_table: HashTable<String> = HashTable::new(SIZE_10, false);
        assert!(test_hash_table.is_empty());
    }

    #[test]
    fn test_size() {
        let mut test_hash_table: HashTable<String> = HashTable::new(SIZE_10, false);
        test_hash_table.insert(TEST_STR_1.to_owned()).unwrap();
        test_hash_table.insert(TEST_STR_2.to_owned()).unwrap();
        assert!(test_hash_table.size() == 2);
    }

    #[test]
    fn test_insert() {
        let mut test_hash_table: HashTable<String> = HashTable::new(SIZE_10, false);
        test_hash_table
            .insert(TEST_STR_1.to_owned())
            .expect("oopsie");
        assert!(!test_hash_table.is_empty());
    }

    #[test]
    fn test_insert_collision() {
        let mut test_hash_table: HashTable<String> = HashTable::new(SIZE_10, false);
        test_hash_table
            .insert(COLLISION_STR_1.to_owned())
            .expect("oopsie");
        test_hash_table
            .insert(COLLISION_STR_2.to_owned())
            .expect("oopsie");
        assert!(test_hash_table.size() == 2);
    }
    #[test]
    fn test_get_by_index() {
        let mut test_hash_table: HashTable<String> = HashTable::new(SIZE_10, false);
        test_hash_table
            .insert(TEST_STR_1.to_owned())
            .expect("oopsie");
        assert!(
            test_hash_table.get_index(hash_function(TEST_STR_1, SIZE_10))
                == Some(&TEST_STR_1.to_string())
        );
    }

    #[test]
    fn test_get_by_string() {
        let mut test_hash_table: HashTable<String> = HashTable::new(SIZE_10, false);
        test_hash_table
            .insert(COLLISION_STR_1.to_owned())
            .expect("Insertion error.");
        test_hash_table
            .insert(COLLISION_STR_2.to_owned())
            .expect("Insertion error.");

        assert!(test_hash_table.get(&COLLISION_STR_2.to_owned()).unwrap() == COLLISION_STR_2);
    }

    #[test]
    fn test_remove() {
        let mut test_hash_table: HashTable<String> = HashTable::new(SIZE_10, false);
        test_hash_table
            .insert(TEST_STR_1.to_owned())
            .expect("failed to insert");
        test_hash_table
            .insert(TEST_STR_2.to_owned())
            .expect("failed to insert");

        let removed_string = test_hash_table.remove(TEST_STR_1.to_owned());
        assert!(removed_string.unwrap() == *TEST_STR_1);
    }

    #[test]
    fn test_remove_with_collision() {
        let mut test_hash_table: HashTable<String> = HashTable::new(SIZE_10, false);
        test_hash_table
            .insert(COLLISION_STR_1.to_owned())
            .expect("failed to insert");
        test_hash_table
            .insert(COLLISION_STR_2.to_owned())
            .expect("failed to insert");

        let removed_string = test_hash_table.remove(COLLISION_STR_2.to_owned());
        assert!(removed_string.unwrap() == *COLLISION_STR_2);
    }

    #[test]
    fn test_remove_object_not_in_table() {
        let mut test_hash_table: HashTable<String> = HashTable::new(SIZE_10, false);
        assert!(test_hash_table.remove(TEST_STR_1.to_owned()) == None);
    }

    #[test]
    fn test_growing_capacity_growable() {
        let mut test_hash_table: HashTable<String> = HashTable::new(SIZE_10, true);
        // test_hash_table.grow_cells();
        for c in 'a'..='z' {
            test_hash_table
                .insert(c.to_string())
                .expect("unexpected insertion error");
        }
        //With 26 being the number of letters a-z
        assert!(test_hash_table.size() == 26);
    }

    #[test]
    fn test_growing_capacity_not_growable() {
        let mut test_hash_table: HashTable<String> = HashTable::new(SIZE_10, false);
        // test_hash_table.grow_cells();
        for c in 'a'..='z' {
            match test_hash_table.insert(c.to_string()) {
                Ok(_) => continue,
                Err(msg) => {
                    println!("{}", msg);
                    break;
                }
            }
        }
        //With 26 being the number of letters a-z
        assert!(test_hash_table.size() == 10);
    }
}

pub trait Hashable {
    fn hash(&self) -> usize;
}

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
        let mut sum = 0;
        self.chars().for_each(|c| sum += c as usize);
        sum
    }
}
impl Hashable for str {
    fn hash(&self) -> usize {
        let mut sum = 0;
        self.chars().for_each(|c| sum += c as usize);
        sum
    }
}
impl Hashable for String {
    fn hash(&self) -> usize {
        let mut sum = 0;
        self.chars().for_each(|c| sum += c as usize);
        sum
    }
}
