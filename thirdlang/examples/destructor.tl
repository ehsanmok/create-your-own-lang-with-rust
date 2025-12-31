# Destructor example
# Demonstrates __del__ method being called on delete

class Resource {
    id: int
    status: int  # 1 = active, 0 = released

    def __init__(self, id: int) {
        self.id = id
        self.status = 1  # Mark as active
    }

    def is_active(self) -> bool {
        return self.status == 1
    }

    def __del__(self) {
        self.status = 0  # Mark as released
    }
}

# Create resource
r = new Resource(42)

# Check it's active
active = r.is_active()

# Delete will call __del__
delete r

# Return 1 if was active (true)
active

