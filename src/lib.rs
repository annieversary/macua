#![feature(marker_trait_attr)]

use std::marker::PhantomData;

pub trait Attribute<T> {
    fn check(_: &dyn Grantable<T>) -> bool {
        false
    }
}

pub struct NoAttribute<T>(T);
pub struct HasAttribute<T, A>(T, PhantomData<A>);

pub trait Grantable<T> {
    fn get_subject(&self) -> &T;
    fn get_subject_mut(&mut self) -> &mut T;

    fn try_grant<A>(self) -> Result<HasAttribute<Self, A>, ()>
    where
        Self: Sized,
        A: Attribute<T>,
    {
        if <A as Attribute<T>>::check(&self) {
            Ok(HasAttribute(self, PhantomData))
        } else {
            Err(())
        }
    }
}

impl<T> Grantable<T> for NoAttribute<T> {
    fn get_subject(&self) -> &T {
        &self.0
    }

    fn get_subject_mut(&mut self) -> &mut T {
        &mut self.0
    }
}
impl<T, U, A> Grantable<T> for HasAttribute<U, A>
where
    U: Grantable<T>,
{
    fn get_subject(&self) -> &T {
        self.0.get_subject()
    }

    fn get_subject_mut(&mut self) -> &mut T {
        self.0.get_subject_mut()
    }
}

#[marker]
pub trait AttributeGranted<T, A>
where
    A: Attribute<T>,
{
}

impl<T, A> AttributeGranted<T, A> for HasAttribute<NoAttribute<T>, A> where A: Attribute<T> {}
impl<T, U, A, B> AttributeGranted<T, A> for HasAttribute<HasAttribute<U, A>, B>
where
    A: Attribute<T>,
    B: Attribute<T>,
    HasAttribute<U, A>: AttributeGranted<T, A>,
{
}
impl<T, U, A, B> AttributeGranted<T, B> for HasAttribute<HasAttribute<U, A>, B>
where
    A: Attribute<T>,
    B: Attribute<T>,
    HasAttribute<U, A>: AttributeGranted<T, A>,
{
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct User {
        enabled: bool,
        deleted: bool,
    }

    struct UserIsEnabled;
    impl Attribute<User> for UserIsEnabled {
        fn check(v: &dyn Grantable<User>) -> bool {
            v.get_subject().enabled
        }
    }
    struct UserIsNotDeleted;
    impl Attribute<User> for UserIsNotDeleted {
        fn check(v: &dyn Grantable<User>) -> bool {
            !v.get_subject().deleted
        }
    }

    trait ActiveUser:
        Grantable<User>
        + AttributeGranted<User, UserIsEnabled>
        + AttributeGranted<User, UserIsNotDeleted>
    {
        fn delete(&mut self) {
            let u: &mut User = self.get_subject_mut();
            u.deleted = true;
        }
    }
    impl<T> ActiveUser for T where
        T: Grantable<User>
            + AttributeGranted<User, UserIsEnabled>
            + AttributeGranted<User, UserIsNotDeleted>
    {
    }

    fn delete_account(mut user: impl ActiveUser) {
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
}
