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

enum CompareDirection {
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,
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

    #[inline]
    fn _equal(self, other: GearsObject) -> bool {
        use self::GearsObject::*;

        match self {
            Int(l) => match other {
                Int(r) => l == r,
                _ => false,
            },
            Bool(l) => match other {
                Bool(r) => l == r,
                _ => false,
            },
            None => false,
        }
    }

    pub fn equal(self, other: GearsObject) -> GearsResult {
        Ok(GearsObject::Bool(self._equal(other)))
    }

    pub fn nequal(self, other: GearsObject) -> GearsResult {
        Ok(GearsObject::Bool(!self._equal(other)))
    }

    #[inline]
    fn compare(self, other: GearsObject, dir: CompareDirection, op: &str) -> GearsResult {
        use self::GearsObject::*;

        match self {
            Int(l) => match other {
                Int(r) => {
                    use self::CompareDirection::*;

                    match dir {
                        LessThan => Ok(Bool(l < r)),
                        GreaterThan => Ok(Bool(l > r)),
                        LessThanEqual => Ok(Bool(l <= r)),
                        GreaterThanEqual => Ok(Bool(l >= r)),
                    }
                }
                _ => Err(create_type_error(op, &self, &other)),
            },
            _ => Err(create_type_error(op, &self, &other)),
        }
    }

    pub fn less(self, other: GearsObject) -> GearsResult {
        self.compare(other, CompareDirection::LessThan, "<")
    }

    pub fn greater(self, other: GearsObject) -> GearsResult {
        self.compare(other, CompareDirection::GreaterThan, ">")
    }

    pub fn less_eq(self, other: GearsObject) -> GearsResult {
        self.compare(other, CompareDirection::LessThanEqual, "<=")
    }

    pub fn greater_eq(self, other: GearsObject) -> GearsResult {
        self.compare(other, CompareDirection::GreaterThanEqual, ">=")
    }

    pub fn get_type_str(&self) -> &str {
        use self::GearsObject::*;

        match self {
            Int(_) => "Integer",
            Bool(_) => "Bool",
            None => "NoneType",
        }
    }

    pub fn as_bool(&self) -> bool {
        use self::GearsObject::*;

        match self {
            Bool(b) => *b,
            Int(i) => *i != 0,
            None => false,
        }
    }
}

impl From<i64> for GearsObject {
    fn from(i: i64) -> GearsObject {
        GearsObject::Int(i)
    }
}

impl From<bool> for GearsObject {
    fn from(b: bool) -> GearsObject {
        GearsObject::Bool(b)
    }
}
