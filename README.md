
-----

# MindVault: Your Personal, Intelligent Knowledge Management System

MindVault is a robust, local, and intelligent knowledge management system built with **Rust**. It's designed to help you organize and gain insights from your personal notes, tasks, reportee reviews, to-dos, and reminders. MindVault is composed of several integrated components to provide a comprehensive solution for managing your personal data and leveraging the power of Large Language Models (LLMs).

-----


## Architecture Overview

The MindVault ecosystem consists of three main parts:

1.  **`mindvault-api`** (Axum-based Rust server):

    * Provides a REST + MCP-compatible API for all data operations, including comments, notes, and tasks.
    * Supports filtering data by keywords and time ranges.
    * Returns structured data optimized for LLM summarization and analysis.

2.  **`mindvault-ui`** (Dioxus-based web UI):

    * A web-based user interface for entering feedback, managing notes, and viewing summaries.
    * Interacts with `mindvault-api` via REST.

3.  **`mindvault-mcp`** (Rust MCP SDK project):

    * Utilizes the `rust-mcp` SDK to register tools with LLMs like LLM.
    * Internally calls `mindvault-api` to retrieve and format data.
    * Transforms the retrieved data into structured prompts suitable for LLMs.


![Architecture Diagram](docs/assets/diagram.svg)

-----

## Features

* **Centralized Data Management:** Store and categorize all your personal notes, tasks, reportee reviews, and reminders for easy access.
* **LLM-Ready Data Export:** Generate structured JSON data from your notes and comments, designed for consumption by any supported Large Language Model (LLM) or AI tool for summarization and analysis.
* **Task & Reminder Tracking:** Keep track of your to-dos and receive timely notifications for important events (planned).
* **Local & Private:** Designed to run entirely on your local system, ensuring your data remains private and secure.
* **High Performance:** Built with Rust for efficiency, reliability, and speed.
* **Flexible Filtering:** The API supports filtering comments and other data by `person_name`, `created_at` (date ranges), and `keywords`.


-----

## Getting Started

These instructions will guide you through setting up and running MindVault on your local machine.

### Installation

1.  **Clone the Repository:**

    ```bash
    git clone https://github.com/mahadeer/mind_vault_app.git
    cd mind_vault_app
    ```

2.  **Build the Project:**
    Navigate to the `mind_vault_app` directory and build the projects.

    ```bash
    cargo build --release --package mindvault-mcp
    ```

    This will create optimized executables for `mindvault-tool` in their respective `target/release/` directories.


-----

## Running MindVault

MindVault can be run as a monolithic application or integrated with external tools.

#### Running as a Monolithic Application

If you're running the `mindvault-api` directly, you would typically start it from its `target/release/` directory:

```bash
./target/release/mindvault-api
```

```bash
./target/release/mindvault-ui
```

You would then interact with it via its API endpoints or the `frontend` UI.

#### Integration with External Tools (MCP Servers)

MindVault is designed to be launched and managed by external tools or workflows configured to interact with "MCP servers." If your external tool expects a configuration to launch and interact with an MCP server, you can configure it to run MindVault's API as follows:

```json
{
  "mcpServers": {
    "mindvault-local": {
      "command": "path/to/target/release/mindvault_mcp"
    }
  }
}
```

-----


## Usage Examples (Input Text)

1.  "Add a note for Alice: 'Met with Alice, she mentioned a new client project starting next week.' Tag it as 'meeting' and 'client'."
2.  "Show me all notes about Bob."
3.  "Summarize Bob's feedback from last month."
4.  "Find notes tagged 'bug' and 'frontend'."
5.  "Update note ID 1234: 'Added crucial details about client requirements.'"
6.  "Remove the note with ID 5678."
7.  "Search all notes for 'performance review'."
8.  "What's the general sentiment of feedback for Alice over the last quarter?"
9.  "What tags are available for notes?"
10. "Give me a daily summary of all new notes."

-----

## Tech Stack

* **Language:** <img src="https://cdn.simpleicons.org/rust/green" alt="Rust" width="15" height="15"/> Rust
* **Backend:** <img src="https://avatars.githubusercontent.com/u/20248544?s=48&v=4" alt="Axum" width="15" height="15"/> Axum
* **Database:** <img src="https://cdn.simpleicons.org/mongodb/green" alt="MongoDB" width="15" height="15"/> MongoDB
* **Frontend:** <img src="https://avatars.githubusercontent.com/u/79236386?s=48&v=4" alt="Dioxus" width="15" height="15"/> Dioxus (WASM/React)
* **LLM Integration:** <img src="https://cdn.simpleicons.org/claude/DDDD11" alt="OpenAI" width="15" height="15"/> Claude or LM Studio via `mindvault_mcp`



-----

## Project Layout

Here's a simplified overview of the project's folder structure:

```
📄 Cargo.toml
📁 core/
  ├── 📄 Cargo.toml
  ├── 📂 src/
  │   ├── 📄 config.rs
  │   ├── 📁 db/
  │   ├── 📄 lib.rs
  │   ├── 📄 models.rs
  │   ├── 📁 repository/
📁 mcp/
  ├── 📄 Cargo.toml
  ├── 📂 src/
  │   ├── 📄 main.rs
  │   ├── 📄 models.rs
  │   ├── 📁 tools/
  │   ├── 📄 utils.rs
📄 README.md
📁 server/
  ├── 📄 Cargo.toml
  ├── 📂 src/
  │   ├── 📁 helpers/
  │   ├── 📄 main.rs
  │   ├── 📁 routes/
  │   ├── 📁 services/
  │   ├── 📁 templates/
📁 shared/
  ├── 📄 Cargo.toml
  ├── 📂 src/
  │   ├── 📄 formatters.rs
  │   ├── 📄 lib.rs
  │   ├── 📄 logger.rs
  │   ├── 📁 models/
```


-----

<div style="
    background-color: #fff8dc; /* Cornsilk - a soft yellow */
    border-left: 5px solid #ffc107; /* Amber yellow border */
    padding: 15px;
    margin: 15px 0;
    border-radius: 5px;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1); /* Subtle shadow */
">
    <p style="margin: 0; font-weight: bold; color: #333;">
        💡 Note: <span style="font-weight: normal;">Doc generated using a custom doc_builder, refer to <code>docs\doc_builder</code></span>
    </p>
</div>


## Contributing

MindVault is currently designed for personal use. However, if you have suggestions or want to contribute, feel free to open an issue or submit a pull request\!

-----

## License

This project is licensed under the MIT License—see the `LICENSE` file for details.

-----
