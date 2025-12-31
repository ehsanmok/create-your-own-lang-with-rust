# Simple node with value
# Demonstrates basic object composition setup

class Node {
    value: int

    def __init__(self, val: int) {
        self.value = val
    }

    def get_value(self) -> int {
        return self.value
    }

    def set_value(self, val: int) {
        self.value = val
    }
}

# Create nodes
n1 = new Node(10)
n2 = new Node(20)
n3 = new Node(30)

# Sum their values
sum = n1.get_value() + n2.get_value() + n3.get_value()

# Clean up
delete n1
delete n2
delete n3

# Return sum (should be 60)
sum

