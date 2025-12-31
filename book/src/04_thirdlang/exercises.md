# Exercises

These exercises extend object-oriented features. Each explores different aspects of OOP implementation.

## Exercise 1: Add Static Methods

Add methods that don't require `self`:

```
class Math {
    def static max(a: int, b: int) -> int {
        if (a > b) { return a }
        return b
    }
}

Math.max(10, 20)
```

**Hints:**

- Parse `static` keyword before method name
- Static methods have no `self` parameter
- Call syntax: `ClassName.method(args)` instead of `obj.method(args)`
- No object allocation needed

<p class="checkpoint-inline"><strong>Checkpoint:</strong> <code>Math.max(5, 10)</code> returns <code>10</code></p>

## Exercise 2: Add Default Field Values

Allow field initializers:

```
class Counter {
    value: int = 0

    def increment(self) -> int {
        self.value = self.value + 1
        return self.value
    }
}

c = new Counter()  # value starts at 0
```

**Hints:**

- Store default value in `FieldDef`
- In codegen, initialize fields to defaults before calling `__init__`
- If field has default but `__init__` also sets it, `__init__` wins

<p class="checkpoint-inline"><strong>Checkpoint:</strong> <code>new Counter().get()</code> returns <code>0</code> without passing arguments</p>

## Exercise 3: Add Getter/Setter Sugar

Add automatic getter/setter generation:

```
class Point {
    x: int { get; set; }
    y: int { get; }  # read-only
}

p = new Point(1, 2)
p.x = 10     # Uses setter
print(p.x)   # Uses getter
```

**Hints:**

- Parse `{ get; set; }` after field type
- Generate `get_x()` and `set_x(value)` methods automatically
- Transform `p.x` to `p.get_x()` and `p.x = v` to `p.set_x(v)`

<p class="checkpoint-inline"><strong>Checkpoint:</strong> Property syntax works transparently</p>

## Exercise 4: Add Inheritance (Simplified)

Add single inheritance without virtual methods:

```
class Animal {
    name: int
    def speak(self) -> int { return 0 }
}

class Dog extends Animal {
    def speak(self) -> int { return 1 }
}
```

**Hints:**

- Child struct includes parent fields first
- `Dog` memory layout: `[Animal fields..., Dog fields...]`
- Method lookup: check child first, then parent
- No vtable needed if methods are resolved statically

<p class="checkpoint-inline"><strong>Checkpoint:</strong> <code>new Dog().speak()</code> returns <code>1</code></p>

## Exercise 5: Add Null/Optional Type

Add null safety:

```
class Node {
    value: int
    next: Node?  # Optional, can be null
}

n = new Node(1)
n.next = null
if (n.next != null) {
    print(n.next.value)
}
```

**Hints:**

- `Type::Optional(Box<Type>)` wraps nullable types
- Store as pointer; null is `0x0`
- Type check: must check for null before accessing
- Or: add `?.` safe navigation operator

<p class="checkpoint-inline"><strong>Checkpoint:</strong> <code>null</code> assignment and checking works</p>

## Stretch Goals

1. **Virtual methods with vtables**: Dynamic dispatch
2. **Multiple inheritance**: Diamond problem handling
3. **Interfaces**: `impl Comparable for Point`
4. **Generic classes**: `class Box<T> { value: T }`
5. **Reference counting**: Automatic memory management
6. **Super calls**: `super.method()` in overridden methods

## What You Practiced

- Extending class features incrementally
- Memory layout decisions
- Method resolution strategies
- Trade-offs in OOP implementation
