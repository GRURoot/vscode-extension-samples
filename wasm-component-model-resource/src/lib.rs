// Generated by `wit-bindgen` 0.21.0. DO NOT EDIT!
// Options used:
pub mod exports {
  pub mod vscode {
    pub mod example {
      #[allow(clippy::all)]
      pub mod types {
        #[used]
        #[doc(hidden)]
        #[cfg(target_arch = "wasm32")]
        static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
        use super::super::super::super::_rt;
        #[repr(u8)]
        #[derive(Clone, Copy, Eq, PartialEq)]
        pub enum Operation {
          Add,
          Sub,
          Mul,
          Div,
        }
        impl ::core::fmt::Debug for Operation {
          fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
              Operation::Add => {
                f.debug_tuple("Operation::Add").finish()
              }
              Operation::Sub => {
                f.debug_tuple("Operation::Sub").finish()
              }
              Operation::Mul => {
                f.debug_tuple("Operation::Mul").finish()
              }
              Operation::Div => {
                f.debug_tuple("Operation::Div").finish()
              }
            }
          }
        }

        impl Operation{
          pub(crate) unsafe fn _lift(val: u8) -> Operation{
            if !cfg!(debug_assertions) {
              return ::core::mem::transmute(val);
            }

            match val {
              0 => Operation::Add,
              1 => Operation::Sub,
              2 => Operation::Mul,
              3 => Operation::Div,

              _ => panic!("invalid enum discriminant"),
            }
          }
        }


        #[derive(Debug)]
        #[repr(transparent)]
        pub struct Machine{
          handle: _rt::Resource<Machine>,
        }

        type _MachineRep<T> = Option<T>;

        impl Machine{
          /// Creates a new resource from the specified representation.
          ///
          /// This function will create a new resource handle by moving `val` onto
          /// the heap and then passing that heap pointer to the component model to
          /// create a handle. The owned handle is then returned as `Machine`.
          pub fn new<T: GuestMachine>(val: T) -> Self {
            Self::type_guard::<T>();
            let val: _MachineRep<T> = Some(val);
            let ptr: *mut _MachineRep<T> =
            _rt::Box::into_raw(_rt::Box::new(val));
            unsafe {
              Self::from_handle(T::_resource_new(ptr.cast()))
            }
          }

          /// Gets access to the underlying `T` which represents this resource.
          pub fn get<T: GuestMachine>(&self) -> &T {
            let ptr = unsafe { &*self.as_ptr::<T>() };
            ptr.as_ref().unwrap()
          }

          /// Gets mutable access to the underlying `T` which represents this
          /// resource.
          pub fn get_mut<T: GuestMachine>(&mut self) -> &mut T {
            let ptr = unsafe { &mut *self.as_ptr::<T>() };
            ptr.as_mut().unwrap()
          }

          /// Consumes this resource and returns the underlying `T`.
          pub fn into_inner<T: GuestMachine>(self) -> T {
            let ptr = unsafe { &mut *self.as_ptr::<T>() };
            ptr.take().unwrap()
          }

          #[doc(hidden)]
          pub unsafe fn from_handle(handle: u32) -> Self {
            Self {
              handle: _rt::Resource::from_handle(handle),
            }
          }

          #[doc(hidden)]
          pub fn take_handle(&self) -> u32 {
            _rt::Resource::take_handle(&self.handle)
          }

          #[doc(hidden)]
          pub fn handle(&self) -> u32 {
            _rt::Resource::handle(&self.handle)
          }

          // It's theoretically possible to implement the `GuestMachine` trait twice
          // so guard against using it with two different types here.
          #[doc(hidden)]
          fn type_guard<T: 'static>() {
            use core::any::TypeId;
            static mut LAST_TYPE: Option<TypeId> = None;
            unsafe {
              assert!(!cfg!(target_feature = "threads"));
              let id = TypeId::of::<T>();
              match LAST_TYPE {
                Some(ty) => assert!(ty == id, "cannot use two types with this resource type"),
                None => LAST_TYPE = Some(id),
              }
            }
          }

