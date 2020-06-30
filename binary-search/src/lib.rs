pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if !array.contains(&key) {
        return None
    };
    let start = 0usize;
    let end = array.len() - 1;
    
    find_with_indices(array, key, start, end)
}

fn find_with_indices(array: &[i32], key: i32, start: usize, end: usize) -> Option<usize> {
    if start > end {
        return None;
    }
    let mid = (start + end) / 2;

    if array[mid] == key {
        return Some(mid);
    } else if array[mid] > key {
        return find_with_indices(array, key, start, mid - 1);
    } else if array[mid] < key {
        return find_with_indices(array, key, mid + 1, end);
    } else {
        return None;
    }
}
