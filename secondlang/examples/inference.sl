# Type Inference Examples
# Demonstrates how Secondlang infers types automatically

# The compiler infers x is int from the literal
x = 42

# Type flows through arithmetic: y is inferred as int
y = x * 2 + 10

# Comparisons produce bool: is_big is inferred as bool
is_big = y > 50

# Function with explicit types
def square(n: int) -> int {
    return n * n
}

# Type inference through function calls
# result is inferred as int because square returns int
result = square(5) + square(3)

# Chained inference: the compiler figures out all intermediate types
def compute(a: int, b: int) -> int {
    temp = a + b        # temp inferred as int
    doubled = temp * 2  # doubled inferred as int
    return doubled + 1
}

# Conditional with type inference
def abs(n: int) -> int {
    if (n < 0) {
        return -n
    } else {
        return n
    }
}

# Complex expression: compiler infers types at each step
def complex_calc(x: int) -> int {
    a = x + 1           # int (from x + literal)
    b = a * a           # int (from int * int)
    c = b - x           # int (from int - int)
    flag = c > 100      # bool (from comparison)
    if (flag) {
        return c
    } else {
        return b
    }
}

# Final computation
complex_calc(10)

