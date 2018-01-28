I'm going to do something different today and write about Rust instead of Java. This is something that I want to give a try and if it all goes well I will continue writing some posts about Rust in the future.

I will be writing about learning Rust from the perspective of a Java developer by making comparisons where appropriate to help you (and me) understand what is going on. These will most likely be shorter posts and honestly might not even contain that much information, but hopefully someone finds these sort of posts helpful and makes it all worth it. 

A good resource for learning Rust is the [Rust Book](https://doc.rust-lang.org/book/second-edition/ch01-00-introduction.html) which I am using to teach myself the language and I will keep mentioning it whenever I can because I have found it very helpful.

This post will focus on creating variables. 

The first thing to mention is that Rust variables are immutable by default whereas Java variables are mutable by default. 

Therefore if you tried to compile the Rust code below.

[gist https://gist.github.com/lankydan/e4c32ffd2befeb3acced1ae110f54626 /]

You will get the following compiler error.
<pre>
error[E0384]: re-assignment of immutable variable `x`
 --> src\main.rs:3:3
  |
2 |   let x = 1;
  |       - first assignment to `x`
3 |   x = 2;
  |   ^^^^^ re-assignment of immutable variable
</pre>
The error message is very clear in telling us what has gone wrong. We have altered the value of <code>x</code> which is currently immutable and thus lead to the program blowing up.

If you compared this to the very similar Java code.

[gist https://gist.github.com/lankydan/e65e22340812559d58779865af942db0 /]

This compiles without any issues.

Although if we ran the code below instead.

[gist https://gist.github.com/lankydan/e3b87d901b3fac149ce33c180806834e /]

This does not compile and is the equivalent of the earlier Rust code.

If we wanted to have mutable variables in Rust we need to do a bit more coding, not much though, and add the <code>mut</code> keyword to the variable declaration. Just like how in Java we need to add <code>final</code> to make a variable immutable (which we did above).

Below is what the earlier failed example will look like with the addition of <code>mut</code>.

[gist https://gist.github.com/lankydan/dab1f6ff8d4319f78c99fba680a6a39e /]

Compiling this will cause no errors.

Something I am sure you noticed from the earlier examples when coming from writing Java is that the type of the variable in Rust does not need to be included. The compiler is able to figure out it's type from the value it has been given and from then on the type cannot change. This is not something that can currently be done in Java as you are required to declare the type of all variables when created. Below is what the earlier example would look like if we included the type when creating the variable.

[gist https://gist.github.com/lankydan/cec64da9eae9b43bdfd64d13c71a9847 /]

Here we have declared that <code>x</code> is of type <code>u32</code> (unsigned integer). This is required when the type cannot be figured out by the compiler due to many possible types being returned by a function, but for the above example it is not necessary.

Next up we have the concept called Shadowing. Lets look at an example first.

[gist https://gist.github.com/lankydan/abd0a61d03284d32d71409805dc962f3 /]

I personally think this looks a bit wierd and if you tried to do something like that in Java it would not work, as seen below.

[gist https://gist.github.com/lankydan/acbb452809a5a4ba4958f5cc1fa144e2 /]

Outputs the compiler error.

<pre>
Exception in thread "main" java.lang.Error: Unresolved compilation problems: 
	Duplicate local variable x
	Duplicate local variable x
</pre>

Back to looking at he Rust code. What happens is that each <code>let x</code> that occurs after the original will steal the name of <code>x</code> but will still create a new variable behind the scenes. As you can see from the example even though we have used <code>x</code> on both sides the line it will still compile and uses the current value of <code>x</code> before creating a shadowed <code>x</code> with a new value.

By the way if you still wondering the value of <code>x</code> from the example is 4.

Shadowing also allows you to change the type of a value because shadowing is simply creating a new instance each time. Therefore the code below works fine even though the types have changed.

[gist https://gist.github.com/lankydan/7afad106c2cc07060112e81e2cb6e92f /]

Obviously this code is trivial and you wouldn't just create variables and not use them like this but for an example its fine. The compiler will give you a warning though for not making use of the original <code>x</code>.

<pre>
warning: unused variable: `x`
 --> src\main.rs:2:7
  |
2 |   let x = 1;
  |       ^
  |
</pre>

Although a type can be changed when shadowing a variable it cannot be done via <code>mut</code>. As this uses the same variable throughout it's lifetime only it's value can be modified but not it's type. So if you tried to compile similar code to the above example using <code>mut</code> instead you will get a complier error.

[gist https://gist.github.com/lankydan/f763c3b38c6fc95b8af9eae04df12f8f /]

Outputs the error.

<pre>
error[E0308]: mismatched types
 --> src\main.rs:3:7
  |
3 |   x = String::from("I'm not a number anymore");
  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected integral variable, found struct `std::string::String`
  |
  = note: expected type `{integer}`
             found type `std::string::String`
  = help: here are some functions which might fulfill your needs:
          - .capacity()
          - .len()
</pre>

The error tells you that the original type of <code>x</code> was an integer (<code>u32</code>) but found another line that tried to set its value to <code>std::string::String</code> instead. The compiler also gives you suggestions of functions that might be useful incase you just forgot to type something or used the wrong function.

Finally lets look at constants. These are always immutable in Rust as they cannot have <code>mut</code> added to their declarations. They are defined using the <code>const</code> keyword instead of <code>let</code> and can be declared in any scope, including the global scope. They also must be set to a constant expression and therefore cannot be set equal to a function's return value which is computed at runtime.

[gist https://gist.github.com/lankydan/c429069ca2cab0b5df081f0cdc0bee64 /]

The type of the constant must also be included and cannot be inferred from the constants value.

These are similar to using <code>static final</code> values in Java that are used for constant values that can be shared between methods and if made public can be used by other classes. Below is the equivalent of the above code but in this context is scoped to the class it is declared in.

[gist https://gist.github.com/lankydan/adeecec86fb73d61eaccd858078fd3d1 /]

That should do it for a first look into Rust variables. In conclusion we have looked into declaring variables in Rust while comparing them to their Java equivalents and highlighting the differences between the two. The main take away is that in Rust variables are immutable by default and their types can be inferred from their value whereas Java is mutable by default and types must always be declared.

Although this post is quite basic I hope it was somewhat useful, especially for Java developers looking to get started with Rust. As I mentioned earlier on, the [Rust Book](https://doc.rust-lang.org/book/second-edition/ch01-00-introduction.html) is worth having a look at if you want to give Rust a proper try.




