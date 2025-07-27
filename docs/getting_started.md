---
layout: page
title: Getting Started
nav_order: 2
description: Installation and setup guide for MindVault Personal Knowledge Management System
---

# Getting Started

Welcome to **MindVault**, a comprehensive personal knowledge management system designed to store and categorize all your personal notes, tasks, reportee reviews, and reminders for easy access.

## 🎯 Project Vision

MindVault serves as your central hub for personal knowledge management, offering:

- **📝 Personal Notes**: Store and organize your thoughts, ideas, and observations
- **✅ Task Management**: Track your to-dos, projects, and deadlines
- **👥 Reportee Reviews**: Manage performance reviews and feedback for team members
- **⏰ Reminders**: Never forget important dates, meetings, or follow-ups
- **🔍 Easy Access**: Quick search and categorization for instant retrieval

## 🔌 Integration Options

MindVault is designed to work in multiple ways:

1. **🖥️ Standalone Web Application**: Use the dedicated UI for direct interaction
2. **🤖 LLM Integration**: Connect with any MCP (Model Context Protocol) server-supported applications
3. **🔗 API Access**: Integrate with your existing tools and workflows

## 🚀 Quick Setup

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
