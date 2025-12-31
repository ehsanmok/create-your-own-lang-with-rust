# Point class example
# Demonstrates basic class features: fields, constructor, methods

class Point {
    x: int
    y: int

    def __init__(self, x: int, y: int) {
        self.x = x
        self.y = y
    }

    def get_x(self) -> int {
        return self.x
    }

    def get_y(self) -> int {
        return self.y
    }

    def distance_squared(self, other: Point) -> int {
        dx = self.x - other.x
        dy = self.y - other.y
        return dx * dx + dy * dy
    }
}

# Create two points
p1 = new Point(0, 0)
p2 = new Point(3, 4)

# Calculate distance squared (should be 25)
result = p1.distance_squared(p2)

# Clean up
delete p1
delete p2

# Return result
result

