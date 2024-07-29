# What is a Trait in Rust?
  * Think of a trait in Rust like a set of rules or a contract that different types of things can follow. It defines what actions (methods) those things can do, but it doesn’t actually do them itself. Instead, it tells other types how they can behave.

#### Example: Animals
  * Imagine we have a trait called Animal. This trait might define that any animal can do two things: speak and move.


```bash
trait Animal {
    fn speak(&self);
    fn move_(&self);
}
```


### `Implementing the Trait`
   * Now, let’s say we have two types of animals: Dog and Cat. Both can follow the Animal rules, but they will do things in their own way.

```bash 

struct Dog;
struct Cat;

impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
    
    fn move_(&self) {
        println!("The dog runs!");
    }
}

impl Animal for Cat {
    fn speak(&self) {
        println!("Meow!");
    }
    
    fn move_(&self) {
        println!("The cat leaps!");
    }
}
```

### `Using the Trait`

  * Now, you can use these animals in a way that doesn’t depend on their specific type. You just know they can speak and move because they follow the Animal trait.

```bash 
fn make_animal_speak(animal: &dyn Animal) {
    animal.speak();
}
```

### `Why Use Traits?`
  * Flexibility: You can write code that works with any type that implements a trait, making your code more flexible and reusable.
Organization: Traits help organize code by grouping related behaviors together.
