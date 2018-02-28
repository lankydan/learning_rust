Next up in Rust for Java Devs we have Structs. They are used to hold data within a logical unit that can then be passed to other functions or execute their own methods on the values stored within them. Now this sounds very familiar... Java objects do the same thing. For example if you took a POJO (Plain Old Java Object) you also pass it to other methods or execute the objects own methods. In this nature they alike, but they do have their differences. In this post we will look into creating Structs, retrieving their values, defining their own methods and how to execute them.

## Creating a struct

Let's start with creating a Struct. 
```rust
struct Person {
  first_name: String,
  last_name: String,
  age: u32,
  weight: u32,
  height: u32,
}
```
Thats all there is to it. The name of the struct is `Person` which has 5 fields inside of it. The nice thing about how structs work compared to Java's objects is that no constructor equivalent needs to be defined to create struct. You simply list out the fields and it will just work.

Let's quickly compare the above Rust struct to a Java object.
```java
class Person {
  private String firstName;
  private String lastName;
  private int age;
  private int weight;
  private int height;

  Person(String firstName, String lastName, int age, int weight, int height) {
    this.firstName = firstName;
    this.lastName = lastName;
    this.age = age;
    this.weight = weight;
    this.height = height;
  }
}
```
I'm sure the first thing you will notice is the much greater amount of code that is needed to achieve the same goal. We needed to first list the fields / properties and then write a constructor to create the object.

There are actually still a few more differences between the two just from these two snippets but we'll get to those in a minute. This amount of extra code (boilerplate code) is one of the main criticisms that people have with Java...Leading to the most common word that I hear about Java being "verbose".

So now we have defined a struct, how do we instantiate one to use in our code? Quite simple really.
```rust
let person = Person {
  first_name: String::from("John"),
  last_name: String::from("Doe"),
  age: 50,
  weight: 200,
  height: 180,
};
```
This looks deceptively like the definition of the struct itself. But this time rather than stating what data type is being used for each field we have passed values instead.

And now in Java.
```java
Person person = new Person("John", "Doe", 50, 200, 180);
```
As you can see the names of the fields have been omitted from the code when compared to Rust's structs. This can lead to Java constructors being slightly more difficult to use if there are lots of parameters of the same type as you could mistakenly order the input values and thus creating an object with incorrect values. A possible solution for this is to use the Builder pattern instead of using a constructor directly. Although Rust does not suffer from this problem as you need to name the fields that values are being passed to.

I mentioned above there are a few more differences between the struct and object that we defined earlier, so let's look at them.

What if I want to pass in values into a struct in a different order to how the fields are listed? Easy, the order doesn't matter.
```rust
let person = Person {
  age: 50,
  first_name: String::from("John"),
  height: 180,
  weight: 200,
  last_name: String::from("Doe"),
};
```
What if I wanted to only define some fields? Mmmm, well your out of luck there. All fields need to be initialized when a struct is created. If we tried to compile the code below.
```rust
let person = Person {
  first_name: String::from("John"),
};
```
The compiler will output.
```
error[E0063]: missing fields `age`, `height`, `last_name` and 1 other field in initializer of `Person`
 --> src\main.rs:2:16
  |
2 |   let person = Person {
  |                ^^^^^^ missing `age`, `height`, `last_name` and 1 other field

error: aborting due to previous error

error: Could not compile `blog_post_sandbox`.
```
The error is self explanatory and states what and how many fields are missing. This is a decision Rust has made to make the language safer as it removes the possibility of a Null Pointer Exception / Error popping up as every value has to be set. Java does not have this requirement which can lead to null pointers occurring from using an object's value that is still null.

How do these two points mentioned above differ from Java? In Java, you can input parameters into a constructor in different orders and you also don't need to initialize every field in the object. But, this comes with a catch. You need to create a new constructor for each order or number of parameters that you wish to pass in, or as I mentioned earlier, you could use the Builder pattern.

For example if we wanted to write constructors that would allow the two Rust snippets above to work, we would write.
```java
class Person {
  // fields
  Person(String firstName, String lastName, int age, int weight, int height) {
    // original constructor
  }
  Person(int age, String firstName, int height, int weight, String lastName) {
    // set values
  }
  Person(String firstName) {
    // set firstName
  }
}
```
A `Person` object can now be instantiated 3 different ways, although the last constructor is a bit risky due to `null` values on most of the fields and could lead to null pointer exceptions.

