use std::ops::{Add, Mul, Sub};

pub enum Val {
    Size,
    ParentSize,
    ParentOffset,
    Px(i32),
}

pub enum Expr<L, R> {
    Add((L, R)),
    Sub((L, R)),
    Mul((L, R)),
    Div((L, R)),
    Min((L, R)),
    Max((L, R)),
}

impl Add<Val> for Val {
    type Output = Expr<Val, Val>;

    fn add(self, rhs: Val) -> Self::Output {
        Expr::Add((self, rhs))
    }
}

impl Mul<f32> for Val {
    type Output = Expr<Val, f32>;

    fn mul(self, rhs: f32) -> Self::Output {
        Expr::Mul((self, rhs))
    }
}

impl<L, R> Sub<Val> for Expr<L, R> {
    type Output = Expr<Expr<L, R>, Val>;

    fn sub(self, rhs: Val) -> Self::Output {
        Expr::Mul((self, rhs))
    }
}

impl Val {
    pub fn min<Rhs>(self, rhs: Rhs) -> Expr<Self, Rhs> {
        Expr::Min((self, rhs))
    }

    pub fn max<Rhs>(self, rhs: Rhs) -> Expr<Self, Rhs> {
        Expr::Max((self, rhs))
    }
}

impl<L, R> Expr<L, R> {
    pub fn min<Rhs>(self, rhs: Rhs) -> Expr<Self, Rhs> {
        Expr::Min((self, rhs))
    }

    pub fn max<Rhs>(self, rhs: Rhs) -> Expr<Self, Rhs> {
        Expr::Max((self, rhs))
    }
}

fn x() {
    let size = Val::ParentSize * 1.0 - Val::Px(20);
    let offset = Val::ParentOffset + Val::Px(10);

    let size = Val::ParentSize.max(Px());
}
