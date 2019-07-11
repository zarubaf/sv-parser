use crate::ast::*;
use crate::parser::*;

include!(concat!(env!("OUT_DIR"), "/any_node.rs"));

pub struct AnyNodes<'a>(pub Vec<AnyNode<'a>>);

impl<'a> From<Vec<AnyNode<'a>>> for AnyNodes<'a> {
    fn from(x: Vec<AnyNode<'a>>) -> Self {
        AnyNodes(x)
    }
}

impl<'a> From<&'a Span<'a>> for AnyNodes<'a> {
    fn from(x: &'a Span<'a>) -> Self {
        vec![AnyNode::Span(x)].into()
    }
}

impl<'a, T: 'a> From<&'a Vec<T>> for AnyNodes<'a>
where
    &'a T: Into<AnyNodes<'a>>,
{
    fn from(x: &'a Vec<T>) -> Self {
        let mut ret = Vec::new();
        for x in x {
            ret.append(&mut x.into().0);
        }
        ret.into()
    }
}

impl<'a, T: 'a> From<&'a Option<T>> for AnyNodes<'a>
where
    &'a T: Into<AnyNodes<'a>>,
{
    fn from(x: &'a Option<T>) -> Self {
        let mut ret = Vec::new();
        if let Some(x) = x {
            ret.append(&mut x.into().0);
        }
        ret.into()
    }
}

impl<'a, T: 'a, U: 'a> From<&'a (T, U)> for AnyNodes<'a>
where
    &'a T: Into<AnyNodes<'a>>,
    &'a U: Into<AnyNodes<'a>>,
{
    fn from(x: &'a (T, U)) -> Self {
        let mut ret = Vec::new();
        let (t, u) = x;
        ret.append(&mut t.into().0);
        ret.append(&mut u.into().0);
        ret.into()
    }
}

impl<'a, T: 'a, U: 'a, V: 'a> From<&'a (T, U, V)> for AnyNodes<'a>
where
    &'a T: Into<AnyNodes<'a>>,
    &'a U: Into<AnyNodes<'a>>,
    &'a V: Into<AnyNodes<'a>>,
{
    fn from(x: &'a (T, U, V)) -> Self {
        let mut ret = Vec::new();
        let (t, u, v) = x;
        ret.append(&mut t.into().0);
        ret.append(&mut u.into().0);
        ret.append(&mut v.into().0);
        ret.into()
    }
}

impl<'a, T: 'a, U: 'a, V: 'a, W: 'a> From<&'a (T, U, V, W)> for AnyNodes<'a>
where
    &'a T: Into<AnyNodes<'a>>,
    &'a U: Into<AnyNodes<'a>>,
    &'a V: Into<AnyNodes<'a>>,
    &'a W: Into<AnyNodes<'a>>,
{
    fn from(x: &'a (T, U, V, W)) -> Self {
        let mut ret = Vec::new();
        let (t, u, v, w) = x;
        ret.append(&mut t.into().0);
        ret.append(&mut u.into().0);
        ret.append(&mut v.into().0);
        ret.append(&mut w.into().0);
        ret.into()
    }
}

pub struct Iter<'a> {
    pub next: AnyNodes<'a>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = AnyNode<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.next.0.pop();
        if let Some(x) = ret.clone() {
            let mut x = x.next();
            x.0.reverse();
            self.next.0.append(&mut x.0);
        }
        ret
    }
}