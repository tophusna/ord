#![allow(clippy::type_complexity)]

use {
  self::{command_builder::CommandBuilder, expected::Expected},
  executable_path::executable_path,
  nix::{sys::signal::Signal, unistd::Pid},
  pretty_assertions::assert_eq as pretty_assert_eq,
  regex::Regex,
  reqwest::{StatusCode, Url},
  std::{
    fs,
    net::TcpListener,
    os::unix::process::ExitStatusExt,
    process::Child,
    process::{Command, Stdio},
    str, thread,
    time::Duration,
  },
  tempfile::TempDir,
  test_server::TestServer,
  unindent::Unindent,
};

macro_rules! assert_regex_match {
  ($string:expr, $pattern:expr $(,)?) => {
    let regex = Regex::new(&format!("^(?s){}$", $pattern)).unwrap();
    let string = $string;

    if !regex.is_match(string.as_ref()) {
      panic!(
        "Regex:\n\n{}\n\n…did not match string:\n\n{}",
        regex, string
      );
    }
  };
}

mod command_builder;
mod epochs;
mod expected;
mod find;
mod index;
mod info;
mod list;
mod parse;
mod range;
mod rune;
mod server;
mod supply;
mod test_server;
mod traits;
mod version;
mod wallet;
