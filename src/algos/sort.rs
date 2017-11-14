use std::collections::BTreeMap;
use std::convert::AsRef;

pub struct Sorter;

// ToDo: merge with Indexing?

impl Sorter {
    pub fn sort<T, R>(values: R) -> Vec<T>
    where
        T: Clone + Ord,
        R: AsRef<Vec<T>>,
    {

        let values: &Vec<T> = values.as_ref();

        // Clone all elements
        let mut result = values.clone();
        result.sort();
        result
    }

    /// Sort by values returning indexer and sorted values
    pub fn argsort<T, R>(values: R) -> (Vec<usize>, Vec<T>)
    where
        T: Clone + Ord,
        R: AsRef<Vec<T>>,
    {

        let values: &Vec<T> = values.as_ref();
        let mut map: BTreeMap<T, Vec<usize>> = BTreeMap::new();

        for (i, v) in values.iter().enumerate() {
            let e = map.entry(v.clone()).or_insert(Vec::<usize>::new());
            e.push(i);
        }

        let mut sorted: Vec<T> = Vec::with_capacity(values.len());
        let mut indexer: Vec<usize> = Vec::with_capacity(values.len());

        for (k, locs) in map.into_iter() {
            for loc in locs {
                sorted.push(k.clone());
                indexer.push(loc);
            }
        }
        (indexer, sorted)
    }

    /// Sort values by key returning sorted key and values
    pub fn sort_by<T, U>(keys: &Vec<T>, values: &Vec<U>) -> (Vec<T>, Vec<U>)
    where
        T: Clone + Ord,
        U: Clone,
    {

        let mut map: BTreeMap<T, Vec<U>> = BTreeMap::new();

        for (k, v) in keys.iter().zip(values) {
            let e = map.entry(k.clone()).or_insert(Vec::<U>::new());
            e.push(v.clone());
        }

        let mut sorted_keys: Vec<T> = Vec::with_capacity(values.len());
        let mut sorted_values: Vec<U> = Vec::with_capacity(values.len());

        for (k, vals) in map.into_iter() {
            for v in vals {
                sorted_keys.push(k.clone());
                sorted_values.push(v);
            }
        }
        (sorted_keys, sorted_values)
    }
}

#[cfg(test)]
mod tests {

    use super::Sorter;

    #[test]
    fn test_argsort_empty() {
        let (indexer, sorted) = Sorter::argsort(&Vec::<i64>::new());
        assert_eq!(indexer, vec![]);
        assert_eq!(sorted, vec![]);
    }

    #[test]
    fn test_argsort_int() {
        let (indexer, sorted) = Sorter::argsort(&vec![2, 2, 2, 3, 3]);
        assert_eq!(indexer, vec![0, 1, 2, 3, 4]);
        assert_eq!(sorted, vec![2, 2, 2, 3, 3]);

        let (indexer, sorted) = Sorter::argsort(&vec![4, 2, 1, 3, 3]);
        assert_eq!(indexer, vec![2, 1, 3, 4, 0]);
        assert_eq!(sorted, vec![1, 2, 3, 3, 4]);
    }

    #[test]
    fn test_argsort_str() {
        let (indexer, sorted) = Sorter::argsort(&vec!["b", "bb", "a", "bb"]);
        assert_eq!(indexer, vec![2, 0, 1, 3]);
        assert_eq!(sorted, vec!["a", "b", "bb", "bb"]);
    }

    #[test]
    fn test_sort_by_int() {
        let (keys, vals) = Sorter::sort_by(&vec![5, 4, 3, 2, 1], &vec![1, 2, 3, 4, 5]);
        assert_eq!(keys, vec![1, 2, 3, 4, 5]);
        assert_eq!(vals, vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_sort_by_int_dup() {
        let (keys, vals) = Sorter::sort_by(&vec![3, 3, 1, 1, 1], &vec![1, 2, 3, 4, 5]);
        assert_eq!(keys, vec![1, 1, 1, 3, 3]);
        assert_eq!(vals, vec![3, 4, 5, 1, 2]);
    }

    #[test]
    fn test_sort_by_int_float() {
        let (keys, vals) = Sorter::sort_by(&vec![3, 2, 1, 4, 1], &vec![1.1, 2.1, 3.1, 4.1, 5.1]);
        assert_eq!(keys, vec![1, 1, 2, 3, 4]);
        assert_eq!(vals, vec![3.1, 5.1, 2.1, 1.1, 4.1]);
    }
}
