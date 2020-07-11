# DAY 7

[Link to the exercice](https://adventofcode.com/2015/day/7)

I was thinking of using an HashMap from the beginning because of the way the instructions were:
```
123 -> b // could be stored in a map with b as the key and 123 as the value.
```
So at first I would have say, let's make an `HashMap<String, u16>`.

The example given was useful to get an idea of how complex it would be:
```
x AND y -> d // uh oh I can't put this in an HashMap<String, u16>
NOT x -> h // I would need to think about the parsing
```

Now of course I'm using Regex because it's much simpler, but it wasn't the case at the beginning...

The first part was to parse the instructions and put them inside a map, I used a Reverse Polish Notation vector as value.
It's not used properly here because at the time I was thinking of getting every dependencies required and merging them into a huge vector,
the RPN would then take care of the priorities of calculation.

But in the second part I was using the HashMap to make recursive calls and calculate the wire `a`,
now the problem is that is was really long (was still processing after 40min) because there was a lot of calculations.

For example I implemented a depth_counter to know how big it was retrieving the values, and it was around 80, so it was a lot of processing.

I struggled with the compiler because I couldn't use the main HashMap as mutable otherwise I would lose the reference, so I took the easiest way: Just make a cache map and share it.

Now it's works like a charm because I respected the Rust borrow rules by taking care of cloning the reference counter BEFORE borrowing the map.

The application is taking around ~45ms to get the wire's value.