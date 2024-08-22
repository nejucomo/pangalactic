use either::Either::{self, Left, Right};
use pangalactic_dir::{Directory, Name};

use crate::{NDNode, NestedDirectory};

#[derive(Debug)]
pub enum DfsIter<L, B> {
    Complete,
    Stack(DirIter<L, B>, Vec<StackItem<L, B>>),
}
use DfsIter::*;

type DirIter<L, B> = <Directory<NDNode<L, B>> as IntoIterator>::IntoIter;
type StackItem<L, B> = (Name, B, DirIter<L, B>);

impl<L, B> DfsIter<L, B> {
    fn next_ctl(&mut self) -> IterCtl<L, B> {
        match self {
            Complete => ReturnNone,
            Stack(first, stack) => {
                let last = stack.last_mut().map(|(_, _, it)| it).unwrap_or(first);
                if let Some((name, node)) = last.next() {
                    Self::node_ctl(stack, name, node)
                } else if let Some((name, b, _)) = stack.pop() {
                    ReturnItem(Self::make_path(stack, name), Right(b))
                } else {
                    *self = Complete;
                    Continue
                }
            }
        }
    }

    fn node_ctl(stack: &mut Vec<StackItem<L, B>>, name: Name, node: NDNode<L, B>) -> IterCtl<L, B> {
        use NDNode::*;

        match node {
            Branch(nested, b) => {
                stack.push((name, b, Directory::from(*nested).into_iter()));
                Continue
            }
            Leaf(l) => ReturnItem(Self::make_path(stack, name), Left(l)),
        }
    }

    fn make_path(stack: &[StackItem<L, B>], name: Name) -> Vec<Name> {
        let mut path: Vec<Name> = stack.iter().map(|(n, _, _)| n.to_string()).collect();
        path.push(name);
        path
    }
}

impl<L, B> From<NestedDirectory<L, B>> for DfsIter<L, B> {
    fn from(nd: NestedDirectory<L, B>) -> Self {
        DfsIter::from(Directory::from(nd))
    }
}

impl<L, B> From<Directory<NDNode<L, B>>> for DfsIter<L, B> {
    fn from(d: Directory<NDNode<L, B>>) -> Self {
        Stack(d.into_iter(), vec![])
    }
}

impl<L, B> Iterator for DfsIter<L, B> {
    type Item = (Vec<Name>, Either<L, B>);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(optitem) = self.next_ctl().into() {
                return optitem;
            }
        }
    }
}

enum IterCtl<L, B> {
    Continue,
    ReturnNone,
    ReturnItem(Vec<Name>, Either<L, B>),
}
use IterCtl::*;

impl<L, B> From<IterCtl<L, B>> for Option<Option<(Vec<Name>, Either<L, B>)>> {
    fn from(ctl: IterCtl<L, B>) -> Self {
        match ctl {
            Continue => None,
            ReturnNone => Some(None),
            ReturnItem(p, e) => Some(Some((p, e))),
        }
    }
}
