# Tasky

A natural language frontend for TaskWarrior powered by AI.

# Usage

First, install & start an [Ollama](https://github.com/ollama/ollama) server
with the `gemma2:2b` model available.

Then, just simply do `tasky "<prompt>"`, for example:

> Note: For now, Tasky just prints the LLM's response, You'll have to
> copy/paste the result yourself.

```
$ tasky "take out the trash every monday moring"
Generated:
  task add 'take out trash' due:monday at 07:00 recur:weekly
Confirm? [Y/n]
```

Additional options:
  - `-y`/`--auto-approve`

# To-dos:

Add "thinking" spinner before response, especially when the LLM is cold.

Add callout to `task` CLI. 

Implement `--auto-confirm` flag. 

Add configuration options for:
  - Model
  - Ollama server URL
  - Auto confirm when TaskWarrior command starts with set prefixes
    - For example, `task add ...` can probably be auto-confirmed, but `task rm ...` shouldn't be.
