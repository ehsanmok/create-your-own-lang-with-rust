# From Functions to Objects

You've built Secondlang - a typed, compiled language that generates native code via LLVM. Fast Fibonacci!

But something's missing. Look at this code:

```
# Two related values
x1 = 10
y1 = 20

x2 = 30
y2 = 40

# How do we know x1 goes with y1?
# How do we pass "a point" to a function?
```

We need a way to **group related data together**.

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

By the end of Part IV:

```
class Counter {
    value: int

    def __init__(self, start: int) { self.value = start }
    def increment(self) -> int {
        self.value = self.value + 1
        return self.value
    }
}

c = new Counter(0)
c.increment()  # 1
c.increment()  # 2
delete c
```

Real OOP, compiled to native code.

## Ready?

Let's add classes and objects.
