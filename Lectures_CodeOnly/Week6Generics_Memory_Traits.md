# Simple Cache With Generics

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

# Cache with Expiration (Time Time To Live) TTL

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
        // Note on strategy : where expired or invalid data 
        // is checked and potentially removed at the time of access (read or write), 
        //is commonly known as "lazy expiration" or "lazy deletion" in the context of caching systems.
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

use std::collections::HashMap;

// Define the required structs
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

// Function to get permissions for a role
fn role_permissions(role: &Role) -> Vec<TagPrivilege> {
    match role {
        Role::User => vec![TagPrivilege::Read],
        Role::Admin => vec![TagPrivilege::Read, TagPrivilege::Write],
    }
}

// Function to check if a user has a certain permission
fn has_permission(user: &User, privilege_to_check: &TagPrivilege) -> bool {
    let privileges = role_permissions(&user.role);
    privileges.iter().position(|p| p == privilege_to_check).is_some()
}

// Function to authenticate and authorize a user
fn authenticate_and_authorize(users: &HashMap<String, User>, username: &str, privilege: TagPrivilege) -> Result<(), String> {
    // Authenticate the user by checking if the username exists
    let user = match users.get(username) {
        Some(user) => user,
        None => return Err("User not found".to_string()),
    };

    // Authorize the user by checking if they have the required permission
    if has_permission(user, &privilege) {
        Ok(())
    } else {
        Err("User does not have the required privilege".to_string())
    }
}

fn main() {
    // Create roles
    let user_role = Role::User;
    let admin_role = Role::Admin;

    // Create users with roles
    let users = [
        (String::from("alice"), User { username: "alice".to_string(), role: user_role.clone() }),
        (String::from("bob"), User { username: "bob".to_string(), role: admin_role.clone() }),
    ]
    .iter()
    .cloned()
    .collect::<HashMap<String, User>>();

    // Authenticate and authorize a user
    //let result = authenticate_and_authorize(&users, "alice", TagPrivilege::Write);
    let result = authenticate_and_authorize(&users, "bob", TagPrivilege::Write);

    match result {
        Ok(_) => println!("User is authenticated and authorized"),
        Err(e) => println!("Error: {}", e),
    }
}