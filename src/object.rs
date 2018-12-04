use errors::GearsError;
use std::sync::Arc;

pub type GearsResult = Result<GearsObject, GearsError>;
pub type ArcGearsResult = Result<ArcGearsObject, GearsError>;
pub type ArcGearsObject = Arc<GearsObject>;

lazy_static! {
    pub static ref TRUE_OBJ: ArcGearsObject = Arc::new(GearsObject::Bool(true));
    pub static ref FALSE_OBJ: ArcGearsObject = Arc::new(GearsObject::Bool(false));
    pub static ref NONE_OBJ: ArcGearsObject = Arc::new(GearsObject::None);
}

#[macro_export]
macro_rules! gears_obj {
    ( list $( $x:expr ),* ) => {
        {
            let mut v = Vec::new();
            $(
                v.push(gears_obj!($x));
            )*
            Arc::new(GearsObject::from(v))
        }
    };

    ($e:expr) => {{
        Arc::new(GearsObject::from($e))
    }};
}

fn create_type_error(op: &str, left: &GearsObject, right: &GearsObject) -> GearsError {
    GearsError::TypeError(format!(
        "TypeError: Unable to perform {} on {} and {}",
        op,
        left.get_type_str(),
        right.get_type_str()
    ))
}

fn create_type_error_unary(op: &str, left: &GearsObject) -> GearsError {
    GearsError::TypeError(format!(
        "TypeError: Unable to perform {} on {}",
        op,
        left.get_type_str(),
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
    Str(String),
    Int(i64),
    Bool(bool),
    List(Vec<ArcGearsObject>),
    None,
}

impl GearsObject {
    pub fn inc(&self) -> GearsResult {
        use self::GearsObject::*;

        match self {
            Int(l) => Ok(Int(l + 1)),
            _ => Err(create_type_error_unary("increment", &self)),
        }
    }

    pub fn add(&self, other: &GearsObject) -> GearsResult {
        use self::GearsObject::*;

        match self {
            Int(l) => match other {
                Int(r) => Ok(Int(l + r)),
                _ => Err(create_type_error("add", &self, &other)),
            },
            Str(ref l) => match other {
                Str(ref r) => Ok(Str(l.to_owned() + r)),
                _ => Err(create_type_error("add", &self, &other)),
            },
            _ => Err(create_type_error("add", &self, &other)),
        }
    }

    pub fn sub(&self, other: &GearsObject) -> GearsResult {
        use self::GearsObject::*;

        match self {
            Int(l) => match other {
                Int(r) => Ok(Int(l - r)),
                _ => Err(create_type_error("sub", &self, &other)),
            },
            _ => Err(create_type_error("sub", &self, &other)),
        }
    }

    pub fn mul(&self, other: &GearsObject) -> GearsResult {
        use self::GearsObject::*;

        match self {
            Int(l) => match other {
                Int(r) => Ok(Int(l * r)),
                _ => Err(create_type_error("mul", &self, &other)),
            },
            _ => Err(create_type_error("mul", &self, &other)),
        }
    }

    pub fn div(&self, other: &GearsObject) -> GearsResult {
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
    fn _equal(&self, other: &GearsObject) -> bool {
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
            Str(l) => match other {
                Str(r) => l == r,
                _ => false,
            },
            List(l) => match other {
                List(r) => l == r,
                _ => false,
            },
            None => false,
        }
    }

    pub fn equal(&self, other: &GearsObject) -> GearsResult {
        Ok(GearsObject::Bool(self._equal(other)))
    }

    pub fn nequal(&self, other: &GearsObject) -> GearsResult {
        Ok(GearsObject::Bool(!self._equal(other)))
    }

    #[inline]
    fn compare(&self, other: &GearsObject, dir: CompareDirection, op: &str) -> GearsResult {
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

    pub fn less(&self, other: &GearsObject) -> GearsResult {
        self.compare(other, CompareDirection::LessThan, "<")
    }

    pub fn greater(&self, other: &GearsObject) -> GearsResult {
        self.compare(other, CompareDirection::GreaterThan, ">")
    }

    pub fn less_eq(&self, other: &GearsObject) -> GearsResult {
        self.compare(other, CompareDirection::LessThanEqual, "<=")
    }

    pub fn greater_eq(&self, other: &GearsObject) -> GearsResult {
        self.compare(other, CompareDirection::GreaterThanEqual, ">=")
    }

    pub fn get_type_str(&self) -> &str {
        use self::GearsObject::*;

        match self {
            Int(_) => "Integer",
            Bool(_) => "Bool",
            Str(_) => "String",
            List(_) => "List",
            None => "NoneType",
        }
    }

    pub fn as_bool(&self) -> bool {
        use self::GearsObject::*;

        match self {
            Bool(b) => *b,
            Int(i) => *i != 0,
            Str(s) => s.len() > 0,
            List(l) => l.len() > 0,
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

impl<'a> From<&'a str> for GearsObject {
    fn from(s: &str) -> GearsObject {
        GearsObject::Str(s.to_string())
    }
}

impl From<Vec<ArcGearsObject>> for GearsObject {
    fn from(v: Vec<ArcGearsObject>) -> GearsObject {
        GearsObject::List(v)
    }
}
