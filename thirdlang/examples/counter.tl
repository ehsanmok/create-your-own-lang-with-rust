# Counter class example
# Demonstrates methods that modify object state

class Counter {
    value: int

    def __init__(self, start: int) {
        self.value = start
    }

    def increment(self) -> int {
        self.value = self.value + 1
        return self.value
    }

    def decrement(self) -> int {
        self.value = self.value - 1
        return self.value
    }

    def get(self) -> int {
        return self.value
    }

    def add(self, n: int) -> int {
        self.value = self.value + n
        return self.value
    }
}

# Create counter starting at 10
c = new Counter(10)

# Increment a few times
c.increment()
c.increment()
c.increment()

# Add 5
c.add(5)

# Get final value (should be 18)
result = c.get()

delete c

result

