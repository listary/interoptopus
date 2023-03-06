use interoptopus::{ffi_function, ffi_type, Inventory, InventoryBuilder, function, ffi_service, ffi_service_ctor, ffi_service_method, pattern};
use interoptopus::patterns::api_guard::APIVersion;

mod result;
use crate::result::{Error, FFIError};

#[ffi_type(opaque)]
#[repr(C)]
pub struct SearchResult {
    a: i32,
    b: String
}

#[derive(Default)]
#[ffi_type(opaque)]
pub struct SearchResultDetail {
    pub a: String,
    pub b: bool
}

#[ffi_service(error = "FFIError", prefix = "search_result")]
impl SearchResult {
    #[ffi_service_ctor]
    pub fn new() -> Result<Self, Error> {
        Ok(Self {
            a: 123,
            b: "hello".into()
        })
    }

    #[ffi_service_method(on_panic = "undefined_behavior")]
    pub fn get_result_detail(
        &self,
        index: u64,
    ) -> *mut SearchResultDetail {
        println!("get_result_detail {}", unsafe{ self as *const _ as u64 });

        println!("index={}, a={}", index, self.a);

        Box::into_raw(Box::new(SearchResultDetail {
            a: String::default(),
            b: false
        }))
    }
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn my_api_guard() -> APIVersion {
    my_inventory().into()
}

pub fn my_inventory() -> Inventory {
   InventoryBuilder::new()
       .register(function!(my_api_guard))
       .register(pattern!(SearchResult))
       .inventory()
}