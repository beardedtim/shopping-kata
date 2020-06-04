# Shopping Kata

## Overview

This kata aims to be simple enough to explore rust and its idiosyncrasies.

## Setup

If you have `rust` and `cargo` installed, you should be able to clone this and move on.
You can run `cargo test` to run your tests.

## Kata

You own a grocery store and have been keeping all of your transactions in a log book.
You have heard of these fancy-shmancy tools like Square but don't want to fork over the
money for it. You have instead decided to write your own system that can calculate, record,
and retrieve transactions.

### Goals

You may not get all of the features done. That is okay! Focus on the things that are interesting
to you and skip the ones that aren't.

- Create a function that, when given an `order` and a `user` returns a `total` for how much the
order will cost after factoring in
  - Coupons
  - Sales Events
  - 2 for 1 deals
  - Tax
  - etc.

- Create functions that when given a `log book` can return
  - the total number of sales
  - the average item price
  - the lowest price item
  - the highest price item
  - the average order price
  - the lowest order price
  - the highest order price
