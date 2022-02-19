use std::{marker::PhantomData, ops::Add};

pub struct Cardinal;
pub struct BlueJay;
pub struct Turkey;

pub trait Red {}

pub trait Blue {}

impl Red for Cardinal {}

impl Blue for BlueJay {}

pub fn red<T: Red>(_: &T) -> &'static str {
    "red"
}

pub fn blue<T: Blue>(_: &T) -> &'static str {
    "blue"
}

// Associate
pub struct Container(pub i32, pub i32);

pub trait Contains<A, B> {
    fn contains(&self, _: &A, _: &B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains<i32, i32> for Container {
    fn contains(&self, n1: &i32, n2: &i32) -> bool {
        (&self.0 == n1) && (&self.1 == n2)
    }

    fn first(&self) -> i32 {
        self.0
    }

    fn last(&self) -> i32 {
        self.1
    }
}

pub fn difference<A, B, C>(container: &C) -> i32
where
    C: Contains<A, B>,
{
    container.last() - container.first()
}

struct ContainerAss(i32, i32);

trait ContainsAss {
    type A;
    type B;

    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl ContainsAss for ContainerAss {
    type A = i32;
    type B = i32;

    fn contains(&self, n1: &Self::A, n2: &Self::B) -> bool {
        (&self.0 == n1) && (&self.1 == n2)
    }

    fn first(&self) -> i32 {
        self.0
    }

    fn last(&self) -> i32 {
        self.1
    }
}

fn differenceAss<C: ContainsAss>(container: &C) -> i32 {
    container.last() - container.first()
}

#[derive(Debug, Clone, Copy)]
pub enum Inch {}

#[derive(Debug, Clone, Copy)]
pub enum Mm {}

#[derive(Debug, Clone, Copy)]
pub struct Length<Unit>(pub f64, pub PhantomData<Unit>);

impl<Unit> Add for Length<Unit> {
    type Output = Length<Unit>;

    fn add(self, rhs: Length<Unit>) -> Length<Unit> {
        Length(self.0 + rhs.0, PhantomData)
    }
}
