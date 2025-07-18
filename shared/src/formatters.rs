use std::fmt::{Display, Formatter};
use crate::models::task_model::Task;

impl Display for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} - {} |{}| Due Date: {} (#{})",
            self.name,
            self.description,
            self.status,
            self.due_date.as_deref().unwrap_or("N/A"),
            self.id,
        )
    }
}