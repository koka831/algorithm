use std::cmp;
use cargo_snippet::snippet;

/// BinarySearch
#[snippet("BinarySearch")]
pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}

#[snippet("BinarySearch")]
impl<T: Ord> BinarySearch<T> for [T] {
    /// returns the minimum index s.t. array[index] >= x
    fn lower_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                cmp::Ordering::Less => {
                    low = mid + 1;
                },
                _ => {
                    high = mid;
                }
            }
        }
        low
    }

    /// returns the minimum index s.t. array[index] > x
    fn upper_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();
        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                cmp::Ordering::Greater => {
                    high = mid;
                },
                _ => {
                    low = mid + 1;
                }
            }
        }
        low
    }
}

#[cfg(test)]
mod tests {
    use super::BinarySearch;

    #[test]
    fn test_lower_bound() {
        let array = vec![2, 2, 5, 5, 9];
        assert_eq!(array.lower_bound(&2), 0);
        assert_eq!(array.lower_bound(&4), 2);
        assert_eq!(array.lower_bound(&5), 2);
        assert_eq!(array.lower_bound(&7), 4);
        assert_eq!(array.lower_bound(&100), 5);
    }

    #[test]
    fn test_upper_bound() {
        let array = vec![2, 2, 5, 5, 9];
        assert_eq!(array.upper_bound(&2), 2);
        assert_eq!(array.upper_bound(&4), 2);
        assert_eq!(array.upper_bound(&5), 4);
        assert_eq!(array.upper_bound(&7), 4);
        assert_eq!(array.upper_bound(&100), 5);
    }
}
