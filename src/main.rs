use either::Either;
use std::collections::HashMap;
use std::sync::atomic::AtomicPtr;
use std::sync::Arc;
use std::thread;

#[derive(Debug)]
struct User {
    id: String,
    age: u32,
}

trait UserRepository {
    fn search(&self, id: &str) -> Option<Either<User, &mut User>>;
}

#[derive(Debug)]
struct InMemoryUserRepository<T, R>
where
    T: AsRef<R>,
    R: UserRepository,
{
    users: AtomicPtr<HashMap<String, User>>,
    internal: T,
    phantom: std::marker::PhantomData<R>,
}

impl<T, R> InMemoryUserRepository<T, R>
where
    T: AsRef<R>,
    R: UserRepository,
{
    fn new(users: &mut HashMap<String, User>, internal: T) -> Self {
        InMemoryUserRepository {
            users: AtomicPtr::new(users),
            internal,
            phantom: std::marker::PhantomData,
        }
    }

    fn get_users(&self) -> Option<&mut HashMap<String, User>> {
        unsafe { self.users.load(std::sync::atomic::Ordering::Acquire).as_mut() }
    }
}

impl<T, R> UserRepository for InMemoryUserRepository<T, R>
where
    T: AsRef<R>,
    R: UserRepository,
{
    fn search(&self, id: &str) -> Option<Either<User, &mut User>> {
        let users = self.get_users()?;

        let cache_user = users.get_mut(id);
        if let Some(user) = cache_user {
            return Some(Either::Right(user));
        }

        let users = self.get_users()?;
        if let Some(user) = self.internal.as_ref().search(id) {
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
    let fixed_user_repository = Arc::new(FixedUserRepository);
    let user_repository = Arc::new(InMemoryUserRepository::new(&mut HashMap::new(), fixed_user_repository));

    let mut threads = vec![];
    for _ in 0..10 {
        let user_repository = user_repository.clone();
        threads.push(
            thread::spawn(move || {
                println!("{:?}", user_repository.get_users());
                let _ = user_repository.search("1").unwrap();
            })
        );
    }

    for thread in threads {
        thread.join().unwrap();
    }
}
