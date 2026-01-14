# Why concatenative?
I am big fan of languages with simple syntaxes, because they are more elegant and easier to learn, parse and metaprogram. I chose concatenative specifically because of how natural it looks. To me, programming is fundamentally about designing systems with data flowing in complex ways, and forth-style is much better suitied for this than the general way its done in most modern languages.
To provide an example of what I mean, I will illustrate using a common problem in modern languages, where an object has to go through a series of transformations.
1. One way to do so would be to stack a bunch of functions on top of each other, which is quite hard to read

```
meal = serve(mash(boil(peel(potato),10)))
```

2. The other is to split the code into multiple read-and-assign statements, which itself is verbose and uncomfortable to write

```
pure = peel(potato)
cooked = boil(pure, 10)
meal = mash(cooked)
meal = serve(meal)
```

The above code is simply much more elegant in a concatenative style

```
potato peel 10 boil mash serve
```

Concatenative syntax eliminates the need for many complex features like `(brackets)` => no need for operator precedence rules, which suck always and are never consistent.

# Why linear types?
I wanted this project to be light-weight and usable in a wide range of fields, which meant I would not settle for garbage collection. Memory safety has been an interess of mine, and Linear types caught my attention particularly when I was reading about the Austral programming language. One of my main gripes with LT (albeit a little silly), has been that they are just annoying to work with, as most languages are not suited for constant moving of data. Stack based languages have a great advantage in that regard, since working data lives on the shared stack; the difference between consuming and non consuming methods is also a lot more pronounced. Its also much harder to 'forget' to free a resource. Not using a value automatically returns it in a way.

# Why act and fun?
When I was studying functional programming, I realized that the usefullness of pure functions is not the 'pure' part. Rather, its the distinction between pure and impure that is useful. Being able to decouple your logic and state code is how you write good software. Pure(logic) code can be easily understood, tested, optimized with memoization + compile time eval, and impure(state) code is easily recognizable and portable.

# Why `then`, `elect` and `cycle`?
One of my objectives for the syntax was to make it as simple as possible. For example, in C you can make:
1. a pre-check loop `while () {}` 
2. a post-check loop `do {} while ()`
3. a ranged loop using `for (...) {}`
4. an infinite loop `for (;;) {}` or `goto label`

All of the above are just syntatic sugar for an infinite loop, with a special way to describe a `break` condition. We can implement it with `cycle` and `break`.
As for the name, I opted for `cycle` because the statement "Break the cycle" felt natural (for when a loop is to be exited).

The same could be said about conditions in C:
1. single conditional statement `if () {}`
2. dual conditional statement `if () {} else {}`
3. dual conditional expression `? x : y`
4. multiple exclusive conditionals `... else if () {} ...`
5. equality-based selection `switch`

These can be implemented using `then` for simple conditional statements and letting `elect` handle all the others.
```
# 1 #
cond? then: ... ;

# 2 & 3 #
elect: cond? then:
	... ;
	... ;

# 4 & 5#
elect:
	a then: ... ;
	b then: ... ;
	c then: ... ;
	... ;

```

