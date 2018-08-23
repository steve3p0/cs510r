# HW #1 of CS510 Rust Programming

## calc
*From Homework #1 Instructions...*  
Implement the command-line gcd example of Chapter 2 of the course text. Rename it as calc. Modify it to apply different functions depending  on the first argument passed to it. Functions to be implemented:

* sum
* product
* gcd
* lcm

## What I did...
I implemented all four functions above and made them accessible via the command line. The number of operands, or numbers rather, that can be calculated is variable length.

## How It went...
I got off to a slow start.  I didn't have a working IDE that can debug, compile, run tests, and was integrated with source control.  I have that now.  It's critical for me to have that ease of use.  I first tried JetBrains Rust plugin for Intellij. There were a lot of complication with this.  However, I discovered JetBrain's Clion, which is C/C++ compiler.  The Rust plugin for that works quite well, actually, including the Intellisense.  

Implementing the above features actually went quite fast.  I love the way this assignment taught us how to curry our functions to calculate variable length parameters.  So it seemed like a breeze.  Well, that is, until I tried to implement the command line feature.  This is where I ran into the shock of Rust's strong, static type system.  It took me a *LONG* time to get a simple match condition mapping the command line function (math operation) argument to the appropriate method.

All four methods are in the lib.rs file.  I tried to move most of the code out of the main.rs so that it is testable.

## How I Tested...
So this is another place I got hung up on.  It was easy to create unit tests, but I spent a lot of time trying to figure out HOW I should test and document.  The RUST guide suggests that you can make this README.md from your code. That feature is now officially depracted, which is funny, since it was never implemented.  

In trying wrap this up, I learned a lot about how easy and sensical it is to embed your unit testing in your comments.  That will be very useful.  I was annoyed that I had to write a separate unit test method for every test that I expect an assertion to 'panic' on. But that is not a problem, because a unit test method is generated for each assertion in your method comments. Very nice.

I tested most of the functionality and edge cases.  I did not write any integration tests.  I will try to do that and resubmit before the assignment is graded.  

TODO:
* Review whether passing a vector of arguments to a method from main is a best practice
* Moving tests into code comments
* Moving argv processing completely out of main so that it is testing (most of it is)
* Error Handling - Look into using standard Result type `Result<T, E>`?
* Integration tests

I actually really enjoyed this. Especially how we curried the math functions to handle variable amount of numbers to calculate. 