          fn as_ptr<T: GuestMachine>(&self) -> *mut _MachineRep<T> {
            Machine::type_guard::<T>();
            unsafe { T::_resource_rep(self.handle()).cast() }
          }
        }

        /// A borrowed version of [`Machine`] which represents a borrowed value
        /// with the lifetime `'a`.
        #[derive(Debug)]
        #[repr(transparent)]
        pub struct MachineBorrow<'a> {
          rep: *mut u8,
          _marker: core::marker::PhantomData<&'a Machine>,
        }

        impl<'a> MachineBorrow<'a>{
          #[doc(hidden)]
          pub unsafe fn lift(rep: usize) -> Self {
            Self {
              rep: rep as *mut u8,
              _marker: core::marker::PhantomData,
            }
          }

          /// Gets access to the underlying `T` in this resource.
          pub fn get<T: GuestMachine>(&self) -> &T {
            let ptr = unsafe { &mut *self.as_ptr::<T>() };
            ptr.as_ref().unwrap()
          }

          // NB: mutable access is not allowed due to the component model allowing
          // multiple borrows of the same resource.

          fn as_ptr<T: 'static>(&self) -> *mut _MachineRep<T> {
            Machine::type_guard::<T>();
            self.rep.cast()
          }
        }


        unsafe impl _rt::WasmResource for Machine{
          #[inline]
          unsafe fn drop(_handle: u32) {
            #[cfg(not(target_arch = "wasm32"))]
            unreachable!();

            #[cfg(target_arch = "wasm32")]
            {
              #[link(wasm_import_module = "[export]vscode:example/types")]
              extern "C" {
                #[link_name = "[resource-drop]machine"]
                fn drop(_: u32);
              }

              drop(_handle);
            }
          }
        }

        #[doc(hidden)]
        #[allow(non_snake_case)]
        pub unsafe fn _export_constructor_machine_cabi<T: GuestMachine>(arg0: i32,arg1: i32,arg2: i32,) -> i32 {let result0 = Machine::new(T::new(arg0 as u32, arg1 as u32, Operation::_lift(arg2 as u8)));
        (result0).take_handle() as i32
      }
      #[doc(hidden)]
      #[allow(non_snake_case)]
      pub unsafe fn _export_method_machine_execute_cabi<T: GuestMachine>(arg0: i32,) -> i32 {let result0 = T::execute(MachineBorrow::lift(arg0 as u32 as usize).get());
      _rt::as_i32(result0)
    }
    pub trait Guest {
      type Machine: GuestMachine;
    }
    pub trait GuestMachine: 'static {

      #[doc(hidden)]
      unsafe fn _resource_new(val: *mut u8) -> u32
      where Self: Sized
      {
        #[cfg(not(target_arch = "wasm32"))]
        unreachable!();

        #[cfg(target_arch = "wasm32")]
        {
          #[link(wasm_import_module = "[export]vscode:example/types")]
          extern "C" {
            #[link_name = "[resource-new]machine"]
            fn new(_: *mut u8) -> u32;
          }
          new(val)
        }
      }

      #[doc(hidden)]
      fn _resource_rep(handle: u32) -> *mut u8
      where Self: Sized
      {
        #[cfg(not(target_arch = "wasm32"))]
        unreachable!();

        #[cfg(target_arch = "wasm32")]
        {
          #[link(wasm_import_module = "[export]vscode:example/types")]
          extern "C" {
            #[link_name = "[resource-rep]machine"]
            fn rep(_: u32) -> *mut u8;
          }
          unsafe {
            rep(handle)
          }
        }
      }


      fn new(left: u32,right: u32,operation: Operation,) -> Self;
      fn execute(&self,) -> u32;
    }
    #[doc(hidden)]

    macro_rules! __export_vscode_example_types_cabi{
      ($ty:ident with_types_in $($path_to_types:tt)*) => (const _: () = {

        #[export_name = "vscode:example/types#[constructor]machine"]
        unsafe extern "C" fn export_constructor_machine(arg0: i32,arg1: i32,arg2: i32,) -> i32 {
          $($path_to_types)*::_export_constructor_machine_cabi::<<$ty as $($path_to_types)*::Guest>::Machine>(arg0, arg1, arg2)
        }
        #[export_name = "vscode:example/types#[method]machine.execute"]
        unsafe extern "C" fn export_method_machine_execute(arg0: i32,) -> i32 {
          $($path_to_types)*::_export_method_machine_execute_cabi::<<$ty as $($path_to_types)*::Guest>::Machine>(arg0)
        }
      };);
    }
    #[doc(hidden)]
    pub(crate) use __export_vscode_example_types_cabi;

  }

}
}
}
mod _rt {


