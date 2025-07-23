
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
