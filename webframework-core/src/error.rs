#[macro_export]
macro_rules! impl_fail_boilerplate {
    ($kind:ident, $error:ident) => {
        impl failure::Fail for $error {
            fn cause(&self) -> Option<&failure::Fail> {
                self.inner.cause()
            }

            fn backtrace(&self) -> Option<&failure::Backtrace> {
                self.inner.backtrace()
            }
        }

        impl $error {
            pub fn kind(&self) -> $kind {
                self.inner.get_context().clone()
            }
        }

        impl From<$kind> for $error {
            fn from(kind: $kind) -> $error {
                $error { inner: Context::new(kind) }
            }
        }

        impl From<Context<$kind>> for $error {
            fn from(inner: Context<$kind>) -> $error {
                $error { inner: inner }
            }
        }

        impl std::fmt::Display for $error {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                std::fmt::Display::fmt(&self.inner, f)
            }
        }
    }
}
