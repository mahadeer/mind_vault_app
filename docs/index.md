---
layout: home
title: Home
nav_order: 1
---

# MindVault - Personal Knowledge Management System

Welcome to MindVault, a comprehensive personal knowledge management system designed to store and categorize all your personal notes, tasks, reportee reviews, and reminders for easy access. Built with Rust, Axum, and MongoDB, MindVault provides powerful organization capabilities with both standalone UI and LLM integration options.

## 🚀 Quick Start

Get started with MindVault in just a few steps:

1. **[Getting Started](getting_started.html)** - Installation and setup guide
2. **[How to Run](how_to_run.html)** - Running the application
3. **[Features](features.html)** - Explore all available features
4. **[Usage Examples](usage_examples.html)** - See practical examples and LLM integration

## 🏗️ Architecture & Design

- **[Architecture Overview](architecture_overview.html)** - System design and components
- **[Tech Stack](tech_stack.html)** - Technologies and frameworks used
- **[Project Layout](project_layout.html)** - Code organization and structure

## 📋 Key Features

### 📝 **Personal Notes Management**
- Store and organize thoughts, ideas, and observations
- Rich text support with categorization
- Tag-based organization for easy retrieval
- Full-text search across all notes

### ✅ **Task Management**
- Create, read, update, and delete tasks
- Priority levels and due date tracking
- Status management (Pending, In Progress, Completed)
- Bulk operations for efficiency

### 👥 **Reportee Reviews**
- Track performance reviews and feedback
- Historical review data management
- Structured feedback organization
- Progress tracking over time

### ⏰ **Reminders & Notifications**
- Set reminders for important dates
- Meeting and deadline notifications
- Follow-up tracking
- Recurring reminder support

### 🔌 **Dual Integration Options**
- **Standalone UI**: Dedicated web interface for direct interaction
- **LLM Integration**: MCP server support for AI assistant integration
- **API Access**: RESTful APIs for custom integrations

### 🛡️ **Data Integrity & Security**
- Comprehensive input validation
- Soft delete functionality with data preservation
- Consistent error handling
- Auto-increment ID generation

## 🎯 Use Cases

### 👤 **Personal Knowledge Management**
- **Daily Notes**: Capture thoughts, ideas, and observations
- **Task Organization**: Manage personal and professional to-dos
- **Meeting Notes**: Store and organize meeting minutes and action items
- **Learning Journal**: Track learning progress and insights

### 👥 **Team Management**
- **Reportee Reviews**: Conduct and track performance evaluations
- **Feedback Management**: Organize constructive feedback and development plans
- **Team Notes**: Share insights and observations about team members
- **Goal Tracking**: Monitor individual and team objectives

### 🤖 **AI Assistant Integration**
- **LLM-Powered Insights**: Use with Claude, ChatGPT, or other AI assistants
- **Smart Categorization**: AI-assisted tagging and organization
- **Intelligent Search**: Natural language queries for information retrieval
- **Automated Summaries**: Generate insights from stored data

### 🔗 **Workflow Integration**
- **API Integration**: Connect with existing tools and systems
- **Custom Workflows**: Build personalized productivity systems
- **Data Export**: Extract information for reporting and analysis

## 📊 API Endpoints

### 📝 Notes Management
| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/notes` | Get all notes |
| `POST` | `/notes` | Create new note |
| `GET` | `/notes/{id}` | Get note by ID |
| `PUT` | `/notes/{id}` | Update note |
| `DELETE` | `/notes/{id}` | Delete note |
| `GET` | `/notes/search` | Search notes by content/tags |

### ✅ Tasks Management
| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/tasks` | Get all tasks |
| `POST` | `/tasks` | Create single task |
| `GET` | `/tasks/{id}` | Get task by ID |
| `PUT` | `/tasks/{id}` | Update task |
| `DELETE` | `/tasks/{id}` | Soft delete task |
| `POST` | `/tasks/bulk` | Bulk create tasks |
| `GET` | `/tasks/search` | Search and filter tasks |

### 👥 Reviews Management
| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/reviews` | Get all reviews |
| `POST` | `/reviews` | Create new review |
| `GET` | `/reviews/{id}` | Get review by ID |
| `PUT` | `/reviews/{id}` | Update review |
| `GET` | `/reviews/reportee/{id}` | Get reviews for specific reportee |

### ⏰ Reminders Management
| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/reminders` | Get all reminders |
| `POST` | `/reminders` | Create new reminder |
| `GET` | `/reminders/{id}` | Get reminder by ID |
| `PUT` | `/reminders/{id}` | Update reminder |
| `GET` | `/reminders/due` | Get due reminders |

## 🔧 Technology Stack

- **Backend**: Rust with Axum framework for high performance
- **Database**: MongoDB with efficient indexing and flexible schema
- **Frontend**: Modern web UI (planned) for standalone access
- **Integration**: MCP (Model Context Protocol) server support
- **API**: RESTful APIs with comprehensive validation
- **Testing**: Postman collection with automated tests
- **Documentation**: GitHub Pages with Jekyll
- **Deployment**: Docker support for easy deployment

## 📚 Documentation Sections

<div class="grid-container">
  <div class="grid-item">
    <h3><a href="getting_started.html">🚀 Getting Started</a></h3>
    <p>Installation, setup, and first steps</p>
  </div>
  
  <div class="grid-item">
    <h3><a href="features.html">✨ Features</a></h3>
    <p>Complete feature overview and capabilities</p>
  </div>
  
  <div class="grid-item">
    <h3><a href="architecture_overview.html">🏗️ Architecture</a></h3>
    <p>System design and component overview</p>
  </div>
  
  <div class="grid-item">
    <h3><a href="tech_stack.html">⚙️ Tech Stack</a></h3>
    <p>Technologies and frameworks used</p>
  </div>
  
  <div class="grid-item">
    <h3><a href="implementation.html">📋 Implementation</a></h3>
    <p>Development progress and milestones</p>
  </div>
  
  <div class="grid-item">
    <h3><a href="usage_examples.html">💡 Usage Examples</a></h3>
    <p>Practical examples and LLM integration</p>
  </div>
  
  <div class="grid-item">
    <h3><a href="project_layout.html">📁 Project Layout</a></h3>
    <p>Code organization and structure</p>
  </div>
  
  <div class="grid-item">
    <h3><a href="how_to_run.html">▶️ How to Run</a></h3>
    <p>Running and testing the application</p>
  </div>
</div>

## 🤝 Contributing

We welcome contributions! Please see our contributing guidelines and feel free to submit issues and pull requests.

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

---

<div class="footer-note">
  <p><strong>MindVault</strong> - Your personal knowledge management system for notes, tasks, reviews, and reminders</p>
</div>
