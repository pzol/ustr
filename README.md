
This lib is designed for developer happiness. If you don't won't to know about Unicode, codepoints, chars, bytes, and a lib with a simple to easy Api, the ustr should be right for you.

UString depends on libicu for the conversion between UTF8 and UTF16 and also for converting to upper, lower case.
In the future regex handling will also come from libicu. The rest of the methods are implemented in Rust for more developer happiness.

UStrings are immutable, each action creates a copy currently. Clojure like persistence would be nice for the future or copy on write? 


### Creating a UString
```rust
let u = "föobär".to_u();
let poo = "💩".to_u()
```

### Back to String

```rust
let u = "föobär".to_u();
let x = u.to_s();         // ~"föobär"
let y = (&u).to_owned();  // ~"föobär"

```

### Using in format!, println!

```rust
let u = "föobär".to_u();
let s = format!("{}", u); // ~"föobär"

println!("{}", u);        // just works
```

### Case Conversions

```rust
// to upper case
let u = "Föobär".to_u().to_upper();   // ~"FÖOBÄR"
let e = "łódź".to_u().to_upper();     // ~"ŁÓDŹ"

// to lower case
let u = "FÖOBÄR".to_u();              // ~"föobär"

// to title case
let u = "In a hill there lives a hobbit".to_u().to_title(); // ~"In A Hill There Lives A Hobbit"

// to capital case
let u = "biLBo".to_u().to_capital(); // ~"Bilbo"
```

### Debug a UString
```rust
let u = "foo".to_u().inspect();      // ~"UString {\"foo\", buf: ~[102u16, 111u16, 111u16]}"
```


### Adding 
```rust
let u1 = "foo".to_u();
let u2 = "bar".to_u();
  
let combined = u1 + u2;              // ~"foobar"
let combined = u1.concat(&u2);       // ~"foobar"
```

### Splitting
```rust
let u = "foo bar".to_u();
let words = u.split(" ".to_u());     // ~["foo".to_u(), "bar".to_u()]);

let u = "In a hill, there lives:   a hobbit".to_u();
let words = u.split(" ,:".to_u());   // ~["In", "a", "hill", "there", "lives", "a", "hobbit"])
```

### Join
```rust
let words = ~["foo".to_u(), "bar".to_u()];
let u = words.join(&", ".to_u());     // ~"foo, bar")
```

### Starts_with, Ends_with

```rust
"foobar".to_u().starts_with(&"foo".to_u()); // true
"foobar".to_u().ends_with(&"bar".to_u());   // true

### Slicing
// with len
let u = "foobar".to_u();
u.slice_len(0, 3)               // "foo".to_u()
u.slice_len(3, 99)              // "bar".to_u()
}

// with start, end position
let u = "föobar".to_u();
u.slice(0, 2)                   // "föo".to_u()
u.slice(3, 2)                   // "".to_u()
u.slice(3, 99)                  // "bar".to_u()
u.slice(-3, -1)                 // "bar".to_u()
u.slice(-99, -1)                // "föobar".to_u()
}
```


### To int, float
```rust
"1".to_u().to_i()                // 1
"x".to_u().to_i()                // 0

"1.2".to_u().to_f()             // 1.2f32
"x".to_u().to_f()               // 0f32
```
