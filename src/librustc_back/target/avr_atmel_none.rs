// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use LinkerFlavor;
use target::{Target, TargetOptions, TargetResult};

pub fn target() -> TargetResult {
    Ok(Target {
        llvm_target: "avr-atmel-none".to_string(),
        target_endian: "little".to_string(),
        target_pointer_width: "16".to_string(),
        data_layout: "e-p:16:8-i8:8-i16:8-i32:8-i64:8-f32:8-f64:8-n8-a:8".to_string(),
        arch: "avr".to_string(),
        linker_flavor: LinkerFlavor::Gcc,
        target_os: "none".to_string(),
        target_env: "gnu".to_string(),
        target_vendor: "unknown".to_string(),
        options: TargetOptions {
            // jemalloc is not supported on 16-bit targets.
            exe_allocation_crate: "alloc_system".to_string(),
            lib_allocation_crate: "alloc_system".to_string(),
            .. super::none_base::opts()
        },
    })
}
