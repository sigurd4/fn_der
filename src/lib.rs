#![feature(unboxed_closures)]

use fn_grad::*;

pub trait FnDerOnce<Arg>: FnOnce<(Arg,)> + FnGradOnce<(Arg,), Gradient = Self::Derivative>
{
    type Derivative: FnOnce<(Arg,)> + ?Sized;

    fn into_derivative(self) -> Self::Derivative
    where
        Self: Sized,
        Self::Derivative: Sized
    {
        self.into_gradient()
    }
}

pub trait FnDerMut<Arg>: FnDerOnce<Arg> + FnMut<(Arg,)> + FnGradMut<(Arg,)>
where
    Self::Derivative: FnMut<(Arg,)>
{
    fn as_derivate_mut(&mut self) -> &mut Self::Derivative
    {
        self.as_gradient_mut()
    }
}

pub trait FnDer<Arg>: FnDerMut<Arg> + Fn<(Arg,)> + FnGrad<(Arg,)>
where
    Self::Derivative: Fn<(Arg,)>
{
    fn as_derivate(&self) -> &Self::Derivative
    {
        self.as_gradient()
    }
}