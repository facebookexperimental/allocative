/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under both the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree and the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree.
 */

use crate::Visitor;

/// This trait allows traversal of object graph.
///
/// # Proc macro
///
/// Typically implemented with proc macro. Like this:
///
/// ```
/// use allocative::Allocative;
///
/// #[derive(Allocative)]
/// struct Foo {
///     x: u32,
///     y: String,
/// }
/// ```
///
/// Proc macro supports two attributes: `#[allocative(skip)]` and
/// `#[allocative(bound = "...")]`.
///
/// ## `#[allocative(skip)]`
///
/// `#[allocative(skip)]` can be used to skip field from traversal (for example,
/// to skip fields which are not `Allocative`, and can be skipped because they
/// are cheap).
///
/// ```
/// use allocative::Allocative;
///
/// /// This does not implement `Allocative`.
/// struct Unsupported;
///
/// #[derive(Allocative)]
/// struct Bar {
///     #[allocative(skip)]
///     unsupported: Unsupported,
/// }
/// ```
///
/// ## `#[allocative(bound = "...")]`
///
/// `#[allocative(bound = "...")]` can be used to overwrite the bounds that are
/// added to the generics of the implementation.
///
/// An empty string (`#[allocative(bound = "")]`) simply erases all bounds. It
/// adds all type variables found in the type to the list of generics but with
/// an empty bound. As an example
///
///
/// ```
/// use std::marker::PhantomData;
/// use allocative::Allocative;
///
/// struct Unsupported;
///
/// #[derive(Allocative)]
/// #[allocative(bound = "")]
/// struct Baz<T> {
///     _marker: PhantomData<T>,
/// }
/// ```
///
/// Would generate an instance
///
/// ```
/// impl<T> Allocative for Baz<T> { ... }
/// ```
///
/// Alternatively you can use the string to provide custom bounds. The string in
/// this case is used *verbatim* as the bounds, which affords great flexibility,
/// but also necessitates that all type variables must be mentioned or will be
/// unbound (compile error). As an example we may derive a size of a `HashMap`
/// by ignoring the hasher type.
///
///
/// ```
/// #[allocative(bound = "K: Allocative, V:Allocative, S")]
/// struct HashMap<K, V, S = RandomState> {
///    ...
/// }
/// ```
///
/// Which generates
///
/// ```
/// impl<K: Allocative, V:Allocative, S> Allocative for HashMap<K, V, S> {
///    ...
/// }
/// ```
pub trait Allocative {
    fn visit<'a, 'b: 'a>(&self, visitor: &'a mut Visitor<'b>);
}
