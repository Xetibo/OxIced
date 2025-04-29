use std::any::TypeId;
use std::fmt::Debug;

use iced::advanced::graphics::futures::MaybeSend;

// Taken from https://doc.rust-lang.org/stable/src/core/any.rs.html
// adjusted to fit for iced
pub trait OxiAny: 'static + MaybeSend + Debug + Send + Sync {
    fn type_id(&self) -> TypeId;
}

impl<T: 'static + ?Sized + Debug + MaybeSend + Send + Sync> OxiAny for T {
    fn type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }
}

impl dyn OxiAny {
    #[inline]
    pub fn is<T: OxiAny>(&self) -> bool {
        // Get `TypeId` of the type this function is instantiated with.
        let t = TypeId::of::<T>();
        // Get `TypeId` of the type in the trait object (`self`).
        let concrete = self.type_id();
        // Compare both `TypeId`s on equality.
        t == concrete
    }

    #[inline]
    pub fn downcast_ref<T: OxiAny>(&self) -> Option<&T> {
        if self.is::<T>() {
            // SAFETY: just checked whether we are pointing to the correct type, and we can rely on
            // that check for memory safety because we have implemented Any for all types; no other
            // impls can exist as they would conflict with our impl.
            unsafe { Some(self.downcast_ref_unchecked()) }
        } else {
            None
        }
    }

    #[inline]
    pub fn downcast_mut<T: OxiAny>(&mut self) -> Option<&mut T> {
        if self.is::<T>() {
            // SAFETY: just checked whether we are pointing to the correct type, and we can rely on
            // that check for memory safety because we have implemented Any for all types; no other
            // impls can exist as they would conflict with our impl.
            unsafe { Some(self.downcast_mut_unchecked()) }
        } else {
            None
        }
    }

    #[inline]
    pub unsafe fn downcast_mut_unchecked<T: OxiAny>(&mut self) -> &mut T {
        debug_assert!(self.is::<T>());
        // SAFETY: caller guarantees that T is the correct type
        unsafe { &mut *(self as *mut dyn OxiAny as *mut T) }
    }

    #[inline]
    pub unsafe fn downcast_ref_unchecked<T: OxiAny>(&self) -> &T {
        debug_assert!(self.is::<T>());
        // SAFETY: caller guarantees that T is the correct type
        unsafe { &*(self as *const dyn OxiAny as *const T) }
    }
}

#[test]
fn test_enum_conversion() {
    #[allow(dead_code)]
    #[derive(Debug, PartialEq)]
    enum TestEnum {
        One,
        Two,
    }

    let enum_any = &mut TestEnum::One as &mut dyn OxiAny;
    let comp = if let Some(val) = enum_any.downcast_ref::<TestEnum>() {
        &TestEnum::One == val
    } else {
        false
    };
    assert!(comp)
}

#[test]
fn test_struct_conversion() {
    #[derive(Debug, Default, PartialEq)]
    struct TestStruct {
        one: u32,
        two: u32,
    }

    let struct_any = &mut TestStruct::default() as &mut dyn OxiAny;
    let comp = if let Some(val) = struct_any.downcast_ref::<TestStruct>() {
        &TestStruct::default() == val
    } else {
        false
    };
    assert!(comp)
}
