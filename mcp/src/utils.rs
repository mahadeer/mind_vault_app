use rmcp::model::Content;

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