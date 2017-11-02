use core::fmt::{self, Debug, Display};

without_std! {
    /// A `Backtrace`.
    ///
    /// This is an opaque wrapper around the backtrace provided by
    /// libbacktrace. A variety of optimizations have been performed to avoid
    /// unnecessary or ill-advised work:
    ///
    /// - If this crate is compiled in `no_std` compatible mode, `Backtrace`
    ///   is an empty struct, and will be completely compiled away.
    /// - If this crate is run without the `RUST_BACKTRACE` environmental
    ///   variable enabled, the backtrace will not be generated at runtime.
    /// - Even if a backtrace is generated, the most expensive part of
    ///   generating a backtrace is symbol resolution. This backtrace does not
    ///   perform symbol resolution until it is actually read (e.g. by
    ///   printing it). If the Backtrace is never used for anything, symbols
    ///   never get resolved.
    ///
    /// Even with these optimizations, including a backtrace in your `Fail`ure
    /// may not be appropriate to your use case. You are not required to put a
    /// backtrace in a custom `Fail` type.
    ///
    /// > (We have detected that this crate was documented with no_std
    /// > compatibility turned on. The version of this crate that has been
    /// > documented here will never generate a backtrace.)
    pub struct Backtrace {
        _secret: (),
    }

    impl Backtrace {
        /// Construct a new backtrace. This will only create a real backtrace
        /// if the cate is compiled in std mode and the `RUST_BACKTRACE`
        /// environmental variable is activated.
        ///
        /// > (We have detected that this crate was documented with no_std
        /// > compatibility turned on. The version of this crate that has been
        /// > documented here will never generate a backtrace.)
        pub fn new() -> Backtrace {
            Backtrace { _secret: () }
        }
    }

    impl Default for Backtrace {
        fn default() -> Backtrace {
            Backtrace::new()
        }
    }

    impl Debug for Backtrace {
        fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
            Ok(())
        }
    }

    impl Display for Backtrace {
        fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
            Ok(())
        }
    }
}

with_std! {
    extern crate backtrace;

    mod internal;

    use self::internal::InternalBacktrace;

    /// A `Backtrace`.
    ///
    /// This is an opaque wrapper around the backtrace provided by
    /// libbacktrace. A variety of optimizations have been performed to avoid
    /// unnecessary or ill-advised work:
    ///
    /// - If this crate is compiled in `no_std` compatible mode, `Backtrace`
    ///   is an empty struct, and will be completely compiled away.
    /// - If this crate is run without the `RUST_BACKTRACE` environmental
    ///   variable enabled, the backtrace will not be generated at runtime.
    /// - Even if a backtrace is generated, the most expensive part of
    ///   generating a backtrace is symbol resolution. This backtrace does not
    ///   perform symbol resolution until it is actually read (e.g. by
    ///   printing it). If the Backtrace is never used for anything, symbols
    ///   never get resolved.
    ///
    /// Even with these optimizations, including a backtrace in your `Fail`ure
    /// may not be appropriate to your use case. You are not required to put a
    /// backtrace in a custom `Fail` type.
    pub struct Backtrace {
        internal: InternalBacktrace
    }

    impl Backtrace {
        /// Construct a new backtrace. This will only create a real backtrace
        /// if the cate is compiled in std mode and the `RUST_BACKTRACE`
        /// environmental variable is activated.
        pub fn new() -> Backtrace {
            Backtrace { internal: InternalBacktrace::new() }
        }

        pub(crate) fn none() -> Backtrace {
            Backtrace { internal: InternalBacktrace::none() }
        }
    }

    impl Default for Backtrace {
        fn default() -> Backtrace {
            Backtrace::new()
        }
    }

    impl Debug for Backtrace {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            if let Some(bt) = self.internal.as_backtrace() {
                bt.fmt(f)
            } else { Ok(()) }
        }
    }

    impl Display for Backtrace {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            if let Some(bt) = self.internal.as_backtrace() {
                bt.fmt(f)
            } else { Ok(()) }
        }
    }
}
