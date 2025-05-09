#[macro_export]
macro_rules! impl_common_event_deref {
  ($event_name:ident) => {
    impl std::ops::Deref for $event_name {
      type Target = CommonEvent;

      #[inline]
      fn deref(&self) -> &Self::Target { &self.common }
    }

    impl std::ops::DerefMut for $event_name {
      #[inline]
      fn deref_mut(&mut self) -> &mut Self::Target { &mut self.common }
    }

    impl std::borrow::Borrow<CommonEvent> for $event_name {
      #[inline]
      fn borrow(&self) -> &CommonEvent { &self.common }
    }

    impl std::borrow::BorrowMut<CommonEvent> for $event_name {
      #[inline]
      fn borrow_mut(&mut self) -> &mut CommonEvent { &mut self.common }
    }

    impl AsRef<$crate::prelude::ProviderCtx> for $event_name {
      fn as_ref(&self) -> &$crate::prelude::ProviderCtx { self.common.as_ref() }
    }

    impl AsMut<$crate::prelude::ProviderCtx> for $event_name {
      fn as_mut(&mut self) -> &mut $crate::prelude::ProviderCtx { self.common.as_mut() }
    }
  };
}
