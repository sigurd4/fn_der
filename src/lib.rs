#![feature(unboxed_closures)]
#![feature(const_trait_impl)]
#![feature(const_mut_refs)]
#![feature(associated_type_defaults)]

use fn_grad::*;

#[const_trait]
pub trait FnDerOnce<Arg>: FnOnce<(Arg,)> + ~const FnGradOnce<(Arg,)>
{
    fn derivative_once(self, arg: Arg) -> Self::GradientOutput
    where
        Self: Sized
    {
        self.gradient_once((arg,))
    }
}

#[const_trait]
pub trait FnDerMut<Arg>: FnDerOnce<Arg> + FnMut<(Arg,)> + ~const FnGradMut<(Arg,)>
{
    fn derivative_mut(&mut self, arg: Arg) -> Self::GradientOutput
    {
        self.gradient_mut((arg,))
    }
}

#[const_trait]
pub trait FnDer<Arg>: FnDerMut<Arg> + Fn<(Arg,)> + ~const FnGrad<(Arg,)>
{
    fn derivative(&self, arg: Arg) -> Self::GradientOutput
    {
        self.gradient((arg,))
    }
}