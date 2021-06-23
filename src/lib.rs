/// Derives From<T> implementations for enums whose variants can be expressed by such foreign
/// types.
///
/// For example, this:
/// ```
/// mod serde_json {
///     pub struct Error;
/// }
///
/// enum MyError {
///     SerializationError(serde_json::Error),
/// }
///
/// impl From<serde_json::Error> for MyError {
///     fn from(e: serde_json::Error) -> Self {
///         Self::SerializationError(e)
///     }
/// }
/// ```
/// is condensed using convertable-errors into this:
/// ```
/// # use convertable_errors::convertable_error;
///
/// mod serde_json {
///     # #[derive(PartialEq, Debug, Clone, Copy)]
///     pub struct Error;
/// }
///
/// convertable_error! {
///     # #[derive(PartialEq, Debug)]
///     enum MyError {
///         (SerializationError(serde_json::Error), [(serde_json::Error, Self::SerializationError)])
///     }
/// }
/// #
/// # let err = serde_json::Error;
/// # let my_err: MyError = err.into();
/// # assert_eq!(my_err, MyError::SerializationError(err));
/// ```
/// 
/// The syntax for defining a convertable enum with convertable-errors is as follows:
/// - Each variant of an enum must be wrapped in a tuple: `enum MyError { (Variant(ForeignType)), (Variant1) }`
/// - The first member of the tuple represents your variant. At the moment, only tuple variants and
/// unit variants are supported bc I'm a lazy fuck.
/// - The second member of the tuple (optional) represents the types that can be converted into
/// that variant: `enum MyError { (Variant(ForeignType), [ ... ]), (Variant1) }`
/// - The members of the convertable types array are each tuples representing the foreign type that
/// can be converted into your enum and the closure or variant to apply the foreign value to:
/// `[(ForeignType, Self::Variant)]`. Internally, this second member can be a closure `|x|
/// Self::Variant(x)`, a unit variant closure `|_| Self::Variant1`, or simply a variant identifier
/// where the value of the foreign type will be stored: `Self::Variant`. In practice, you can use
/// this macro for any enum, but I find it most useful for Error-like enums.
///
/// NOTE: This isn't a serious project, I might have made some mistakes, so feel free to open a PR
/// :) This is just a helpful snippet that I use and felt like sharing.
#[macro_export]
macro_rules! convertable_error {
    // A variant can have foreign types it can be derived from. Each of these may have a converter
    (
        $(#[$supermeta:meta])*
        $v:vis enum $name:ident {
            $(
                $(#[$meta:meta])*
                ($variant:ident$(($($field:ty),*))?$(, $equivalents:tt)?)
            ),+$(,)?
        }
    ) => {
        $(#[$supermeta])*
        $v enum $name {
            $($(#[$meta])*$variant$(($($field),*))?),+
        }

        // Build From<Foreign> impls for each of the variants
        $(convertable_error!(@from $name, ($variant$(, $equivalents)?));)+
    };

    (@from $name:ident, ($variant:ident, [$(($ftype:ty, $converter:expr)),+])) => {
        $(impl From<$ftype> for $name {
            fn from(v: $ftype) -> Self {
                $converter(v)
            }
        })+
    };
    // function f(v: ForeignType) -> Self, or simply be type-to-type
    (@from $name:ident, ($variant:ident, [$($ftype:ty),+])) => {
        $(impl From<$ftype> for $name {
            fn from(v: $ftype) -> Self {
                Self::$variant(v)
            }
        })+
    };
    // A variant of an error without any equivalent foreign types needs no further computation.
    (@from $name:ident, ($variant:ident)) => {};
}
