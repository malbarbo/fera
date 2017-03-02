#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Control {
    Continue,
    Break,
}

impl From<()> for Control {
    fn from(_: ()) -> Self {
        Control::Continue
    }
}

pub fn continue_if(cond: bool) -> Control {
    if cond {
        Control::Continue
    } else {
        Control::Break
    }
}

pub fn break_if(cond: bool) -> Control {
    if cond {
        Control::Break
    } else {
        Control::Continue
    }
}

macro_rules! return_unless {
    ($e:expr) => (
        if $e == Control::Break  {
            return Control::Break;
        }
    )
}

macro_rules! break_unless {
    ($e:expr) => (
        if $e == Control::Break {
            break;
        }
    )
}
