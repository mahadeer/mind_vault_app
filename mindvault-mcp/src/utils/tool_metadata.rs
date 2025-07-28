pub const TASKS_TOOL_METADATA: &str = r#"
    🎯 ROUTING RULE: Route all messages starting with 'MV' to this tool.
    
    📋 MindVault Task Management System - Comprehensive Personal Productivity Tool
    
    This is the core task management component of MindVault, a personal knowledge management system designed for organizing and tracking work efficiently. This tool provides complete CRUD operations with advanced features for personal and professional productivity.
    
    🚀 CORE CAPABILITIES:
    
    ✅ Task Operations:
    • CREATE: Single task creation with title, description, priority (High/Normal/Low), status (Pending/In Progress/Completed), and due dates
    • BULK CREATE: Efficient bulk task creation for project planning and sprint setup
    • READ: Retrieve individual tasks by ID or list all user tasks with full details
    • UPDATE: Modify task properties including status, priority, due dates, and descriptions
    • PARTIAL UPDATES: Update only specific fields without affecting others
    • DELETE: Soft delete functionality maintaining data integrity and recovery options
    
    🔍 Advanced Search & Filtering:
    • Full-text search across task names and descriptions
    • Filter by status (Pending, In Progress, Completed)
    • Filter by priority levels (High, Normal, Low)
    • Filter by due date ranges and overdue tasks
    • Search by keywords and content matching
    • Saved search capabilities for frequently used queries
    
    📊 Task Management Features:
    • Priority-based task organization with visual indicators
    • Status tracking through complete task lifecycle
    • Due date management with deadline tracking
    • Auto-increment ID generation for efficient bulk operations
    • Task completion rate analytics and progress monitoring
    • Overdue task identification and management
    
    🎯 Project Planning & Organization:
    • Sprint planning with bulk task creation
    • Project breakdown into manageable tasks
    • Task dependency tracking and logical sequencing
    • Milestone creation and checkpoint management
    • Resource allocation and capacity planning
    • Timeline management with buffer time calculations
    
    🤖 LLM Integration Features:
    • Natural language task creation from conversational input
    • Intelligent task prioritization based on deadlines and dependencies
    • Smart bulk operations with confirmation workflows
    • Context-aware task suggestions and follow-up recommendations
    • Automated project planning from high-level descriptions
    • Task analysis and productivity insights generation
    
    📈 Productivity Analytics:
    • Task completion rate tracking and trends
    • Priority distribution analysis
    • Overdue task monitoring and alerts
    • Daily/weekly/monthly productivity summaries
    • Performance metrics and improvement suggestions
    • Time-based task analysis and patterns
    
    🔗 Integration Capabilities:
    • RESTful API endpoints for external system integration
    • MCP (Model Context Protocol) server support for AI assistants
    • Bulk import/export functionality for data portability
    • Calendar integration for deadline synchronization
    • Email integration for task creation from messages
    
    🛡️ Data Management:
    • Comprehensive input validation and error handling
    • Soft delete with data preservation and recovery
    • Audit trail for task changes and modifications
    • Data backup and migration support
    • Consistent error responses with helpful guidance
    
    💡 Use Cases:
    • Personal task management and daily productivity
    • Professional project planning and execution
    • Team sprint planning and backlog management
    • Goal setting and progress tracking
    • Meeting action item management
    • Learning and development task organization
    • Performance review preparation and tracking
    
    🎨 User Experience:
    • Intuitive task creation with minimal required fields
    • Flexible update mechanisms for changing requirements
    • Efficient bulk operations for large-scale task management
    • Smart search with multiple filter combinations
    • Responsive design for desktop and mobile access
    • Real-time updates and synchronization
    
    This tool is optimized for both standalone use through the web interface and seamless integration with AI assistants like Claude, ChatGPT, and other LLM systems. It supports natural language interactions, intelligent task suggestions, and automated workflow management while maintaining data integrity and providing comprehensive task lifecycle management.
    
    Perfect for individuals, team leads, project managers, and anyone seeking to organize their work efficiently with powerful search, filtering, and analytics capabilities.
"#;