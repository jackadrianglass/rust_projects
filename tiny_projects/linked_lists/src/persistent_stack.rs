// persistent stack is where all previous values of the stack are kept.
// Ownership here is shared so it's not possible to just use a Box which
// communicates unique ownership
//
// e.g.
// ```
// list1 = A -> B -> C -> D
// list2 = tail(list1) = B -> C -> D
// list3 = push(list2, X) = X -> B -> C -> D
// ```
//
// But we want our memory to look something like this
//
// ```
// list1 -> A ---+
//               |
//               v
// list2 ------> B -> C -> D
//               ^
//               |
// list3 -> X ---+
// ```
//
// In most other languages, the garbage collector would take care of cleaning
// up. But in Rust and other unmanaged languages, this doesn't work. We will
// use reference counting here to manage memory by using an `RC` (reference counted
// pointer)
#![allow(dead_code)]

use std::rc::Rc;

// Note that our list at the moment isn't thread safe. We can make it thread safe by using
// an ARC (atomically reference counted pointer). This has some overhead if thread safety isn't
// required.
//
// We know that types are thread safe when they implement Send and Sync. Send is basically where
// values can be safely sent between threads. Sync means that types can safely be shared between
// threads. These are auto derived marker traits (meaning that they don't add any behaviour
// technically in the form of methods)
//
// Types that don't implement Send and Sync are those that have interior mutability. The two major
// types are "cells" and "locks".
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    fn new() -> Self {
        List { head: None }
    }

    // Basically we're adding a new element who's `next` pointer is to the head of the list that is
    // currently being operated on. Since the values in the list are immutable, then we don't
    // really need to modify any of them in place
    fn prepend(&self, elem: T) -> Self {
        List {
            head: Some(Rc::new(Node {
                elem,
                next: self.head.clone(),
            })),
        }
    }

    // Terms coming from functional programming
    // [head] [...tail]
    // where
    // - head is the first element
    // - tail is the rest of the elements without the head
    fn tail(&self) -> Self {
        Self {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }

    fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

impl<T> Drop for List<T> {
    // We have the same recursion problem as the basic list when running the drop method.
    // In this case, we can't simply swap out in the drop since the node are shared references
    // rather than unique references.
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            // Here, we can check if there's only one reference left and do the regular swapping
            // and destruct the things that need to be destructed
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list1 = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), None);
        assert_eq!(list1.head(), Some(&3));

        let list = list1.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);

    }

    #[test]
    fn iter() {
        let list = List::new().prepend(1).prepend(2).prepend(3);
        let mut iter = list.iter();

        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }
}

