#![cfg_attr(windows, feature(abi_vectorcall))]

use ext_php_rs::prelude::*;
use ext_php_rs::zend::ModuleEntry;
use ext_php_rs::{info_table_end, info_table_row, info_table_start};

/// A constant exposed to PHP.
#[php_const]
pub const HELLO_VERSION: &str = "0.1.0";

/// A simple PHP class.
#[php_class]
pub struct Greeter {
    #[php(prop)]
    name: String,
}

#[php_impl]
impl Greeter {
    pub fn __construct(name: String) -> Self {
        Self { name }
    }

    pub fn greet(&self) -> String {
        format!("Hello, {}!", self.name)
    }
}

/// A simple function exposed to PHP.
#[php_function]
pub fn hello_world(name: &str) -> String {
    format!("Hello, {}!", name)
}

/// Used by the `phpinfo()` function and when you run `php -i`.
pub extern "C" fn php_module_info(_module: *mut ModuleEntry) {
    info_table_start!();
    info_table_row!("hello-php-rs", "enabled");
    info_table_row!("version", env!("CARGO_PKG_VERSION"));
    info_table_end!();
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
        .constant(wrap_constant!(HELLO_VERSION))
        .class::<Greeter>()
        .function(wrap_function!(hello_world))
        .info_function(php_module_info)
}
