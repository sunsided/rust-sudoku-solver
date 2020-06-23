#[macro_export]
/// Create a **BTreeSet** from a list of elements.
/// Inspired by https://docs.rs/maplit/0.1.6/maplit/macro.btreeset.html.
///
/// ## Example
///
/// ```
/// # fn main() {
///
/// let set = join_btreeset!("a", "b");
/// assert!(set.contains("a"));
/// assert!(set.contains("b"));
/// assert!(!set.contains("c"));
/// # }
/// ```
macro_rules! join_btreeset {
    ($($key:expr,)+) => (btreeset!($($key),+));

    ( $($key:expr),* ) => {
        {
            let mut _set = ::std::collections::BTreeSet::new();
            $(
                _set.extend($key);
            )*
            _set
        }
    };
}

#[macro_export]
/// Create a **BTreeSet** from a list of elements.
/// Inspired by https://docs.rs/maplit/0.1.6/maplit/macro.btreeset.html.
///
/// ## Example
///
/// ```
/// # fn main() {
///
/// let set = hashset!("a", "b");
/// assert!(set.contains("a"));
/// assert!(set.contains("b"));
/// assert!(!set.contains("c"));
/// # }
/// ```
macro_rules! hashset {
    ($($key:expr,)+) => (hashset!($($key),+));

    ( $($key:expr),* ) => {
        {
            let mut _set = ::std::collections::HashSet::<_, ::std::collections::hash_map::RandomState>::new();
            $(
                _set.insert($key);
            )*
            _set
        }
    };
}
