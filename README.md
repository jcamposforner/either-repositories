# Rust User Repository Example

This Rust code demonstrates an implementation of an in-memory user repository with caching. It involves two main
components: `FixedUserRepository` and `InMemoryUserRepository`. `FixedUserRepository` provides a fixed set of user data,
while `InMemoryUserRepository` serves as a caching layer to efficiently store and retrieve user data.

## Components

### `User` Struct

```rust
struct User {
    id: String,
    age: u32,
}
```

- User represents a user entity with an id and an age.

```rust
trait UserRepository {
    fn search(&self, id: &str) -> Option<Either<User, &mut User>>;
}
```

- **UserRepository** is a trait defining the search method to find a user by their ID. The method returns an Option
  containing either a User or a mutable reference to a User from the repository.

```rust
struct InMemoryUserRepository<'a, T>
where
    T: UserRepository,
{
    users: AtomicPtr<HashMap<String, User>>,
    internal: &'a T,
}
```

- **InMemoryUserRepository** is a generic struct that implements the UserRepository trait. It uses an AtomicPtr to hold a
  mutable pointer to a HashMap of users, allowing thread-safe access.
  internal is a reference to another UserRepository used to fetch users if they are not found in the cache.

