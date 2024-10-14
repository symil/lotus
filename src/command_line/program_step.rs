#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProgramStep {
    Read,
    Parse,
    Process,
    Resolve,
    Stringify,
    Write,
    Total
}

impl ProgramStep {
    pub fn get_name(&self) -> &'static str {
        match self {
            ProgramStep::Read => "read",
            ProgramStep::Parse => "parse",
            ProgramStep::Process => "process",
            ProgramStep::Resolve => "resolve",
            ProgramStep::Stringify => "stringify",
            ProgramStep::Write => "write",
            ProgramStep::Total => "total",
        }
    }

    pub fn is_negligible(&self) -> bool {
        match self {
            ProgramStep::Read => true,
            ProgramStep::Parse => false,
            ProgramStep::Process => false,
            ProgramStep::Resolve => false,
            ProgramStep::Stringify => false,
            ProgramStep::Write => true,
            ProgramStep::Total => false,
        }
    }
}