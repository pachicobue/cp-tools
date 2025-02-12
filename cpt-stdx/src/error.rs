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

#[cfg(test)]
mod tests {
    use super::stacktrace;
    use pretty_assertions::assert_eq;
    use test_case::test_case;
    use thiserror::Error;

    #[derive(Error, Debug)]
    enum Err1 {
        #[error("Case11")]
        Case11,
        #[error("Case12 with {0}")]
        Case12(String),
    }
    #[derive(Error, Debug)]
    enum Err2 {
        #[error("Case21")]
        Case21(#[source] Err1),
    }
    #[derive(Error, Debug)]
    enum Err3 {
        #[error("Case32")]
        Case32(#[source] Err2),
    }

    #[test_case(Err1::Case11, vec!["* Case11"]; "Level0 error")]
    #[test_case(Err1::Case12("Test".into()), vec!["* Case12 with Test"]; "Level0 error w/ String")]
    #[test_case(Err2::Case21(Err1::Case11), vec!["* Case21", "** Case11"]; "Level1 error")]
    #[test_case(Err3::Case32(Err2::Case21(Err1::Case11)), vec!["* Case32", "** Case21", "*** Case11"]; "Level2 error")]
    fn stacktrace_fmt<E: std::error::Error>(e: E, disps: Vec<&str>) {
        let expect = disps.join("\n");
        assert_eq!(stacktrace(e), expect);
    }
}
