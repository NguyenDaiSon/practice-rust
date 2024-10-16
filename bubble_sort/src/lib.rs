pub fn bubble_sort<T: PartialOrd>(a: &mut [T]) {
    let n = a.len();
    for i in 0..n {
        let mut is_sorted = true;
        for j in i..(n - 1) {
            if a[j] > a[j + 1] {
                a.swap(j, j + 1);
                is_sorted = false;
            }
        }

        if is_sorted {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::bubble_sort;

    #[test]
    fn bubble_sort_test_0() {
        let mut actual_vec: Vec<i32> = vec![];
        let expected_vec: Vec<i32> = vec![];
        bubble_sort(&mut actual_vec);
        assert_eq!(actual_vec, expected_vec);
    }

    #[test]
    fn bubble_sort_test_1() {
        let mut actual_vec = vec![2];
        let expected_vec = vec![2];
        bubble_sort(&mut actual_vec);
        assert_eq!(actual_vec, expected_vec);
    }

    #[test]
    fn bubble_sort_test_2() {
        let mut actual_vec = vec![2, 1];
        let expected_vec = vec![1, 2];
        bubble_sort(&mut actual_vec);
        assert_eq!(actual_vec, expected_vec);
    }

    #[test]
    fn bubble_sort_test_3() {
        let mut actual_vec = vec![2, 1, 4];
        let expected_vec = vec![1, 2, 4];
        bubble_sort(&mut actual_vec);
        assert_eq!(actual_vec, expected_vec);
    }

    #[test]
    fn bubble_sort_test_4() {
        let mut actual_vec = vec![2, 1, 4, 3];
        let expected_vec = vec![1, 2, 3, 4];
        bubble_sort(&mut actual_vec);
        assert_eq!(actual_vec, expected_vec);
    }
}
