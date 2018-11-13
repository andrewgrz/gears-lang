
def test_true() -> bool {
    true
}

def test_false() -> bool {
    false
}

def simple_branch(test: bool) -> int {
    if test {
        5
    } else {
        4
    }
}

def five_or_none(test: bool) -> int | none {
    if test {
        5
    }
}

def while_loop() -> int {
    let a: int = 1;
    while a < 5 {
        a = a + 1;
    };
    a
}

def for_loop() -> int {
    let result: int = 10;

    for x in 1 to 10 {
        result = result + 1;
    };
    result
}
