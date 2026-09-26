//! Page orders: where pages go when some of them move, and how to put
//! them back.

/// The order after moving `moving` (in any order) to the gap before page
/// `to`, where `to` may be `count` for the end. `order[i]` is the page
/// that ends up at index `i`; the moved pages keep their relative order.
pub fn moved_order(count: usize, moving: &[usize], to: usize) -> Vec<usize> {
    let mut moved: Vec<usize> = moving
        .iter()
        .copied()
        .filter(|page| *page < count)
        .collect();
    moved.sort_unstable();
    moved.dedup();
    let staying: Vec<usize> = (0..count).filter(|page| !moved.contains(page)).collect();
    let at = staying.iter().filter(|page| **page < to).count();
    let mut order = staying[..at].to_vec();
    order.extend(&moved);
    order.extend(&staying[at..]);
    order
}

/// The order that undoes `order`, which is also where each page went:
/// `inverse(order)[old] == new`.
pub fn inverse(order: &[usize]) -> Vec<usize> {
    let mut inverse = vec![0; order.len()];
    for (new, old) in order.iter().enumerate() {
        inverse[*old] = new;
    }
    inverse
}

/// Whether `order` leaves every page where it is.
pub fn is_identity(order: &[usize]) -> bool {
    order.iter().enumerate().all(|(index, page)| index == *page)
}

/// Single page moves, each `(from, to)` in the order at that point, that
/// turn the pages into `order`.
pub fn moves(order: &[usize]) -> Vec<(usize, usize)> {
    let mut current: Vec<usize> = (0..order.len()).collect();
    let mut moves = Vec::new();
    for (target, page) in order.iter().enumerate() {
        let Some(from) = current.iter().position(|current| current == page) else {
            continue;
        };
        if from != target {
            let page = current.remove(from);
            current.insert(target, page);
            moves.push((from, target));
        }
    }
    moves
}

#[cfg(test)]
mod tests {
    use super::*;

    fn apply(order: &[usize], pages: &[char]) -> Vec<char> {
        order.iter().map(|page| pages[*page]).collect()
    }

    #[test]
    fn moving_pages_forward_and_back() {
        let pages = ['a', 'b', 'c', 'd', 'e'];
        let order = moved_order(5, &[1, 2], 4);
        assert_eq!(apply(&order, &pages), ['a', 'd', 'b', 'c', 'e']);
        let order = moved_order(5, &[4], 0);
        assert_eq!(apply(&order, &pages), ['e', 'a', 'b', 'c', 'd']);
        let order = moved_order(5, &[3, 0], 5);
        assert_eq!(apply(&order, &pages), ['b', 'c', 'e', 'a', 'd']);
        // Dropping pages next to themselves changes nothing.
        assert!(is_identity(&moved_order(5, &[1, 2], 2)));
        assert!(is_identity(&moved_order(5, &[1, 2], 3)));
    }

    #[test]
    fn inverse_puts_pages_back() {
        let order = moved_order(6, &[0, 4], 3);
        let pages: Vec<usize> = (0..6).collect();
        let moved: Vec<usize> = order.iter().map(|page| pages[*page]).collect();
        let back: Vec<usize> = inverse(&order).iter().map(|page| moved[*page]).collect();
        assert_eq!(back, pages);
        // Where each page went.
        let went = inverse(&order);
        for (old, new) in went.iter().enumerate() {
            assert_eq!(order[*new], old);
        }
    }

    #[test]
    fn moves_replay_an_order() {
        let order = moved_order(7, &[6, 1, 3], 2);
        let mut pages: Vec<usize> = (0..7).collect();
        for (from, to) in moves(&order) {
            let page = pages.remove(from);
            pages.insert(to, page);
        }
        assert_eq!(pages, order);
    }
}