## Accessing struct values

Next up, we will look at retrieving values from a struct.
```rust
let first_name = person.first_name;
``` 
Thats about it. Whereas in Java you would write.
```java
String firstName = person.firstName;
```
This snippet relies on the `firstName` field being public, but this is considered bad practice in Java so you would normally write the below instead.
```java
String firstName = person.getFirstName();
```
This allows you to keep `firstName` private which provides you the flexibility to change the internals of the object without breaking existing code as long as the `getFirstName` is still available.

From my understanding of Rust structs, struct fields are private by default within a module (I know I haven't gone through modules yet). So if we make this simpler, if we ran the last Rust snippet in the same file as the struct was defined, it would work, if it was in a different file it would not. To make the fields publicly accessible we need to add the `pub` modifier onto each field, like so.
```rust
pub struct Person {
  pub first_name: String,
  pub last_name: String,
  age: u32,
  weight: u32,
  height: u32,
}
```
Here, `pub` was added onto the struct itself so it can be referenced from outside the module. The fields `first_name` and `last_name` are also marked with `pub` making them the only fields that could be used outside of the module when using the `Person` struct. This actually causes a problem, we have only 2 public fields, so when we go to construct a new `Person` instance, it will fail. This is the double edged sword of accessibility, if we want to hide some fields to prevent access it also means we can't pass values into them on construction, because technically we don't "know" about them. We will look at getting past this barrier later on. 

If tried to use the struct shown above, I would get the following error.
```
error[E0451]: field `age` of struct `blog_post_sandbox::people::Person` is private
 --> src\main.rs:8:5
  |
8 |     age: 50,
  |     ^^^^^^^ field `age` is private

error[E0451]: field `weight` of struct `blog_post_sandbox::people::Person` is private
 --> src\main.rs:9:5
  |
9 |     weight: 200,
  |     ^^^^^^^^^^^ field `weight` is private

error[E0451]: field `height` of struct `blog_post_sandbox::people::Person` is private
  --> src\main.rs:10:5
   |
10 |     height: 180,
   |     ^^^^^^^^^^^ field `height` is private

error: aborting due to 3 previous errors
```
## Mutability

If you wanted to change the value of a struct field after it's original creation, it must be marked with `mut` making it mutable. In Rust you cannot choose which fields are mutable and which are not, either all the fields are mutable or none are.
```rust
let mut person = Person {
  first_name: String::from("John"),
  last_name: String::from("Doe"),
  age: 50,
  weight: 200,
  height: 180,
};
person.first_name = String::from("Bob");
```
This will compile and when ran will change the `first_name` fields value from "John" to "Bob".

Earlier we talked about visibility of a struct from outside of it's module, we could use the same concept to hide fields from everyone else except for the fields that we accept as mutable.

Just to prove I'm not lying, if we omitted the `mut` keyword and wrote the below instead.
```rust
let person = Person {
  first_name: String::from("John"),
  last_name: String::from("Doe"),
  age: 50,
  weight: 200,
  height: 180,
};
person.first_name = String::from("Bob");
```
You would get the following compiler error.
```
error[E0594]: cannot assign to immutable field `person.first_name`
 --> src\main.rs:9:3
  |
2 |   let person = Person {
  |       ------ consider changing this to `mut person`
...
9 |   person.first_name = String::from("Bob");
  |   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot mutably borrow immutable field

error: aborting due to previous error
```
Comparing this to Java, we would normally use a setter if we wanted to make changes to our objects. Like so.
```java
person.setFirstName("Bob");
```
Using setters also makes it easy to control which fields we wish to be mutable. If we don't a setter, then the field's value can't change. Simple.

## Methods and Associated Functions

We have now got to the point where we can make our structs actually do things.

I personally think how methods are written in Rust are very different to those in Java. You can form your own opinion once we have gone through this section.

To define methods in Rust we first need to create an implementation block.
```rust
struct Person {
  first_name: String,
  last_name: String,
  age: u32,
  weight: u32,
  height: u32,
}

impl Person {
  // methods go here
}
```
It looks almost the same as the code we have seen throughout this post but with the addition of an implementation block that is created using the `impl` keyword along with the struct's name (in this case `Person`). 

Now we can start adding some methods into the block. I will omit the creation of the struct's fields from code snippets for now.
```rust
impl Person {
  pub fn full_name(&self) -> String {
    [&self.first_name.to_string(), " ", &self.last_name.to_string()].concat()
  }
}
```
Here we have a method that is tied to the struct that it is invoked on. `&self` is what distinguishes this as an instance method as it has access to it's own values. `self` and `&mut self` can be used instead while keeping it as an instance method. I still need to go into what these syntaxes mean, but in short, `&self` allows you to use the structs own values without effecting any other code (doesn't take ownership of the `Person` instance), `&mut self` is the same but allows changes to the instance and `self` prevents any code after the method is invoked to use the `Person` instance that was used. Some of this might sound confusing, but that's due to my bad ordering of writing these posts... I should probably write about ownership sometime soon.

