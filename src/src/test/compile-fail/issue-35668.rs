// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

fn func<'a, T>(a: &'a [T]) -> impl Iterator<Item=&'a T> {
    a.iter().map(|a| a*a)
    //~^ ERROR binary operation `*` cannot be applied to type `&T`
}

fn main() {
    let a = (0..30).collect::<Vec<_>>();

    for k in func(&a) {
        println!("{}", k);
    }
}
