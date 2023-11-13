pub trait Transition<A> {
    fn transite(&self, action: &A) -> Self;
    fn initial_state() -> Self;
}

pub trait FiniteTransition<A>: Transition<A> {
    fn final_state() -> Self;
    fn is_finished(&self) -> bool;
}

pub trait FiniteLinearTransition {
    fn transite(&self) -> Self;
    fn initial_state() -> Self;
    fn final_state() -> Self;
    fn is_finished(&self) -> bool;
}

pub trait CycleLinearTransition {
    fn transite(&self) -> Self;
    fn initial_state() -> Self;
}
