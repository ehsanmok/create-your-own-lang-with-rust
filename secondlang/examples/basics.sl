# Basic Secondlang examples
# Same as Firstlang basics.fl, but with type annotations

# Variables with explicit types
x: int = 10
y: int = 20

# Variables with inferred types (type is deduced from the value)
sum = x + y
diff = x - y
prod = x * y

# Comparisons (result is bool, but stored in int for simplicity)
is_less = x < y
is_equal = x == 10

# Functions with typed parameters and return types
def double(n: int) -> int {
    return n * 2
}

def add(a: int, b: int) -> int {
    return a + b
}

# Function calls - final result
add(double(5), 3)
