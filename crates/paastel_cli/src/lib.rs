// Copyright (c) 2024 Murilo Ijanc' <mbsd@m0x.ru>
//
// Permission to use, copy, modify, and distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

pub mod cmd;
pub mod error;
pub mod util;

use clap::{builder::ValueParser, Arg, Command};

use paastel_settings::{Location, Settings};

pub async fn execute() -> Result<(), error::Error> {
    util::init_tracing();

    let command = Command::new("paastel").arg(
        Arg::new("settings-file")
            .long("settings-file")
            .value_parser(ValueParser::new(util::parse_settings_var))
            .default_value(Location::default_path().into_os_string())
            .env("PAASTEL_SETTINGS")
            .help("Set path of settings file"),
    );
    let matches = command.clone().get_matches();
    let _settings = matches.get_one::<Settings>("settings-file").unwrap();

    Ok(())
}
