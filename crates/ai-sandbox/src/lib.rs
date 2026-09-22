//! Policy checks for AI tool requests. This crate is not OS-level isolation.
use std::collections::BTreeSet;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum Error {
    #[error("tool is not allowed")]
    ToolNotAllowed,
    #[error("step budget exceeded")]
    StepBudgetExceeded,
    #[error("argument budget exceeded")]
    ArgumentBudgetExceeded,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolRequest { pub name: String, pub arguments: String }

pub struct Sandbox { allowed: BTreeSet<String>, max_steps: usize, max_argument_bytes: usize, steps: usize }

impl Sandbox {
    pub fn new<I, S>(allowed: I, max_steps: usize, max_argument_bytes: usize) -> Self
    where I: IntoIterator<Item = S>, S: Into<String> {
        Self { allowed: allowed.into_iter().map(Into::into).collect(), max_steps, max_argument_bytes, steps: 0 }
    }

    /// Validates a request and returns it for a host-owned executor.
    pub fn authorize(&mut self, request: ToolRequest) -> Result<ToolRequest, Error> {
        if !self.allowed.contains(&request.name) { return Err(Error::ToolNotAllowed); }
        if self.steps >= self.max_steps { return Err(Error::StepBudgetExceeded); }
        if request.arguments.len() > self.max_argument_bytes { return Err(Error::ArgumentBudgetExceeded); }
        self.steps += 1;
        Ok(request)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn allowlist_and_budget_are_enforced() {
        let mut sandbox = Sandbox::new(["search"], 1, 10);
        assert!(sandbox.authorize(ToolRequest { name: "search".into(), arguments: "{}".into() }).is_ok());
        assert_eq!(sandbox.authorize(ToolRequest { name: "search".into(), arguments: "{}".into() }), Err(Error::StepBudgetExceeded));
        assert_eq!(sandbox.authorize(ToolRequest { name: "shell".into(), arguments: "{}".into() }), Err(Error::ToolNotAllowed));
    }
}
