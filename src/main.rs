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

//! Binary entry point: parse CLI flags, run any auxiliary subcommand
//! (completions, manuals), otherwise build the [`tui::model::Model`]
//! from config or wizard and hand it to [`tui::app::run`].

mod cli;
mod config;
mod tui;
#[cfg(all(feature = "imap", feature = "smtp", feature = "jmap"))]
mod wizard;

use clap::Parser;
use pimalaya_cli::{error::ErrorReport, printer::StdoutPrinter};

use crate::{cli::Cli, tui::app};

fn main() {
    let cli = Cli::parse();
    let mut printer = StdoutPrinter::new(&cli.json);

    if let Some(command) = cli.command {
        let result = command.execute(&mut printer);
        return ErrorReport::eval(&mut printer, result);
    }

    let result = cli.try_into_tui_model();
    let model = ErrorReport::eval(&mut printer, result);

    let result = app::run(model);
    ErrorReport::eval(&mut printer, result);
}
