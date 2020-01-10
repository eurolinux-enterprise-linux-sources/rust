// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ops::{Index, IndexMut};

struct Foo {
    x: isize,
    y: isize,
}

impl Index<String> for Foo {
    type Output = isize;

    fn index(&self, z: String) -> &isize {
        if z == "x" {
            &self.x
        } else {
            &self.y
        }
    }
}

impl IndexMut<String> for Foo {
    fn index_mut(&mut self, z: String) -> &mut isize {
        if z == "x" {
            &mut self.x
        } else {
            &mut self.y
        }
    }
}

struct Bar {
    x: isize,
}

impl Index<isize> for Bar {
    type Output = isize;

    fn index<'a>(&'a self, z: isize) -> &'a isize {
        &self.x
    }
}

fn main() {
    let mut f = Foo {
        x: 1,
        y: 2,
    };
    let mut s = "hello".to_string();
    let rs = &mut s;

    println!("{}", f[s]);
    //~^ ERROR cannot move out of `s` because it is borrowed

    f[s] = 10;
    //~^ ERROR cannot move out of `s` because it is borrowed
    //~| ERROR use of moved value: `s`

    let s = Bar {
        x: 1,
    };
    let i = 2;
    let _j = &i;
    println!("{}", s[i]); // no error, i is copy
    println!("{}", s[i]);
}
