# From Functions to Objects

You've built Secondlang - a typed, compiled language that generates native code via LLVM. It's fast (computes `fib(35)` in milliseconds), type-safe (catches errors at compile time), and generates efficient machine code.

But try modeling something from the real world - say, a 2D point on a screen, or a player in a game, or a bank account. You'll quickly hit a wall.

Look at this code for computing the distance between two points:

```
# Point 1
x1 = 10
y1 = 20

# Point 2
x2 = 30
y2 = 40

# Distance computation
dx = x2 - x1
dy = y2 - y1
distance_squared = dx*dx + dy*dy
```

This works, but there are problems:

1. **No semantic grouping** - Nothing says `x1` and `y1` belong together. They're just two independent integers.
2. **Easy to mix up** - What if you accidentally use `x1` with `y2`? The compiler won't catch it.
3. **Can't pass as a unit** - You can't write `def distance(p1, p2)`. You need `def distance(x1, y1, x2, y2)`.
4. **No encapsulation** - The distance formula is scattered across your code. If you want to change it, you need to find every place you computed it.

We need a way to **group related data** and **attach behavior to that data**.

## The Problem with Primitives

> Think of a filing cabinet. Secondlang gives you individual papers (integers, booleans). But you want *folders* that group related papers together. Classes are those folders.

Without classes:

```
def distance(x1: int, y1: int, x2: int, y2: int) -> int {
    dx = x2 - x1
    dy = y2 - y1
    return dx*dx + dy*dy
}
```

With classes:

```
def distance(p1: Point, p2: Point) -> int {
    dx = p2.x - p1.x
    dy = p2.y - p1.y
    return dx*dx + dy*dy
}
```

The second version is clearer - we're computing distance between *points*, not between four arbitrary integers.

## What Classes Give Us

| Feature | Benefit |
|---------|---------|
| **Grouping** | Related data lives together |
| **Methods** | Functions that operate on the data |
| **Encapsulation** | Data + behavior in one place |
| **Types** | `Point` is a type, like `int` |

## The Implementation Challenge

Classes require new concepts:

```
class Point {
    x: int      # Fields - data stored in the object
    y: int

    def __init__(self, x: int, y: int) {  # Constructor
        self.x = x
        self.y = y
    }

    def move(self, dx: int, dy: int) {    # Method
        self.x = self.x + dx
        self.y = self.y + dy
    }
}

p = new Point(10, 20)   # Heap allocation
p.move(5, 5)            # Method call
delete p                # Manual deallocation
```

Each of these needs compiler support.

## Stack vs Heap

> The stack is like a notepad - you write, you're done, you tear off the page. The heap is like a whiteboard - you write, it stays until you erase it. Objects live on the heap because they need to *outlive* the function that created them.

| Memory | Lifetime | Management |
|--------|----------|------------|
| Stack | Function call | Automatic |
| Heap | Until deleted | Manual |

Secondlang used only the stack. Thirdlang adds the heap.

## What Carries Forward

Everything from Secondlang, plus:

| From Secondlang | Extended In Thirdlang |
|-----------------|----------------------|
| Types (`int`, `bool`) | + `ClassName` types |
| Functions | + Methods (with `self`) |
| Stack allocation | + Heap allocation (`new`/`delete`) |
| LLVM i64, i1 | + LLVM struct types |

## What's New

| Concept | LLVM Equivalent |
|---------|-----------------|
| Class | Struct type |
| Object | Pointer to struct on heap |
| Field access | GEP (GetElementPointer) |
| `new` | `malloc` + constructor |
| `delete` | Destructor + `free` |

## The Goal

By the end of Part IV, you'll have a full object-oriented language:

```
class Counter {
    value: int

    def __init__(self, start: int) {
        self.value = start
    }

    def increment(self) -> int {
        self.value = self.value + 1
        return self.value
    }

    def __del__(self) {
        # Cleanup when deleted
    }
}

c = new Counter(0)
c.increment()  # 1
c.increment()  # 2
delete c
```

This small example demonstrates everything OOP adds:

- **Grouping** - `value` lives inside `Counter`, not as a loose variable
- **Methods** - `increment` operates on `self`, not global state
- **Encapsulation** - Implementation details hidden inside the class
- **Lifecycle** - `__init__` creates, `__del__` cleans up
- **Heap allocation** - Objects live beyond function scope
- **Manual memory** - You control when objects are freed

## What You'll Learn

Thirdlang teaches you how OOP is implemented under the hood:

1. **Class types** - How classes become LLVM struct types
2. **Heap allocation** - Using `malloc` to create objects
3. **Field access** - LLVM's `getelementptr` instruction
4. **Method calls** - Passing `self` as the first parameter
5. **Constructors/Destructors** - Initialization and cleanup
6. **Memory management** - Manual `new`/`delete` like C++

After Thirdlang, you'll understand how Python's classes, JavaScript's objects, and C++'s structs work at the machine level.

## The Trade-offs

Thirdlang's memory model is deliberately simple - and dangerous:

- **No garbage collection** - You must manually `delete` objects
- **Dangling pointers** - Using an object after `delete` is undefined behavior
- **Memory leaks** - Forgetting to `delete` leaks memory forever
- **Double free** - Calling `delete` twice is undefined behavior

Real languages solve this with:

- **Garbage collection** (Python, Java, Go)
- **Reference counting** (Swift, Python's implementation)
- **Ownership** (Rust)
- **Smart pointers** (C++)

We don't implement these because they're complex. But after Thirdlang, you'll understand *why* they exist and *what problems* they solve.

## Ready?

Let's add classes and objects.
