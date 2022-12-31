pub fn bin_search(nums: Vec<i64>, num: i64) -> Option<usize> {
    let mut lower_bound = 0;
    let mut upper_bound = nums.len() - 1;

    while lower_bound <= upper_bound {
        let mid = (lower_bound + upper_bound) / 2;
        let guess = nums[mid];
        if guess == num {
            return Some(mid);
        }
        if guess > num {
            upper_bound = mid - 1;
        } else {
            lower_bound = mid + 1;
        }
    }
    return None;
}

#[cfg(test)]
mod tests {
    use crate::binary_search::bin_search;

    #[test]
    fn test_bin_search_ok() {
        let num_index = bin_search(vec![-5, 0, 143, 800, 14800], 143);
        assert_eq!(2, num_index.unwrap());
    }

    #[test]
    fn test_bin_search_with_vector_size_one_ok() {
        let num_index = bin_search(vec![-5], -5);
        assert_eq!(0, num_index.unwrap());
    }
}
