use std::collections::HashSet;
use std::hash::Hash;

// -----------------------------------------------------------------------------
pub fn vec_hashset_eq<T>(some: &Vec<HashSet<T>>, other: &Vec<HashSet<T>>) -> bool
where T: Copy + Eq + Hash
{
    let eq = some.iter().all(|s| other.contains(s))
        && other.iter().all(|s| some.contains(s));

    eq
}

// =============================================================================
#[cfg(test)]
mod test {
    use super::*;

    // -------------------------------------------------------------------------
    #[test]
    fn test_vec_hashset_eq() {
        let some = vec![
            HashSet::from([0, 2, 1]),
            HashSet::from([4]),
            HashSet::from([]),
        ];
        let other = vec![
            HashSet::from([]),
            HashSet::from([2, 1, 0]),
            HashSet::from([4])
        ];
        assert!(vec_hashset_eq(&some, &other));

        let other = vec![
            HashSet::from([]),
            HashSet::from([2, 1, 0]),
            HashSet::from([3])
        ];
        assert!(!vec_hashset_eq(&some, &other));

        let other = vec![
            HashSet::from([]),
            HashSet::from([2, 1, 0]),
        ];
        assert!(!vec_hashset_eq(&some, &other));
    }
}
