use std::convert;
use std::mem;

#[derive(Debug)]
pub struct VariantEquals<'a, T>(&'a T);

impl<'a, T> PartialEq for VariantEquals<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        mem::discriminant(self) == mem::discriminant(other)
    }
}

impl<'a, T> convert::From<&'a T> for VariantEquals<'a, T> {
    fn from(v: &'a T) -> Self {
        VariantEquals(v)
    }
}
