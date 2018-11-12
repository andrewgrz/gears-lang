
def assignment() -> int {
    let a: int = 6;
    let b: int = 6; 
    let c: int = 8;

    a * b + c
}

def return_assignment() -> int {
    let a: int = 4 + 5;
    let b: int = a + 4;

    b
}

def reassign() -> int {
    let a: int = 4;
    let b: int = 5;
    a = a + b;

    a
}
