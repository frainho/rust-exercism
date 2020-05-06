use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list == second_list {
        return Comparison::Equal;
    }

    if is_sublist_of(first_list, second_list) {
        return Comparison::Sublist;
    }

    if is_sublist_of(second_list, first_list) {
        return Comparison::Superlist;
    }

    Comparison::Unequal
}

fn is_sublist_of<T: PartialEq>(list_a: &[T], list_b: &[T]) -> bool {
    if list_a.is_empty() {
        return true;
    }

    for subset in list_b.windows(list_a.len()) {
        if subset == list_a {
            return true;
        }
    }
    false
}
