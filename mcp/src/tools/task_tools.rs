use crate::models::{
    CreateUserTask, GetTaskById, SearchRequest, UpdateSearchRequest, UpdateUserTask,
};
use crate::utils::{as_content_string, get_content_from_response_task, get_content_from_tasks};
use reqwest::Client;
use rmcp::ErrorData as RmcpError;
use rmcp::handler::server::tool::{Parameters, ToolRouter};
use rmcp::model::{
    CallToolResult, Implementation, ProtocolVersion, ServerCapabilities, ServerInfo,
};
use rmcp::{ServerHandler, tool, tool_handler, tool_router};
use shared::models::task_model::Task;

#[derive(Debug)]
pub struct TaskTool {
    base_url: String,
    pub tool_router: ToolRouter<Self>,
}

#[tool_router]
impl TaskTool {
    pub(crate) fn new() -> TaskTool {
        TaskTool {
            base_url: "http://localhost:4500".to_string(),
            tool_router: Self::tool_router(),
        }
    }

    #[tool(
        name = "Get Tasks",
        description = "Get all available tasks for the user"
    )]
    pub async fn get_user_tasks(&self) -> Result<CallToolResult, RmcpError> {
        let client = Client::new(); // Create an HTTP client instance
        let get_all_tasks_url = format!("{}/tasks", self.base_url);
        let response = client.get(get_all_tasks_url).send().await;
        let tasks: Vec<Task> = response.unwrap().json().await.unwrap();
        get_content_from_tasks(tasks).await
    }

    #[tool(
        name = "Get Task by id",
        description = "Retrieves tasks based on optional filters such as status, title, date, or schedule. Can be used to list all tasks or only those matching certain criteria like 'pending', 'due today', or by keyword in title."
    )]
    pub async fn get_task_by_id(
        &self,
        Parameters(GetTaskById { task_id }): Parameters<GetTaskById>,
    ) -> Result<CallToolResult, RmcpError> {
        let client = Client::new(); // Create an HTTP client instance
        let get_task_by_id_url = format!("{}/tasks/{}", self.base_url, task_id);
        let response = client.get(get_task_by_id_url).send().await;
        let task: Task = response.unwrap().json().await.unwrap();
        let selected_task = as_content_string(vec![task]);
        Ok(CallToolResult::success(selected_task))
    }

    #[tool(
        name = "Create Task",
        description = "Creates a new user task. Requires a title. If no description is provided, one will be auto-generated from the title. If due_date or schedule contains natural date terms like 'today', 'tomorrow', or specific times, they will be parsed and used appropriately."
    )]
    pub async fn create_user_task(
        &self,
        Parameters(CreateUserTask {
            name,
            description,
            due_date,
            status,
            schedule,
        }): Parameters<CreateUserTask>,
    ) -> Result<CallToolResult, RmcpError> {
        let client = Client::new();
        let create_task_url = format!("{}/tasks", self.base_url);
        let user_new_task = CreateUserTask {
            name,
            description,
            status,
            schedule,
            due_date,
        };
        let response = client
            .post(create_task_url)
            .json(&user_new_task)
            .send()
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        get_content_from_response_task(response, "Error creating user task".to_string()).await
    }

    #[tool(
        name = "Update Task",
        description = "Updates an existing user task by id. Can update the title, description, due date, status, or schedule using task ID. If ID is not provided, task can be searched using title or status. Can also be used to bulk update tasks (e.g., mark all pending tasks as completed) when combined with filtering tools."
    )]
    pub async fn update_user_task(
        &self,
        Parameters(UpdateUserTask { task_id, user_task }): Parameters<UpdateUserTask>,
    ) -> Result<CallToolResult, RmcpError> {
        let client = Client::new();
        let update_task_url = format!("{}/tasks/{}", self.base_url, task_id);
        let response = client
            .put(update_task_url)
            .json(&user_task)
            .send()
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        get_content_from_response_task(response, "Error updating user task".to_string()).await
    }

    #[tool(
        name = "Delete Task",
        description = "Deletes a user task. Typically uses the task ID, but if only a title, date, or status is given, it will first search for the most relevant matching task and delete it."
    )]
    pub async fn delete_user_task(
        &self,
        Parameters(GetTaskById { task_id }): Parameters<GetTaskById>,
    ) -> Result<CallToolResult, RmcpError> {
        let client = Client::new();
        let delete_task_url = format!("{}/tasks/{}", self.base_url, task_id);
        let response = client
            .delete(delete_task_url)
            .send()
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;
        get_content_from_response_task(response, "Error deleting user task".to_string()).await
    }

    #[tool(
        name = "Search in Tasks",
        description = "Retrieves a list of tasks filtering by a search term present in either the task's name or its description."
    )]
    pub async fn search_tasks(
        &self,
        Parameters(SearchRequest { search_term }): Parameters<SearchRequest>,
    ) -> Result<CallToolResult, RmcpError> {
        let client = Client::new();
        let search_task_url = format!("{}/tasks/search?search_term={}", self.base_url, search_term);
        let response = client
            .get(search_task_url)
            .send()
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;
        let tasks: Vec<Task> = response.json().await.unwrap();
        get_content_from_tasks(tasks).await
    }

    #[tool(
        name = "Update Tasks by search text",
        description = "Retrieves a list of tasks by search term present in either the task's name or its description. and update existing user tasks matching the search term"
    )]
    pub async fn update_by_search_tasks(
        &self,
        Parameters(UpdateSearchRequest {
            search_term,
            user_task,
        }): Parameters<UpdateSearchRequest>,
    ) -> Result<CallToolResult, RmcpError> {
        let client = Client::new();
        let search_task_url = format!("{}/tasks/search?search_term={}", self.base_url, search_term);
        let response = client
            .post(search_task_url)
            .json(&user_task)
            .send()
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;
        let tasks: Vec<Task> = response.json().await.unwrap();
        get_content_from_tasks(tasks).await
    }

    #[tool(
        name = "Delete Tasks by search text",
        description = "Retrieves a list of tasks by a search term present in either the task's name or its description. and delete the existing user tasks"
    )]
    pub async fn delete_by_search_tasks(
        &self,
        Parameters(SearchRequest { search_term }): Parameters<SearchRequest>,
    ) -> Result<CallToolResult, RmcpError> {
        let client = Client::new();
        let search_task_url = format!("{}/tasks/search?search_term={}", self.base_url, search_term);
        let response = client
            .delete(search_task_url)
            .send()
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;
        let tasks: Vec<Task> = response.json().await.unwrap();
        get_content_from_tasks(tasks).await
    }
}

#[tool_handler]
impl ServerHandler for TaskTool {
    fn get_info(&self) -> ServerInfo {
        let tool_instruction = "This tool can provide a todo list for the user, this tool can update, delete, list and search task by id or name.";
        ServerInfo {
            protocol_version: ProtocolVersion::V_2025_03_26,
            instructions: Some(tool_instruction.to_string()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
        }
    }
}
