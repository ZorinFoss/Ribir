use super::*;
use crate::{pipe::*, widget::*};

impl<T: ChildOfCompose> ComposeChildFrom<T, 0> for T {
  #[inline]
  fn compose_child_from(from: T) -> Self { from }
}

impl<F: FnMut() -> Widget<'static> + 'static> ComposeChildFrom<F, 1> for GenWidget {
  #[inline]
  fn compose_child_from(from: F) -> Self { GenWidget::new(from) }
}

impl<'w, F: FnOnce() -> Widget<'w> + 'w> ComposeChildFrom<F, FN> for FnWidget<'w> {
  #[inline]
  fn compose_child_from(from: F) -> FnWidget<'w> { FnWidget::new(from) }
}

impl<'a, const M: usize, T: IntoWidget<'a, M>> ComposeChildFrom<T, M> for Widget<'a> {
  #[inline(always)]
  fn compose_child_from(from: T) -> Widget<'a> { from.into_widget() }
}

impl<W, C: ComposeChildFrom<T, M>, T, const M: usize> ComposeChildFrom<Pair<W, T>, M>
  for Pair<W, C>
{
  fn compose_child_from(from: Pair<W, T>) -> Pair<W, C> {
    let Pair { parent, child } = from;
    Pair { parent, child: C::compose_child_from(child) }
  }
}

impl<P: Pipe> ComposeChildFrom<P, 1> for BoxPipe<P::Value> {
  #[inline]
  fn compose_child_from(from: P) -> Self { BoxPipe::pipe(Box::new(from)) }
}

impl<U, const M: usize, T: DeclareInto<U, M>> ComposeChildFrom<T, M> for DeclareInit<U> {
  #[inline]
  fn compose_child_from(from: T) -> Self { from.declare_into() }
}

impl<T, C, const M: usize> IntoChildCompose<C, M> for T
where
  C: ComposeChildFrom<T, M>,
{
  fn into_child_compose(self) -> C { C::compose_child_from(self) }
}