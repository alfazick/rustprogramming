# Comprehensive Analysis of Rust Generics in Caching and Authentication

## 1. Simple Cache With Generics

```rust
use std::collections::HashMap;

struct Cache<T> {
    storage: HashMap<String, T>,
}

impl<T> Cache<T> {
    fn new() -> Cache<T> {
        Cache {
            storage: HashMap::new(),
        }
    }

    fn get(&self, key: &str) -> Option<&T> {
        self.storage.get(key)
    }

    fn set(&mut self, key: String, value: T) {
        self.storage.insert(key, value);
    }
}
```

Explanation:
- This example demonstrates a basic implementation of a generic cache in Rust.
- The `Cache<T>` struct is generic over type `T`, which means it can store values of any type.
- The `storage` field is a `HashMap<String, T>`, allowing string keys to be mapped to values of type `T`.
- In the `impl<T> Cache<T>` block:
  - `new()` creates an empty cache.
  - `get(&self, key: &str) -> Option<&T>` retrieves a reference to a value if it exists.
  - `set(&mut self, key: String, value: T)` inserts a new key-value pair or updates an existing one.
- This implementation showcases how generics allow for type-safe, flexible data structures.

## 2. Cache with Expiration (Time To Live - TTL)

```rust
use std::collections::HashMap;
use std::hash::Hash;
use std::time::{Duration, Instant};

struct ExpiringCache<K, V> where K: Eq + Hash {
    storage: HashMap<K, (V, Instant)>,
    default_ttl: Duration,
}

impl<K, V> ExpiringCache<K, V> where K: Eq + Hash {
    fn new(default_ttl: Duration) -> Self {
        ExpiringCache {
            storage: HashMap::new(),
            default_ttl,
        }
    }

    fn insert(&mut self, key: K, value: V, ttl: Option<Duration>) {
        let expiration = Instant::now() + ttl.unwrap_or(self.default_ttl);
        self.storage.insert(key, (value, expiration));
    }

    fn get(&mut self, key: &K) -> Option<&V> {
        let expired = if let Some((_, expiration)) = self.storage.get(key) {
            Instant::now() > *expiration
        } else {
            false
        };
    
        if expired {
            self.storage.remove(key);
            None
        } else {
            self.storage.get(key).map(|(value, _)| value)
        }
    }
}

fn main() {
    let mut cache = ExpiringCache::new(Duration::new(2, 0)); // Items expire after 2 seconds
    cache.insert("key1", "value1", None);

    // Immediately retrieve the item
    match cache.get(&"key1") {
        Some(value) => println!("Retrieved: {}", value),
        None => println!("Not found or expired"),
    }

    // Wait for more than 2 seconds
    std::thread::sleep(Duration::new(3, 0));

    // Try to retrieve the item again
    match cache.get(&"key1") {
        Some(value) => println!("Retrieved: {}", value),
        None => println!("Not found or expired"),
    }
}
```

Explanation:
- This example presents a more advanced cache implementation with expiration functionality.
- `ExpiringCache<K, V>` is generic over two types: `K` for keys and `V` for values.
- The `where K: Eq + Hash` clause ensures that the key type can be used in a `HashMap`.
- The `storage` field is a `HashMap<K, (V, Instant)>`, storing both the value and its expiration time.
- `new(default_ttl: Duration)` creates a cache with a default time-to-live for entries.
- `insert` allows adding or updating entries with an optional custom TTL.
- `get` implements "lazy expiration":
  - It checks if the requested entry has expired.
  - If expired, it removes the entry and returns `None`.
  - If not expired, it returns a reference to the value.
- The `main` function demonstrates usage, including the expiration behavior.
- This example shows how generics can be combined with standard library types (`Duration`, `Instant`) to create sophisticated, type-safe data structures.

## 3. Authentication and Authorization System

```rust
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct User {
    username: String,
    role: Role,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Role {
    User,
    Admin,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum TagPrivilege {
    Read,
    Write,
}

fn role_permissions(role: &Role) -> Vec<TagPrivilege> {
    match role {
        Role::User => vec![TagPrivilege::Read],
        Role::Admin => vec![TagPrivilege::Read, TagPrivilege::Write],
    }
}

fn has_permission(user: &User, privilege_to_check: &TagPrivilege) -> bool {
    let privileges = role_permissions(&user.role);
    privileges.iter().any(|p| p == privilege_to_check)
}

fn authenticate_and_authorize(users: &HashMap<String, User>, username: &str, privilege: TagPrivilege) -> Result<(), String> {
    let user = users.get(username).ok_or("User not found")?;

    if has_permission(user, &privilege) {
        Ok(())
    } else {
        Err("User does not have the required privilege".to_string())
    }
}

fn main() {
    let users = [
        (String::from("alice"), User { username: "alice".to_string(), role: Role::User }),
        (String::from("bob"), User { username: "bob".to_string(), role: Role::Admin }),
    ]
    .iter()
    .cloned()
    .collect::<HashMap<String, User>>();

    let result = authenticate_and_authorize(&users, "bob", TagPrivilege::Write);

    match result {
        Ok(_) => println!("User is authenticated and authorized"),
        Err(e) => println!("Error: {}", e),
    }
}
```

Explanation:
- This example demonstrates an authentication and authorization system.
- While it doesn't explicitly use generics in its struct or enum definitions, it shows how non-generic types can be used in a system that could potentially benefit from generics.
- `User`, `Role`, and `TagPrivilege` are defined as custom types.
- `role_permissions` maps roles to their corresponding privileges.
- `has_permission` checks if a user has a specific privilege.
- `authenticate_and_authorize` performs both authentication (checking if the user exists) and authorization (checking if the user has the required privilege).
- The `main` function demonstrates the usage of this system.
- Although this example doesn't directly use generics, it shows a scenario where generics could be applied for more flexible role and privilege systems.
- The use of `HashMap<String, User>` in `authenticate_and_authorize` shows how generic standard library types are used in the system.

Key observations about generics from these examples:
1. Generics provide type safety while allowing flexibility in data types.
2. They can be used with structs, enums, and functions to create reusable code.
3. Trait bounds (like `K: Eq + Hash`) can constrain generic types to those with specific capabilities.
4. Generics in Rust are zero-cost abstractions, resolved at compile-time.
5. They can be effectively combined with other Rust features like enums, pattern matching, and standard library types.
6. Even in systems not explicitly using generics (like the authentication example), there's often potential to introduce generics for increased flexibility.

These examples demonstrate the power and versatility of generics in Rust, showcasing their application in real-world scenarios such as caching systems and potentially in authentication and authorization systems.

