# Factorial - Recursive implementation

def factorial(n: int) -> int {
    if (n <= 1) {
        return 1
    } else {
        return n * factorial(n - 1)
    }
}

factorial(10)