  use core::fmt;
  use core::marker;
  use core::sync::atomic::{AtomicU32, Ordering::Relaxed};

  /// A type which represents a component model resource, either imported or
  /// exported into this component.
  ///
  /// This is a low-level wrapper which handles the lifetime of the resource
  /// (namely this has a destructor). The `T` provided defines the component model
  /// intrinsics that this wrapper uses.
  ///
  /// One of the chief purposes of this type is to provide `Deref` implementations
  /// to access the underlying data when it is owned.
  ///
  /// This type is primarily used in generated code for exported and imported
  /// resources.
  #[repr(transparent)]
  pub struct Resource<T: WasmResource> {
    // NB: This would ideally be `u32` but it is not. The fact that this has
    // interior mutability is not exposed in the API of this type except for the
    // `take_handle` method which is supposed to in theory be private.
    //
    // This represents, almost all the time, a valid handle value. When it's
    // invalid it's stored as `u32::MAX`.
    handle: AtomicU32,
    _marker: marker::PhantomData<T>,
  }

  /// A trait which all wasm resources implement, namely providing the ability to
  /// drop a resource.
  ///
  /// This generally is implemented by generated code, not user-facing code.
  pub unsafe trait WasmResource {
    /// Invokes the `[resource-drop]...` intrinsic.
    unsafe fn drop(handle: u32);
  }

  impl<T: WasmResource> Resource<T> {
    #[doc(hidden)]
    pub unsafe fn from_handle(handle: u32) -> Self {
      debug_assert!(handle != u32::MAX);
      Self {
        handle: AtomicU32::new(handle),
        _marker: marker::PhantomData,
      }
    }

    /// Takes ownership of the handle owned by `resource`.
    ///
    /// Note that this ideally would be `into_handle` taking `Resource<T>` by
    /// ownership. The code generator does not enable that in all situations,
    /// unfortunately, so this is provided instead.
    ///
    /// Also note that `take_handle` is in theory only ever called on values
    /// owned by a generated function. For example a generated function might
    /// take `Resource<T>` as an argument but then call `take_handle` on a
    /// reference to that argument. In that sense the dynamic nature of
    /// `take_handle` should only be exposed internally to generated code, not
    /// to user code.
    #[doc(hidden)]
    pub fn take_handle(resource: &Resource<T>) -> u32 {
      resource.handle.swap(u32::MAX, Relaxed)
    }

    #[doc(hidden)]
    pub fn handle(resource: &Resource<T>) -> u32 {
      resource.handle.load(Relaxed)
    }
  }

