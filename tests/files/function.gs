
def main_no_args() {
    sub(6, 2);
    let a = 4 - 3;
    let b = a * 9;
    let c = sub(b, a);
    12 * add(a + 4 - add_two(5), b) + c
}

def expr_test() {
    let a = 2;
    let b = 3;
    add(a, b);
}

def main_args(a, b) {
    12 * add(a + 4 - add_two(5), b) + 8
}

def sub(a, b) {
    a - b
}

def add(a, b) {
    a + b
}

def add_two(a) {
    add(a, 2)
}
