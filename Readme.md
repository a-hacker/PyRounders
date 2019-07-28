# PyRounders

A simple python library for more powerful rounding functions.

## Why not use python built-ins?

Python's math module already has ceil and floor functions but those functions only work in the one's place. If you know you need precision to a different placement, things get a bit tricker. This library makes dealing with that a lot easier by allowing you to specify which position to round too.

## Where's the Python?

This library is written in rust and uses PyO3 to create a python extension. There's no real reason why rust was used. I just wanted to learn how PyO3 works.

## Should I use this library?

Probably not. The way this code was implemented is not amazing and can be pretty fragile. It might be faster than a similar library written in Python though. Yet to be tested.