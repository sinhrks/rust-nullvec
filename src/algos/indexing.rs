
pub struct Indexing;

impl Indexing {
    pub fn blocs<T: Clone>(values: &[T], flags: &[bool]) -> Vec<T> {
        assert!(
            values.len() == flags.len(),
            "flags must be the same length as values"
        );
        // should use filter_map?
        let new_values: Vec<T> = values
            .iter()
            .zip(flags.iter())
            .filter(|&(_, y)| *y)
            .map(|(ref x, _)| (*x).clone())
            .collect();
        new_values
    }

    /// reorder values based on given locations
    pub fn reindex<T: Clone>(values: &[T], locs: &[usize]) -> Vec<T> {
        Self::assert_index_boundary(values, locs);
        unsafe { Self::reindex_unchecked(values, locs) }
    }

    /// reorder values based on given locations
    pub unsafe fn reindex_unchecked<T: Clone>(values: &[T], locs: &[usize]) -> Vec<T> {
        locs.iter()
            .map(|&i| values.get_unchecked(i).clone())
            .collect()
    }

    pub fn assert_index_boundary<T>(values: &[T], locs: &[usize]) {
        let len = values.len();
        assert!(locs.iter().all(|&i| i < len), "Index out of bounds");
    }
}

#[cfg(test)]
mod tests {

    use super::Indexing;

    #[test]
    fn test_blocs() {
        let i = vec![1, 2, 3];
        let f = vec![true, false, true];
        assert_eq!(Indexing::blocs(&i, &f), vec![1, 3]);

        let s: Vec<String> = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        assert_eq!(
            Indexing::blocs(&s, &f),
            vec!["a".to_string(), "c".to_string()]
        );
    }

    #[test]
    #[should_panic]
    fn test_blocs_length_mismatch() {
        let i = vec![1, 2, 3, 4];
        let f = vec![true, false, true];
        Indexing::blocs(&i, &f);
    }

    #[test]
    fn test_reindex() {
        let res = Indexing::reindex(&vec![1, 2, 3, 4, 5], &vec![4, 2, 3]);
        assert_eq!(res, vec![5, 3, 4]);
    }
}
