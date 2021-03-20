# What

With all that in mind, what features should skate have

## Fast Compilation/Startup

Starting up skate should feel instantanious. The gold standard here is Go for compilled languages
and python for interprited. It shoud not feel like rust, with a very heavy compiler, or node that
has to load a bunch of dependencys, and JIT them before it can even start webpack

## Dynamicism

The language and runtime should be "dynamic" ([whatever that means](https://xkcd.com/2318/)).

Some ideas

- Recoverability.

	A crash should be non-fatal, but a regular part of skateing, if you try to read a file, and it doesn't
	exist, you shoud be ably to recover this and keep moving, instead of going back to the start.

- Redefinition
- Reflection

The idea here is more programer power. We are not trying to stop nooglers from creating imposibly high
towers of abstraction. This should encourage creativity, recombination, and not the "right way" or millions
of lines that will remain for decades.

## Deffered compliance

The minimal number of lints should block code running. Even if we are sure code will fail at runtime,
we should let it go there, instead of blocking at compile time

This extends to the use of types. While we should have times, they should not exist at runtime, and type
errors should (optionaly) be warnings. Think the JVM, which has both typed java/scala, and untyped clojure


## Repl

While all repl's are great, the best is the console in the browser. Being able to run repl code side by side
with application code is SO COOL, and I wish more languages supported it.

## Debuger.

It should be trivial to see what code is doing. This means a great, build it debuger. Againg, browser dev tools
are the gold standard in this regard.

## Great tooling.

We should want
- Autoformatter.
- IDE autocomlete
- Helpfull errors.

These should all aim to be as good as the best in class (black, RA, Rustc)

## Coding in the small

Skate shoud never be used above projects of about 2000 Lines. Therefor, we shoud not need to focus on large 
scale code organization

## Large Standard library

Everything you are likely to need should be in the Standard library, with little left to the "Ecosystem"
Thing python and go, not rust/javascript
