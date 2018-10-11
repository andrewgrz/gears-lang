use errors::GearsError;

pub type GearsResult = Result<GearsObject, GearsError>;

fn create_type_error(op: &str, left: &GearsObject, right: &GearsObject) -> GearsError {
    GearsError::TypeError(format!(
        "TypeError: Unable to perform {} on {} and {}",
        op,
        left.get_type_str(),
        right.get_type_str()
    ))
}

#[derive(Debug, PartialEq, Clone)]
pub enum GearsObject {
    Int(i64),
    Bool(bool),
    None,
}

impl GearsObject {
    pub fn add(self, other: GearsObject) -> GearsResult {
        use self::GearsObject::*;

        match self {
            Int(l) => match other {
                Int(r) => Ok(Int(l + r)),
                _ => Err(create_type_error("add", &self, &other)),
            },
            _ => Err(create_type_error("add", &self, &other)),
        }
    }

    pub fn sub(self, other: GearsObject) -> GearsResult {
        use self::GearsObject::*;

        match self {
            Int(l) => match other {
                Int(r) => Ok(Int(l - r)),
                _ => Err(create_type_error("sub", &self, &other)),
            },
            _ => Err(create_type_error("sub", &self, &other)),
        }
    }

    pub fn mul(self, other: GearsObject) -> GearsResult {
        use self::GearsObject::*;

        match self {
            Int(l) => match other {
                Int(r) => Ok(Int(l * r)),
                _ => Err(create_type_error("mul", &self, &other)),
            },
            _ => Err(create_type_error("mul", &self, &other)),
        }
    }

    pub fn div(self, other: GearsObject) -> GearsResult {
        use self::GearsObject::*;

        match self {
            Int(l) => match other {
                Int(r) => Ok(Int(l / r)),
                _ => Err(create_type_error("div", &self, &other)),
            },
            _ => Err(create_type_error("div", &self, &other)),
        }
    }

    pub fn get_type_str(&self) -> &str {
        use self::GearsObject::*;

        match self {
            Int(_) => "Integer",
            Bool(_) => "Bool",
            None => "NoneType",
        }
    }
}
