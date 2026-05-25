// This file is part of Himalaya TUI, a TUI to manage emails.
//
// Copyright (C) 2025-2026  soywod <pimalaya.org@posteo.net>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! Terminal UI built on the Elm Architecture (Model-Update-View).
//!
//! - [`model`] owns every piece of state, including the
//!   `EmailClientStd`, and defines the [`model::Message`] enum that
//!   names each transition.
//! - [`update`] is a pure function from `(Model, Message)` to a new
//!   `Model` plus an optional follow-up `Message`. All I/O lives
//!   here; nothing else mutates state.
//! - [`view`] renders the current `Model` to a ratatui [`Frame`]; it
//!   never produces messages or mutates anything user-visible.
//! - [`app`] drives the loop: poll events, fold the resulting message
//!   chain through `update`, then redraw via `view`.
//!
//! Why TEA: a single state container plus a single transition
//! function makes the data flow easy to follow, eliminates ad-hoc
//! callbacks, and keeps rendering side-effect-free. The pattern
//! scales from a counter demo to a multi-pane email client without
//! changing shape.
//!
//! References:
//! - <https://ratatui.rs/concepts/application-patterns/the-elm-architecture/>
//! - <https://guide.elm-lang.org/architecture/>
//!
//! [`Frame`]: ratatui::Frame

pub mod app;
pub mod model;
pub mod theme;
pub mod update;
pub mod view;
