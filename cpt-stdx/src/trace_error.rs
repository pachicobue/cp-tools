use itertools::Itertools;

pub fn error_trace(error: &impl std::error::Error) -> String {
    let mut messages = vec![];
    error_trace_inner(error, &mut messages);
    let children_message = &messages
        .iter()
        .map(|message| format!("* {message}"))
        .collect_vec()
        .join("\n");
    format!("{}\n{children_message}", error)
}

fn error_trace_inner(error: &dyn std::error::Error, messages: &mut Vec<String>) {
    messages.push(error.to_string());
    if let Some(source) = error.source() {
        error_trace_inner(source, messages);
    }
}
