# skimr
#### Recursive web scraper in Rust

Returns text for each element in a recursive list of tags  

##### Installation:  

```cargo install --git https://github.com/p1g30n/skimr```  

>Needs the Rust Toolchain available at [https://rustup.rs/]().  
> Note that ~/.cargo/bin/ must be in your $PATH.

##### Usage:  
```skimr [website] [selectors 1..n]``` (tag only, tag#id or tag.class)

##### Example:  
``` skimr news.ycombinator.com table#hnmain td.title a.storylink ```

##### Output:

```
Enigma, the Bombe, and Typex
Mathigon – an interactive, personalized mathematics textbook
Apple iPhone SE Available on Apple Store Again
Integer multiplication in time O(n log n) [pdf]
The cortex is a neural network of neural networks
Local Variables · Crafting Interpreters
Fyne: Cross-Platform GUI in Go Based on Material Design
A Meta Lesson
The Elaborate, Dying Art of Hustling for Money at Dave and Buster's
Credder Wants to Create an Equivalent to “Rotten Tomatoes” for News
A Short History of Chaosnet (2018)
M-16: A Bureaucratic Horror Story (1981)
Prince Of Persia Code Review (2013)
Rotating Black Holes May Serve as Gentle Portals for Hyperspace Travel
Ask HN: Best way to test accessibility of a website?
[..]
```