The method has also been marked with `pub` so it can be used outside of it's module, it can be removed if access should be more restrictive. 

In general, it looks very similar to normal a Rust function.

To call the this method we need to write.
```rust
let full_name = person.full_name();
```
The `&self` reference is taken as the `person` instance and does not need to be passed into the method manually. If we wanted to pass in another parameter into a method, it would look like.
```rust
impl Person {
  pub fn full_name_with_random_parameter(&self, random_parameter: &str) -> String {
    [&self.first_name.to_string(), " ", &self.last_name.to_string(), random_parameter].concat()
  }
}
```
And invoked by.
```rust
let random_full_name = person.full_name_with_random_parameter("I'm a string");
```
If we take a moment to compare Rust's instance methods to Java's the biggest difference is the passing in of `&self` into the method to be able to access it's fields. In Java you can access an objects owns fields directly or by using the `this` keyword (like `&self` is used in Rust) and does not require any sort of reference to itself to be passed in as a parameter.

The above method would simply look like this when written in Java.
```java
public String fullNameWithRandomParameter(final String randomParameter) {
  return firstName + " " + lastName + randomParameter;
}
```
or
```java
public String fullNameWithRandomParameter(final String randomParameter) {
  return this.firstName + " " + this.lastName + randomParameter;
}
```
So if we pass in `&self` into a method it becomes an instance method, then what happens if we don't? Well, technically they are functions instead of methods, associated functions to be precise because they are functions that are "associated" to a struct. They don't require an instance to be invoked which might sound familiar coming from Java. That's right (I'll assume figured it out), they are like Java's static methods.

So let's look at an example.
```rust
impl Person {
  pub fn new(first_name: String, last_name: String, age: u32, weight: u32, height: u32) -> Person {
    Person {
      first_name: first_name,
      last_name: last_name,
      age: age,
      weight: weight,
      height: height,
    }
  }
}
```
This method is effectively a constructor to create a new `Person` instance from outside of the `Person`'s module where code cannot create an instance without all the `Person` fields being made public. If you recall to earlier in this post (actually quite long ago now), I mentioned that if any struct fields are private then code outside of it's module cannot create any instances of the struct... Well, this is the solution. By using a public constructor function we can still control accessability without restricting where instances can be created.

Comparing this function to the methods we looked at previously, the only real difference is that there is no reference to `&self` and therefore no instance tied to the function.

####### CALLING STATIC FUNCTION CODE

As a Java static method is the equivalent of associated function, let's take a look at one.
```java
public static void create(final String firstName, final String lastName, final int age, final int weight, final int height) {
  return new Person(firstName, lastName, age, weight, height);
}
```
Nothing particularly interesting to say about this, except for the name of the method. I needed to name this method `create` rather than `new` like in the Rust version, because `new` is a keyword in Java for creating new instances via constructors.

One nice feature before we move on. When creating a struct, if you have a variable whose name matches a struct's field directly, it's value will be passed in without a need to directly assign the value to the field. I can't think of a nice way to explain it in English, so we will use the language we understand better, code!
```rust
impl Person {
  pub fn new(first_name: String, last_name: String, age: u32, weight: u32, input_height: u32) -> Person {
    Person {
      first_name
      last_name
      age
      weight
      height: input_height,
    }
  }
}
```
As you can see `first_name`, `last_name`, `age` and `weight` are passed in as parameters whose values are then used directly in creating the `Person` struct. `input_height` does not match up and therefore still needs to be passed in manually.