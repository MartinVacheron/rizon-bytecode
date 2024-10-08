use anyhow::{bail, Result};
use std::{fmt::Display, ops::Deref};
use Value::*;

use crate::{gc::GcRef, object::{BoundMethod, Closure, Function, Instance, Iterator, Struct}};


#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(GcRef<String>),
    Iter(GcRef<Iterator>),
    Fn(GcRef<Function>),
    Closure(GcRef<Closure>),
    NativeFn(NativeFunction),
    Struct(GcRef<Struct>),
    Instance(GcRef<Instance>),
    BoundMethod(GcRef<BoundMethod>),
    Null,
}

pub type NativeFunction = fn(usize, usize) -> Value;

impl Value {
    pub fn sub(self, other: Self) -> Option<Self> {
        match (self, other) {
            (Int(v1), Int(v2)) => Some(Int(v1 - v2)),
            (Float(v1), Float(v2)) => Some(Float(v1 - v2)),
            _ => None,
        }
    }

    pub fn mul(self, other: Self) -> Option<Self> {
        match (self, other) {
            (Int(v1), Int(v2)) => Some(Int(v1 * v2)),
            (Float(v1), Float(v2)) => Some(Float(v1 * v2)),
            _ => None,
        }
    }

    pub fn div(self, other: Self) -> Option<Self> {
        match (self, other) {
            (Int(v1), Int(v2)) => Some(Int(v1 / v2)),
            (Float(v1), Float(v2)) => Some(Float(v1 / v2)),
            _ => None,
        }
    }

    pub fn eq(self, other: Self) -> Option<Self> {
        match (self, other) {
            (Int(_), Int(_))
            | (Float(_), Float(_))
            | (Bool(_), Bool(_))
            | (Str(_), Str(_))
            | (Null, Null) => Some(Bool(self == other)),
            (_, Null) => Some(Bool(false)),
            (Null, _) => Some(Bool(false)),
            _ => None,
        }
    }

    pub fn lt(self, other: Self) -> Option<Self> {
        match (self, other) {
            (Int(v1), Int(v2)) => Some(Bool(v1 < v2)),
            (Float(v1), Float(v2)) => Some(Bool(v1 < v2)),
            _ => None,
        }
    }

    pub fn gt(self, other: Self) -> Option<Self> {
        match (self, other) {
            (Int(v1), Int(v2)) => Some(Bool(v1 > v2)),
            (Float(v1), Float(v2)) => Some(Bool(v1 > v2)),
            _ => None,
        }
    }

    pub fn negate(&mut self) -> Result<()> {
        match self {
            Int(v) => *v *= -1,
            Float(v) => *v *= -1.,
            _ => bail!("can't negate type other than int and float")
        }

        Ok(())
    }

    pub fn not(&mut self) -> Result<()> {
        match self {
            Bool(v) => *v = !*v,
            _ => bail!("can't use not operator on other type than bool")
        }

        Ok(())
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Int(v) => write!(f, "{}", v),
            Float(v) => write!(f, "{}", v),
            Bool(v) => write!(f, "{}", v),
            Str(v) => write!(f, "\"{}\"", v.deref()),
            Iter(v) => write!(f, "<iter {} -> {}>", v.range.start, v.range.end),
            Fn(v) => write!(f, "<fn {}>", v.name.deref()),
            Closure(v) => write!(f, "<fn {}>", v.function.name.deref()),
            NativeFn(_) => write!(f, "<native fn>"),
            Struct(v) => write!(f, "<structure {}>", v.name.deref()),
            Instance(v) => write!(f, "<instance of {}>", v.structure.name.deref()),
            BoundMethod(v) => write!(f, "<fn {}>", v.method.function.name.deref()),
            Null => write!(f, "null"),
        }
    }
}