//! A full-featured framework that empowers you to easily build [Telegram bots]
//! using [Rust]. It handles all the difficult stuff so you can focus only on
//! your business logic.
//!
//! For a high-level overview, see [our GitHub repository](https://github.com/teloxide/teloxide).
//!
//! [[`examples/throw_dice.rs`](https://github.com/teloxide/teloxide/blob/master/examples/throw_dice.rs)]
//! ```no_run
//! use teloxide::prelude::*;
//!
//! # #[tokio::main]
//! # async fn main() {
//! pretty_env_logger::init();
//! log::info!("Starting throw dice bot...");
//!
//! let bot = Bot::from_env().auto_send();
//!
//! teloxide::repl(bot, |message: Message, bot: AutoSend<Bot>| async move {
//!     bot.send_dice(message.chat.id).await?;
//!     respond(())
//! })
//! .await;
//! # }
//! ```
//!
//! <div align="center">
//!   <kbd>
//!     <img src=https://github.com/teloxide/teloxide/raw/master/media/throw-dice.gif width=420px />
//!   </kbd>
//! </div>
//!
//! [Telegram bots]: https://telegram.org/blog/bot-revolution
//! [`async`/`.await`]: https://rust-lang.github.io/async-book/01_getting_started/01_chapter.html
//! [Rust]: https://www.rust-lang.org/

// This hack is used to cancel formatting for a Markdown table. See [1], [2], and [3].
//
// [1]: https://github.com/rust-lang/rustfmt/issues/4210
// [2]: https://github.com/rust-lang/rustfmt/issues/4787
// [3]: https://github.com/rust-lang/rust/issues/82768#issuecomment-803935643
#![cfg_attr(feature = "nightly", cfg_attr(feature = "nightly", doc = include_str!("features.txt")))]
// https://github.com/teloxide/teloxide/raw/master/logo.svg doesn't work in html_logo_url, I don't know why.
#![doc(
    html_logo_url = "https://github.com/teloxide/teloxide/raw/master/ICON.png",
    html_favicon_url = "https://github.com/teloxide/teloxide/raw/master/ICON.png"
)]
// We pass "--cfg docsrs" when building docs to add `This is supported on
// feature="..." only.`
//
// "--cfg dep_docsrs" is used for the same reason, but for `teloxide-core`.
//
// To properly build docs of this crate run
// ```console
// $ RUSTFLAGS="--cfg dep_docsrs" RUSTDOCFLAGS="--cfg docsrs -Znormalize-docs" cargo +nightly doc --open --all-features
// ```
#![cfg_attr(all(docsrs, feature = "nightly"), feature(doc_cfg, doc_auto_cfg))]
#![forbid(unsafe_code)]
#![warn(rustdoc::broken_intra_doc_links)]
#![allow(clippy::match_bool)]
#![allow(clippy::redundant_pattern_matching)]
// https://github.com/rust-lang/rust-clippy/issues/7422
#![allow(clippy::nonstandard_macro_braces)]

#[cfg(feature = "ctrlc_handler")]
pub use dispatching::repls::{
    commands_repl, commands_repl_with_listener, repl, repl_with_listener,
};

mod logging;

pub mod dispatching;
pub mod error_handlers;
pub mod prelude;
pub mod utils;

#[doc(inline)]
pub use teloxide_core::*;

#[cfg(feature = "macros")]
pub use teloxide_macros as macros;

pub use dispatching::filter_command;
pub use dptree::{self, case as handler};

#[cfg(all(feature = "nightly", doctest))]
#[cfg_attr(feature = "nightly", cfg_attr(feature = "nightly", doc = include_str!("../README.md")))]
enum ReadmeDocTests {}

use teloxide_core::requests::ResponseResult;

/// A shortcut for `ResponseResult::Ok(val)`.
pub fn respond<T>(val: T) -> ResponseResult<T> {
    ResponseResult::Ok(val)
}
