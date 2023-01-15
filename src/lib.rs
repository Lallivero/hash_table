mod premade_type_hashes;

use std::fmt::Debug;
use std::fmt::Display;

pub trait Hashable {
    fn hash(&self) -> usize;
}

fn hash_function<T>(input: &T, table_size: usize) -> usize
where
    T: Hashable + ?Sized,
{
    input.hash() % table_size
    // sum_ascii_values(input_string) as usize % table_size
}

#[derive(Debug)]
pub struct HashTable<T>
where
    T: Debug + Hashable + Clone + PartialEq,
{
    _capacity: usize,
    _growable: bool,
    _cells: Vec<Option<T>>,
}

impl<T> Display for HashTable<T>
where
    T: Debug + Hashable + Clone + PartialEq,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[").expect("Error formatting.");
        for i in 0..self._cells.len() {
            if i == self._cells.len() - 1 {
                write!(f, "{:?} - {:?}]", i, self._cells[i]).expect("Error formatting.");
            } else {
                writeln!(f, "{:?} - {:?},", i, self._cells[i]).expect("Error formatting.");
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
            _capacity: capacity,
            _growable: growable,
            _cells: vec![None; capacity],
        }
    }

    pub fn is_empty(&self) -> bool {
        let num_none = self._cells.iter().filter(|cell| cell.is_none()).count();
        num_none == self._capacity
    }

    pub fn insert(&mut self, input: T) -> Result<(), &str> {
        let input_hash = hash_function(&input, self._capacity);

        if self.size() == self._capacity {
            if self._growable {
                self.grow_cells().expect("Error growing cells.")
            } else {
                return Err("Capacity reached");
            }
        }

        match self
            ._cells
            .get(input_hash)
            .expect("Unexpected error occured, found value None where Some(_) was expected.")
        {
            Some(_) => self.insert_next_index(input, input_hash),
            None => {
                self._cells[input_hash] = Some(input);

                Ok(())
            }
        }
    }

    pub fn get_index(&self, index: usize) -> Option<&T> {
        match self
            ._cells
            .get(index)
            .expect("Expected Some(_) but found None")
        {
            Some(s) => Some(s),
            None => None,
        }
    }

    pub fn get(&self, input: &T) -> Option<&T> {
        let hash_index = hash_function(input, self._capacity);
        match self
            ._cells
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
        self._cells.iter().filter(|cell| cell.is_some()).count()
    }

    pub fn remove(&mut self, input: T) -> Option<T> {
        let hash_index = hash_function(&input, self._capacity);
        match self
            ._cells
            .get(hash_index)
            .expect("Unexpected error occured, found value None where Some(_) was expected.")
        {
            Some(s) => {
                if *s == input {
                    self._cells.remove(hash_index)
                } else {
                    self.remove_next(input, hash_index)
                }
            }
            None => None,
        }
    }

    pub fn insert_vector(&mut self, my_vector: &[T]) -> Result<(), &str> {
        for index in my_vector.iter() {
            self.insert(index.clone())
                .expect("Unable to insert full list.");
        }
        Ok(())
    }
}

//Private helper methods
impl<T> HashTable<T>
where
    T: Debug + Hashable + Clone + PartialEq,
{
    fn grow_cells(&mut self) -> Result<(), &str> {
        let new_capacity = self._capacity * 2;
        let new_cells: Vec<Option<T>> = vec![None; new_capacity];
        let old_cells = self._cells.clone();
        self._cells = new_cells;
        self._capacity = new_capacity;

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
            ._cells
            .get(next_index)
            .expect("Unexpected error occured, found value None where Some(_) was expected.")
        {
            Some(_) => Ok(self.insert_next_index(input, next_index)?),
            None => {
                self._cells[next_index] = Some(input);
                Ok(())
            }
        }
    }

    fn get_next(&self, input: &T, previous_index: usize) -> Option<&T> {
        let next_index = self.calculate_next_index_for_reccursion(previous_index);

        if next_index == hash_function(input, self._capacity) {
            return None;
        }

        match self
            ._cells
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

        if next_index == hash_function(&input, self._capacity) {
            return None;
        }

        match self
            ._cells
            .get(next_index)
            .expect("Unexpected error occured, found value None where Some(_) was expected.")
        {
            Some(a) => {
                if *a == input {
                    self._cells.remove(next_index)
                } else {
                    self.remove_next(input, next_index)
                }
            }
            None => self.remove_next(input, next_index),
        }
    }

    fn calculate_next_index_for_reccursion(&self, previous_index: usize) -> usize {
        let next_index = previous_index + 1;
        if next_index == self._capacity {
            0
        } else {
            next_index
        }
    }
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

    #[test]
    fn test_insert_vec() {
        let mut test_hash_table: HashTable<u32> = HashTable::new(SIZE_10, true);
        let my_vector = vec![22, 10, 2, 30, 23, 45, 1, 67, 4];
        test_hash_table
            .insert_vector(&my_vector)
            .expect("Error during test.");
        println!("{}", test_hash_table);
        assert!(test_hash_table.size() == 9);
    }
}