  impl<T: WasmResource> fmt::Debug for Resource<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      f.debug_struct("Resource")
      .field("handle", &self.handle)
      .finish()
    }
  }

  impl<T: WasmResource> Drop for Resource<T> {
    fn drop(&mut self) {
      unsafe {
        match self.handle.load(Relaxed) {
          // If this handle was "taken" then don't do anything in the
          // destructor.
          u32::MAX => {}

          // ... but otherwise do actually destroy it with the imported
          // component model intrinsic as defined through `T`.
          other => T::drop(other),
        }
      }
    }
  }
  pub use alloc_crate::boxed::Box;

  pub fn as_i32<T: AsI32>(t: T) -> i32 {
    t.as_i32()
  }

  pub trait AsI32 {
    fn as_i32(self) -> i32;
  }

  impl<'a, T: Copy + AsI32> AsI32 for &'a T {
    fn as_i32(self) -> i32 {
      (*self).as_i32()
    }
  }

  impl AsI32 for i32 {
    #[inline]
    fn as_i32(self) -> i32 {
      self as i32
    }
  }

  impl AsI32 for u32 {
    #[inline]
    fn as_i32(self) -> i32 {
      self as i32
    }
  }

  impl AsI32 for i16 {
    #[inline]
    fn as_i32(self) -> i32 {
      self as i32
    }
  }

  impl AsI32 for u16 {
    #[inline]
    fn as_i32(self) -> i32 {
      self as i32
    }
  }

  impl AsI32 for i8 {
    #[inline]
    fn as_i32(self) -> i32 {
      self as i32
    }
  }

  impl AsI32 for u8 {
    #[inline]
    fn as_i32(self) -> i32 {
      self as i32
    }
  }

  impl AsI32 for char {
    #[inline]
    fn as_i32(self) -> i32 {
      self as i32
    }
  }

  impl AsI32 for usize {
    #[inline]
    fn as_i32(self) -> i32 {
      self as i32
    }
  }
  extern crate alloc as alloc_crate;
}

use exports::vscode::example::types::Guest;

use crate::exports::vscode::example::types::{ GuestMachine, Operation };

struct CalcMachine {
	left: u32,
	right: u32,
	operation: Operation,
}

impl GuestMachine for CalcMachine {

	fn new(left: u32, right: u32, operation: Operation) -> Self {
		Self { left, right, operation }
	}

	fn execute(&self) -> u32 {
		match self.operation {
			Operation::Add => self.left + self.right,
			Operation::Sub => self.left - self.right,
			Operation::Mul => self.left * self.right,
			Operation::Div => self.left / self.right,
		}
	}
}

impl Drop for CalcMachine {
	fn drop(&mut self) {
		self.left = 0;
		self.right = 0;
	}
}

struct Implementation;
impl Guest for Implementation {
	type Machine = CalcMachine;
}

export!(Implementation);

#[allow(unused_macros)]
#[doc(hidden)]

macro_rules! __export_calculator_impl {
  ($ty:ident) => (self::export!($ty with_types_in self););
  ($ty:ident with_types_in $($path_to_types_root:tt)*) => (
  $($path_to_types_root)*::exports::vscode::example::types::__export_vscode_example_types_cabi!($ty with_types_in $($path_to_types_root)*::exports::vscode::example::types);
  )
}
#[doc(inline)]
pub(crate) use __export_calculator_impl as export;

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.21.0:calculator:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 338] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xd1\x01\x01A\x02\x01\
A\x02\x01B\x09\x01m\x04\x03add\x03sub\x03mul\x03div\x04\0\x09operation\x03\0\0\x04\
\0\x07machine\x03\x01\x01i\x02\x01@\x03\x04lefty\x05righty\x09operation\x01\0\x03\
\x04\0\x14[constructor]machine\x01\x04\x01h\x02\x01@\x01\x04self\x05\0y\x04\0\x17\
[method]machine.execute\x01\x06\x04\x01\x14vscode:example/types\x05\0\x04\x01\x19\
vscode:example/calculator\x04\0\x0b\x10\x01\0\x0acalculator\x03\0\0\0G\x09produc\
ers\x01\x0cprocessed-by\x02\x0dwit-component\x070.201.0\x10wit-bindgen-rust\x060\
.21.0";

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_custom_section_describing_imports() {
  wit_bindgen::rt::maybe_link_cabi_realloc();
}