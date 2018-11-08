
def test_true() {
    true
}

def test_false() {
    false
}

def simple_branch(test: bool) {
    if test {
        5
    } else {
        4
    }
}

def five_or_none(test: bool) {
    if test {
        5
    }
}

def while_loop() {
    let a: int = 1;
    while a < 5 {
        a = a + 1;
    };
    a
}

def for_loop() {
    let result: int = 10;

    for x in 1 to 10 {
        result = result + 1;
    };
    result
}
