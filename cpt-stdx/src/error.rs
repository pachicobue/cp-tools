pub fn stacktrace<E>(error: E) -> String
where
    E: std::error::Error,
{
    let mut messages = vec![];
    trace_inner(error, &mut messages, "*".into());
    messages.join("\n")
}

fn trace_inner<E>(error: E, messages: &mut Vec<String>, prefix: String)
where
    E: std::error::Error,
{
    messages.push(format!("{} {}", prefix, error));
    if let Some(source) = error.source() {
        trace_inner(source, messages, prefix + "*");
    }
}
