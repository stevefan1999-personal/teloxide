use crate::{
    dispatching::{Dispatcher, DispatcherHandlerRx, UpdateWithCx},
    error_handlers::OnError,
    requests::ResponseResult,
    types::Message,
    Bot,
};
use futures::StreamExt;
use std::{future::Future, sync::Arc};

/// A [REPL] for messages.
///
/// # Caution
/// **DO NOT** use this function together with [`Dispatcher`] and
/// [`commands_repl`], because Telegram disallow multiple requests at the same
/// time from the same bot.
///
/// [REPL]: https://en.wikipedia.org/wiki/Read-eval-print_loop
/// [`Dispatcher`]: crate::dispatching::Dispatcher
/// [`commands_repl`]: crate::dispatching::commands_repl
pub async fn repl<H, Fut>(bot: Bot, handler: H)
where
    H: Fn(UpdateWithCx<Message>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ResponseResult<()>> + Send + 'static,
{
    let handler = Arc::new(handler);

    Dispatcher::new(bot)
        .messages_handler(|rx: DispatcherHandlerRx<Message>| {
            rx.for_each_concurrent(None, move |message| {
                let handler = Arc::clone(&handler);

                async move {
                    handler(message).await.log_on_error().await;
                }
            })
        })
        .dispatch()
        .await;
}