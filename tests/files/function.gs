
def main_no_args() -> int {
    sub(6, 2);
    let a: int = 4 - 3;
    let b: int = a * 9;
    let c: int = sub(b, a);
    12 * add(a + 4 - add_two(5), b) + c
}

def expr_test() {
    let a: int = 2;
    let b: int = 3;
    add(a, b);
}

def main_args(a: int, b: int) -> int {
    12 * add(a + 4 - add_two(5), b) + 8
}

def returns_none(a: int, b: int) -> none {
    4 + 5;
}

def sub(a: int, b: int) -> int {
    a - b
}

def add(a: int, b: int) -> int {
    a + b
}

def add_two(a: int) -> int {
    add(a, 2)
}
