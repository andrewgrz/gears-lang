
def test_true() {
    true
}

def test_false() {
    false
}

def simple_branch(test) {
    if test {
        5
    } else {
        4
    }
}

def five_or_none(test) {
    if test {
        5
    }
}

def while_loop() {
    let a = 1;
    while a < 5 {
        a = a + 1;
    };
    a
}
