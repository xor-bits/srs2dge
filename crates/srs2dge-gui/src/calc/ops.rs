use crate::prelude::{BaseOffset, BaseSize, Const, GuiCalc, Max, Min, SelfSize, WidgetLayout};
use srs2dge_core::glam::Vec2;
use std::ops::{Add, Div, Mul, Sub};

//

macro_rules! size_calc_math {
    ($name:tt $op:tt) => {
		#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
        pub struct $name<T: GuiCalc, U: GuiCalc> {
            pub lhs: T,
            pub rhs: U,
        }

        impl<T: GuiCalc, U: GuiCalc> GuiCalc for $name<T, U> {
            fn reduce(&self, refs: &(WidgetLayout, Vec2)) -> Vec2 {
                self.lhs.reduce(refs) $op self.rhs.reduce(refs)
            }
        }
    };
}

size_calc_math! { GuiCalcAdd + }
size_calc_math! { GuiCalcSub - }
size_calc_math! { GuiCalcMul * }
size_calc_math! { GuiCalcDiv / }

//

macro_rules! impl_op {
    (0 $ty:tt $op_f:ident $op:tt for $name:tt) => {
        impl<T: GuiCalc> $op<T> for $ty {
            type Output = $name<$ty, T>;

            fn $op_f(self, rhs: T) -> Self::Output {
                Self::Output { lhs: self, rhs }
            }
        }
    };

    (1 $ty:tt $op_f:ident $op:tt for $name:tt) => {
        impl<T: GuiCalc, A: GuiCalc, B: GuiCalc> $op<T> for $ty<A, B> {
            type Output = $name<$ty<A, B>, T>;

            fn $op_f(self, rhs: T) -> Self::Output {
                Self::Output { lhs: self, rhs }
            }
        }
    };

    ($op_f:ident $op:tt for $name:tt) => {
        impl_op! { 0 Const $op_f $op for $name }
        impl_op! { 0 BaseSize $op_f $op for $name }
        impl_op! { 0 BaseOffset $op_f $op for $name }
        impl_op! { 0 SelfSize $op_f $op for $name }

        impl_op! { 1 GuiCalcAdd $op_f $op for $name }
        impl_op! { 1 GuiCalcSub $op_f $op for $name }
        impl_op! { 1 GuiCalcMul $op_f $op for $name }
        impl_op! { 1 GuiCalcDiv $op_f $op for $name }
        impl_op! { 1 Min $op_f $op for $name }
        impl_op! { 1 Max $op_f $op for $name }
    };
}

impl_op! { add Add for GuiCalcAdd }
impl_op! { sub Sub for GuiCalcSub }
impl_op! { mul Mul for GuiCalcMul }
impl_op! { div Div for GuiCalcDiv }
