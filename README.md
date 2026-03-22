# Vel'Koz

This is an experimental CLI tool for non-directive therapy. It is simultaneously an excuse to put Rust on my CV and an exploration into a topic I found interesting. It currently supports using Google's Gemini LLM and user hosted LLMs for complete privacy.

Note: It has not tested on linux or mac. If you encounter difficulties on those systems feel free to fork & raise pull requests.

## Prerequisites

1. Install Rust.

See https://rust-lang.org/tools/install/

2. (Optional - local runs) Install Ollama

See https://ollama.com/download

## How to use

1. Clone the repository to your local machine:

```
git clone https://github.com/f-okd/velkoz
```

2. Navigate to the project directory:

```
cd C:\...\velkoz
```

3. Add environment varaibles in `.\env`. The project will not work without this file.

4. Run the project

```
cargo run
# alternatively: cargo run -- --gemini
```

### Commands

**CLI arguments**

- `--help`: Displays help message
- `--gemini`: Starts a chat using the Gemini LLM
- `--local`: Starts a chat using the locally configured LLM

**In-chat commands**

- `"/save <path>"`: Saves conversation history to a file at specifed path.
- `"/load <path>"`: Loads converstaion history from a file at specified path. Overwrites current session.

### Environment Variables & Configuration

**Set Gemini API key**

Update `./env`:

```
GEMINI_API_KEY="<API_KEY>"
```

**Configure Ollama model**

Phi4-mini is recommended as a good consumer-range model. Pick a model from https://ollama.com/library and install it locally with:

```
ollama pull <model_name>
```

Afterwards, set model name in `.env`:

```
OLLAMA_MODEL=<model_name>
```

Before trying to run with `--local`, be sure to start local server running your model with:

```
ollama
```

Local mode performance depends heavily on hardware, and that users with a GPU will have a much better experience.

See more: https://docs.ollama.com/quickstart

n.b. _I dont even play Vel'Koz , [I was an Irelia & Camille 2TP](https://op.gg/lol/summoners/euw/burton%20guster-koods) but it seemed like an interesting & lore appropriate name_
