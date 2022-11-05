/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under both the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree and the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree.
 */

// TODO(nga): features only on nightly.
#![feature(const_type_name)]
#![feature(never_type)]

mod flamegraph;
mod impls;
mod key;
mod measure;
mod rc_str;
mod test_derive;

pub use allocative_derive::Allocative;

pub use crate::flamegraph::FlameGraphBuilder;
pub use crate::key::Key;
pub use crate::measure::Allocative;
pub use crate::measure::Visitor;
