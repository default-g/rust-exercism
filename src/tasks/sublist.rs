#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {

    if _first_list == _second_list {
        return Comparison::Equal;
    }

    if _first_list.len() == 0 {
        return Comparison::Sublist;
    }

    if _second_list.len() == 0 {
        return Comparison::Superlist;
    }

    for subslice in _first_list.windows(_second_list.len()) {
        if subslice == _second_list {
            return Comparison::Superlist;
        }
    }

    for sublist in _second_list.windows(_first_list.len()) {
        if sublist == _first_list {
            return Comparison::Sublist;
        }
    }

    return Comparison::Unequal;
}

