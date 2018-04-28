# The Higher/Lower Game

The *higher/lower game* (as I'm calling it, since I haven't been able to find a real name for it)
is a counterintuitive probability puzzle that goes as follows:

- I pick two distinct numbers and write them down on cards.
- I randomly show one of the numbers to you, and you must decide whether it is the higher or lower
  one. Your goal is to win with a probability higher than **0.5**.

The goal seems impossible to achieve, but it actually can be done using the following strategy:
before you are shown the card, pick your own number, and when you are shown one number *A* act as
if the other number is the one you chose (i.e. if the one you are shown is greater than the one you
picked, say that it is the higher of the two; if the one you are shown is smaller than the number
you picked; say that the number you were not shown is higher).

To understand why this works, say that the numbers I generate are *L* and *H* with *L < H*, and you
are randomly shown either *L* or *H*. The number you came up with is *X*. There are three different
possibilities:

1. *L < H < X*. In this case, you will see either *L* or *H* and, because they are both less than
   *X*, you will claim the other is greater. This means you will win with probability **0.5**.
2. *X < L < H*. In this case, you will see either *L* or *H* and, because they are both greater
   than *X*, you will claim they are the greater number. This means that, again, you will win with
   probability **0.5**.
3. *L < X < H*. In this case, if you see *H* it will be greater than *X* so you will conclude
   that it is the greater; if you see *L* it will be less than *X* so you will conclude that it is
   the smaller. This you will always win.

As long as you pick *X* in such a way that it has some chance of lying between any *L* and *H*
(for example, by picking from any
[continuous probability distribution](https://en.wikipedia.org/wiki/Probability_distribution#Continuous_probability_distribution)),
then the probability of winning is slightly higher than 0.5.

I've ignored the slight possibility that the number you are shown happens to be the same one you
picked; in this case, the simplest course is to generate a new *X* (preferably in an automated way
to prevent human bias). Given the infinitude of the reals, however, this scenario is negligibly
unlikely, so I ignore it from a mathematical perspective.

This repository contains a Rust program that simulates this game with both the strategy outlined
above and the strategy to guess randomly. For our purposes, *H*, *L*, and *X* are all chosen from a
normal distribution, but any strategy to generate *H* and *L* should still work. I conjecture that
this strategy wins with probability **2/3**, but I have no idea how this could be proven.

This program simulates **100,000,000** trials, and when built on release mode, takes about 37
seconds to execute on my machine.
