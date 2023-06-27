use serde::Deserialize;

use crate::{
    client::Client,
    config::Config,
    error::OpenAIError,
    types::{CompletionResponseStream, CreateCompletionRequest, CreateCompletionResponse},
};

/// Given a prompt, the model will return one or more predicted
/// completions, and can also return the probabilities of alternative
/// tokens at each position.
pub struct Completions<'c, C: Config> {
    client: &'c Client<C>,
}

#[derive(Debug, Deserialize)]
pub struct NECreateCompletionResponse {
    /// 返回结果状态，000000表示成功
    pub status: String,
    /// 返回结果描述
    pub desc: Option<String>,
    /// 返回数据详情
    pub detail: Option<CreateCompletionResponse>,
}

impl<'c, C: Config> Completions<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self { client }
    }

    /// Creates a completion for the provided prompt and parameters
    pub async fn create(
        &self,
        request: CreateCompletionRequest,
    ) -> Result<NECreateCompletionResponse, OpenAIError> {
        if request.stream.is_some() && request.stream.unwrap() {
            return Err(OpenAIError::InvalidArgument(
                "When stream is true, use Completion::create_stream".into(),
            ));
        }
        self.client.post("/api/v2/text/completion", request).await
    }

    /// Creates a completion request for the provided prompt and parameters
    ///
    /// Stream back partial progress. Tokens will be sent as data-only
    /// [server-sent events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#event_stream_format)
    /// as they become available, with the stream terminated by a data: \[DONE\] message.
    ///
    /// [CompletionResponseStream] is a parsed SSE stream until a \[DONE\] is received from server.
    pub async fn create_stream(
        &self,
        mut request: CreateCompletionRequest,
    ) -> Result<CompletionResponseStream, OpenAIError> {
        if request.stream.is_some() && !request.stream.unwrap() {
            return Err(OpenAIError::InvalidArgument(
                "When stream is false, use Completion::create".into(),
            ));
        }

        request.stream = Some(true);

        Ok(self.client.post_stream("/completions", request).await)
    }
}
