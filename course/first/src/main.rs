#![allow(unused)]

use core::num;
use std::{io, vec};
use rand::Rng;
use std::io::{Write, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::collections::HashMap;

mod restaurant;
use crate::restaurant::order_food;

fn main() {
    order_food();
}