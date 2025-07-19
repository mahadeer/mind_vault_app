use reqwest::Response;
use rmcp::ErrorData as RmcpError;
use rmcp::model::CallToolResult;
use rmcp::model::Content;
use shared::models::task_model::Task;

pub async fn get_content_from_tasks(
    tasks: Vec<Task>
) -> Result<CallToolResult, RmcpError> {
    if tasks.is_empty() {
        let no_tasks = as_content_list_string(vec!["Task not found".to_string()]);
        Ok(CallToolResult::success(no_tasks))
    } else {
        Ok(CallToolResult::success(as_content_string(tasks)))
    }
}

pub async fn get_content_from_response_task(
    response: Response,
    error_message: String,
) -> Result<CallToolResult, RmcpError> {
    if response.status().is_success() {
        let updated_task: Task = response
            .json()
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(as_content_string(vec![
            updated_task,
        ])))
    } else {
        let error_msg = response
            .text()
            .await
            .unwrap_or_else(|_| error_message.clone());
        Ok(CallToolResult::error(vec![Content::text(format!(
            "Failed to {}: {}",
            error_message,
            error_msg
        ))]))
    }
}

pub fn as_content_list_string<T>(items: Vec<T>) -> Vec<Content>
where
    T: std::fmt::Display,
{
    items
        .iter()
        .enumerate()
        .map(|(index, item)| Content::text(format!("{}. {}", index, item.to_string())))
        .collect()
}

pub fn as_content_string<T>(items: Vec<T>) -> Vec<Content>
where
    T: std::fmt::Display,
{
    items
        .iter()
        .map(|item| Content::text(format!("{}", item.to_string())))
        .collect()
}
