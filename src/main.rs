use clap::Parser;
use ollama_rs::{generation::completion::request::GenerationRequest, Ollama};

mod cli;

use cli::Cmd;
use tokio::io::AsyncWriteExt;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let args = Cmd::parse();
    call_llm(args.prompt).await;
}

async fn call_llm(prompt: String) {
    let ollama = Ollama::default();
    let model = "gemma2:2b".to_string();
    let prompt = format!(r#"
        Create a taskwarrior cli command from this prompt: '{}'. 
        The response NEEDS to be only the task warrior command the user wantes to implement.
        Assume the taskwarror CLI is called `task`.
        ALWAYS reply with valid json, DO NOT wrap the json in backticks or codeblocks.

        Here are some good examples:
        
          Prompt: 'take out the trash every Sunday night at 7pm.'
          Response: {{"command": "task add 'take out the trash' due:sunday at 19:00 recur:weekly"}}

          Prompt: 'Show all work tasks due by Friday.'
          Response: {{ "command": "task context:work due:friday or due.before:friday"}}

        "#, prompt);

    let mut stream = ollama
        .generate_stream(GenerationRequest::new(model, prompt))
        .await
        .unwrap();

    let mut stdout = tokio::io::stdout();
    while let Some(res) = stream.next().await {
        let responses = res.unwrap();
        for resp in responses {
            stdout.write(resp.response.as_bytes()).await.unwrap();
            stdout.flush().await.unwrap();
        }
    }
}
