pub type Manifest = Vec<(String, Parm)>;

#[derive(Debug, Clone, PartialEq)]
pub enum Parm {
    String { value: String },
    Bool { value: bool },
    Int { value: i64, min: i64, max: i64 },
    UInt { value: u64, min: u64, max: u64 },
    Double { value: f64, min: f64, max: f64 },
    Choice { value: String, choices: Vec<String> },
}

impl Parm {
    pub fn take_value_from(&mut self, other: &Parm) {
        match (self, other) {
            (Self::String { value }, Self::String { value: oldval }) => *value = oldval.clone(),
            (Self::Bool { value }, Self::Bool { value: oldval }) => *value = *oldval,
            (Self::Int { value, min, max }, Self::Int { value: oldval, .. }) => {
                if (*min..=*max).contains(oldval) {
                    *value = *oldval;
                }
            }
            (Self::UInt { value, min, max }, Self::UInt { value: oldval, .. }) => {
                if (*min..=*max).contains(oldval) {
                    *value = *oldval;
                }
            }
            (Self::Double { value, min, max }, Self::Double { value: oldval, .. }) => {
                if (*min..=*max).contains(oldval) {
                    *value = *oldval;
                }
            }
            (Self::Choice { value, choices }, Self::Choice { value: oldval, .. }) => {
                if choices.contains(oldval) {
                    *value = oldval.clone();
                }
            }
            _ => {}
        }
    }
}
