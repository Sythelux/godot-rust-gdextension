use super::sys;

pub mod gen {
    pub mod central {
        pub mod global {}
    }
    pub mod classes {
        pub struct Node {}
        pub struct Resource {}

        pub mod class_macros {}
    }
    pub mod utilities {}
}

/*pub mod callbacks {
    use super::sys;
    use crate::obj::{Base, GodotClass};

    pub unsafe extern "C" fn create<T>(
        _class_userdata: *mut std::ffi::c_void,
    ) -> sys::GDNativeObjectPtr {
        sys::panic_no_godot!(create)
    }

    pub(crate) fn create_custom<T, F>(_make_user_instance: F) -> sys::GDNativeObjectPtr
    where
        T: GodotClass,
        F: FnOnce(Base<T::Base>) -> T,
    {
        sys::panic_no_godot!(create_custom)
    }
}*/

pub mod engine {
    use super::sys;
    use crate::obj::{Gd, GodotClass};

    pub struct Object {}

    pub struct RefCounted {}

    impl RefCounted {
        pub fn init_ref(&self) -> bool {
            sys::panic_no_godot!(RefCounted::init_ref)
        }
        pub fn reference(&self) -> bool {
            sys::panic_no_godot!(RefCounted::reference)
        }
        pub fn unreference(&self) -> bool {
            sys::panic_no_godot!(RefCounted::unreference)
        }
    }

    impl GodotClass for Object {
        type Base = ();
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::DynamicRefCount;
        const CLASS_NAME: &'static str = "";
    }

    impl GodotClass for RefCounted {
        type Base = Object;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::StaticRefCount;
        const CLASS_NAME: &'static str = "";
    }

    pub mod utilities {
        use super::sys;

        pub fn is_instance_id_valid(id: i64) -> bool {
            sys::panic_no_godot!(is_instance_id_valid)
        }
    }

    #[allow(non_camel_case_types)]
    pub mod global {
        use super::sys;

        #[derive(Debug)]
        pub enum PropertyHint {
            PROPERTY_HINT_NONE,
        }

        impl PropertyHint {
            pub fn ord(&self) -> i32 {
                sys::panic_no_godot!(PropertyHint::ord)
            }
        }

        #[derive(Debug)]
        pub enum PropertyUsageFlags {
            PROPERTY_USAGE_DEFAULT,
        }

        impl PropertyUsageFlags {
            pub fn ord(&self) -> i32 {
                sys::panic_no_godot!(PropertyUsageFlags::ord)
            }
        }
    }

    pub(crate) fn debug_string<T: GodotClass>(
        ptr: &Gd<T>,
        f: &mut std::fmt::Formatter<'_>,
        ty: &str,
    ) -> std::fmt::Result {
        sys::panic_no_godot!(Debug)
    }

    pub(crate) fn display_string<T: GodotClass>(
        ptr: &Gd<T>,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        sys::panic_no_godot!(Display)
    }
}
