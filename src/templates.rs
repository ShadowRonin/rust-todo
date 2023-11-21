use askama::Template;

#[derive(Template)]
#[template(path = "todo_row.html")]
pub struct TodoRow<'a> {
    pub title: &'a str,
    pub desc: &'a str,
}
