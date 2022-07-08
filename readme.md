# macua

this was an attempt at making [dacquiri](https://github.com/resyncgg/dacquiri) work on stable rust and without funny macros

i did not succeed, so this crate still requires nightly for the `marker_trait_attr` feature. at least it uses *different* nightly features than daiquiri's, so hey i guess this kinda can be counted as a win? the ergonomics are materially worse though, so idk

## example

the following example is lifted out of the only test in `src/lib.rs`:

```rust
// this will be our subject for the day
#[derive(Default)]
struct User {
    enabled: bool,
    deleted: bool,
}

// we define two attributes: `UserIsEnabled` and `UserIsNotDeleted`
// and we implement the check function for them

struct UserIsEnabled;
impl Attribute<User> for UserIsEnabled {
    fn check(v: &dyn Grantable<User>) -> bool {
        // we can use `get_subject` to get a `&User`
        v.get_subject().enabled
    }
}
struct UserIsNotDeleted;
impl Attribute<User> for UserIsNotDeleted {
    fn check(v: &dyn Grantable<User>) -> bool {
        !v.get_subject().deleted
    }
}

// then we can define an entitlement, listing the attribute requirements

trait ActiveUser:
    Grantable<User> // don't pay too much attention to this
    + AttributeGranted<User, UserIsEnabled>
    + AttributeGranted<User, UserIsNotDeleted>
{
    fn delete(&mut self) {
        // we can use `get_subject` and `get_subject_mut`
        let u: &mut User = self.get_subject_mut();
        u.deleted = true;
    }
}
// we have to manually implement the entitlement trait for every T that meets the attribute requirements
impl<T> ActiveUser for T where
    T: Grantable<User>
        + AttributeGranted<User, UserIsEnabled>
        + AttributeGranted<User, UserIsNotDeleted>
{
}

// we can use the entitlement on a function
fn delete_account(mut user: impl ActiveUser) {
    // ... display a confirmation dialog or whatever we need
    user.delete()
}

#[test]
fn it_works() -> Result<(), ()> {
    let user: User = User {
        enabled: true,
        deleted: false,
    };
    let user = NoAttribute(user)
        .try_grant::<UserIsEnabled>()?
        .try_grant::<UserIsNotDeleted>()?;
    delete_account(user);

    Ok(())
}
```

## using

no.
