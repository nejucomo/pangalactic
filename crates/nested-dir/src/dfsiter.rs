use either::Either::{self, Left, Right};
use pangalactic_dir::Directory;
use pangalactic_name::Name;

use crate::NDBranch::*;
use crate::{NDNode, NestedDirectory};

#[derive(Debug)]
pub struct DfsIter<N, L>(Option<Either<(N, L), Guts<N, L>>>);

impl<N, L> From<NDNode<N, L>> for DfsIter<N, L> {
    fn from(NDNode { data, branch }: NDNode<N, L>) -> Self {
        DfsIter(Some(match branch {
            Subdir(d) => Right(Guts::from((data, d))),
            Leaf(l) => Left((data, l)),
        }))
    }
}

impl<N, L> Iterator for DfsIter<N, L> {
    type Item = (Vec<Name>, N, Option<L>);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.take().and_then(|ei| match ei {
            Left((n, l)) => Some((vec![], n, Some(l))),
            Right(mut guts) => {
                let optitem = guts.next();
                if optitem.is_some() {
                    self.0 = Some(Right(guts));
                }
                optitem
            }
        })
    }
}

#[derive(Debug)]
pub struct Guts<N, L> {
    stack: Vec<(N, DirIter<N, L>)>,
    path: Vec<Name>,
}

type DirIter<N, L> = <Directory<NDNode<N, L>> as IntoIterator>::IntoIter;

impl<N, L> From<(N, Box<NestedDirectory<N, L>>)> for Guts<N, L> {
    fn from((data, d): (N, Box<NestedDirectory<N, L>>)) -> Self {
        Guts {
            stack: vec![(data, (*d).into())],
            path: vec![],
        }
    }
}

impl<N, L> Iterator for Guts<N, L> {
    type Item = (Vec<Name>, N, Option<L>);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((data, mut it)) = self.stack.pop() {
            if let Some((childname, child)) = it.next() {
                self.stack.push((data, it));
                match child.branch {
                    Subdir(d) => {
                        self.path.push(childname);
                        self.stack.push((child.data, (*d).into()));
                        continue;
                    }
                    Leaf(l) => {
                        let mut path = self.path.clone();
                        path.push(childname);
                        return Some((path, child.data, Some(l)));
                    }
                }
            } else {
                let path = self.path.clone();
                self.path.pop();
                return Some((path, data, None));
            }
        }
        None
    }
}
