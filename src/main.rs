use either::Either;
use std::collections::HashMap;
use std::sync::atomic::AtomicPtr;

#[derive(Debug)]
struct User {
    id: String,
    age: u32,
}

trait UserRepository {
    fn search(&self, id: &str) -> Option<Either<User, &mut User>>;
}

#[derive(Debug)]
struct InMemoryUserRepository<'a, T>
where
    T: UserRepository,
{
    users: AtomicPtr<HashMap<String, User>>,
    internal: &'a T,
}

impl<'a, T> InMemoryUserRepository<'a, T>
where
    T: UserRepository,
{
    fn new(users: &mut HashMap<String, User>, internal: &'a T) -> Self {
        InMemoryUserRepository {
            users: AtomicPtr::new(users),
            internal,
        }
    }

    fn get_users(&self) -> Option<&mut HashMap<String, User>> {
        unsafe { self.users.load(std::sync::atomic::Ordering::Acquire).as_mut() }
    }
}

impl<T> UserRepository for InMemoryUserRepository<'_, T>
where
    T: UserRepository,
{
    fn search(&self, id: &str) -> Option<Either<User, &mut User>> {
        let users = self.get_users()?;

        let cache_user = users.get_mut(id);
        if let Some(user) = cache_user {
            return Some(Either::Right(user));
        }

        let users = self.get_users()?;
        if let Some(user) = self.internal.search(id) {
            users.insert(id.to_string(), user.left()?);

            let cache_user = users.get_mut(id);
            if let Some(user) = cache_user {
                return Some(Either::Right(user));
            }
        }

        None
    }
}

#[derive(Debug)]
struct FixedUserRepository;

impl UserRepository for FixedUserRepository {
    fn search(&self, id: &str) -> Option<Either<User, &mut User>> {
        let user = User {
            id: "1".to_string(),
            age: 20,
        };

        Some(Either::Left(user))
    }
}


fn main() {
    let fixed_user_repository = FixedUserRepository;
    let user_repository = InMemoryUserRepository::new(&mut HashMap::new(), &fixed_user_repository);

    let user = user_repository.search("1");
    let user = user_repository.search("1");

    println!("{:?}", user);
}
