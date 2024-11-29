# LLM API

A command-line interface and API server for interacting with various Language Learning Models (LLMs) including OpenAI and Anthropic.

## Installation

Download the latest binary for your platform from the releases page.

## Configuration

Set the following environment variables for the LLMs you want to use:

### OpenAI
- `OPENAI_API_KEY` - Your OpenAI API key

### Anthropic
- `ANTHROPIC_API_KEY` - Your Anthropic API key

## Usage

The application can be run in two modes: chat (interactive) or api (server).

```bash
llmapi-rust <mode> [options]

Options:
  --set-port <PORT>  Set the port number for the API server
  -h, --help        Print help
  -V, --version     Print version
```

### Chat Mode

Start an interactive chat session:

```bash
llmapi-rust chat
```

Available commands in chat mode:
- `/list` - List all available models
- `/select <model_name>` - Select a model to chat with
- `/exit` - Exit the chat

Example session:
```
Chat mode started. Available commands:
  /list              - List all available models
  /select <n>     - Select a model by name
  /exit              - Exit the chat

> /list
Available models:
- gpt-4o (openai / gpt-4o)
- gpt-4o-mini (openai / gpt-4o-mini)
- Sonnet 3.5 (anthropic / claude-3-5-sonnet-latest)

> /select gpt-4o
Selected model: gpt-4o (openai)

> Hello, how are you?
I'm doing well, thank you for asking...
```

### API Server Mode

Start the API server:

```bash
llmapi-rust api [--set-port <PORT>]
```

The server runs by default on port 3000. You can change the port using the `--set-port` option.

## API Endpoints

### Query Endpoint

`POST /query`

Send a prompt to the selected model.

**Request Body:**
```json
{
    "ModelName": "gpt-4o",
    "Prompt": "Hello, how are you?"
}
```

**Response:**
```json
{
    "Response": "Hello! I'm doing well, thank you for asking..."
}
```

### Models Endpoint

`GET /models`

List all available models.

**Response:**
```json
[
    {
        "ModelName": "gpt-4o",
        "Provider": "openai"
    },
    {
        "ModelName": "gpt-4o-mini",
        "Provider": "openai"
    },
    {
        "ModelName": "Sonnet 3.5",
        "Provider": "anthropic"
    }
]
```

## Error Handling

The API uses standard HTTP status codes:
- 200: Success
- 400: Bad Request
- 401: Unauthorized
- 404: Not Found
- 500: Internal Server Error

Error responses include a message explaining what went wrong:
```json
{
    "error": {
        "message": "Invalid request: model field is required",
        "type": "invalid_request_error"
    }
}
