pub mod first;
pub mod third;
pub mod fourth;
#[cfg(test)]
mod tests {
//    use super::third::List;
//    #[test]
//    fn peek () {
//        let mut list = List::new();
//
//        assert_eq!(list.peek(), None);
//        assert_eq!(list.peek_mut(), None);
//
//        list.push(1);
//        list.push(2);
//        list.push(3);
//
//        assert_eq!(list.peek(), Some(&3));
//        assert_eq!(list.peek_mut(), Some(&mut 3));
//
//        list.peek_mut().map(|value| {
//            *value = 42
//        });
//
//        let mut iter = list.into_iter();
//        assert_eq!(iter.next(), Some(42));
//        assert_eq!(iter.next(), Some(2));
//        assert_eq!(iter.next(), Some(1));
//        assert_eq!(iter.next(), None);
//    }
    #[test]
    fn basics() {
    let list = List::new();
    assert_eq!(list.head(), None);

    let list = list.append(1).append(2).append(3);
    assert_eq!(list.head(), Some(&3));

    let list = list.tail();
    assert_eq!(list.head(), Some(&2));

    let list = list.tail();
    assert_eq!(list.head(), Some(&1));

    let list = list.tail();
    assert_eq!(list.head(), None);

    // Make sure empty tail works
    let list = list.tail();
    assert_eq!(list.head(), None);
    }
}
