# ffs
parsing and handling of different file formats

This is a library to create parsers in a simple manner.

## Examples
- [json](examples/json/main.rs)
- [yaml](examples/yaml/main.rs)

## Quick Start
Say you want to parse a Point, e.g. `(10, 14)`, the easy way to do that is like this:
```rs
// some code

#[derive(Parsable, Debug)]
struct Point {
	x: Number,
	y: Number
}

pub fn some_func(buffer: &mut CharStream) {
	let value = Point::parse(buffer);
	println!("value: {:#?}", value);
}
```

Note that the attributes of point are of type `Number`, instead of something like `u32`. This is because `Number` stores additional parsing information for the abstract syntax tree, like the [span](#Span).

## Types
This is a list of the types that you can use from this library, with an explanation of each of them.
### CharStream
The `parse` function from the [Parse](#Parse) trait uses this type, instead of a string.

The `CharStream` struct has some additional functionality to deal with parsing:
- It has multiple options on how it deals with whitespace.
- It keeps track of the current position in the buffer.
- It is possible to go to a specific position in the buffer (but only if the new position is after the current one).

#### Creation
To create a `CharStream` you can do the following:
```rs
let value = "Hello, World!"
let stream = CharStream::new(value).build();
```

we call build here because the `new` function returns a [CharStreamBuilder](#CharStreamBuilder)

#### Functions
|name|description|args|
|`new`|creates a `CharStreamBuilder`|`value`: the `String` buffer to create the CharStream from|
|`set_whitespace`|sets the white space mode|`type`: the `WhitespaceType` to set the stream to|
|`position`|returns the current position||
|`indent`|returns the current indent level (indent is only kept track of when `WhitespaceType` is set to `Indent`|
### Position
This is the struct that stores a position in a `CharStream` buffer.
### Span
This is the struct that stores a beginning and an end `Position` from a `CharStream` buffer.
### CharStreamBuilder
This is the struct that is used to create a `CharStream`.
### ParseError
Any error that can be returned by parsing.

## Traits
This is a list of the traits that you can use from this library.
### Parse
Used for any parsable value.
#### Functions
|name|description|args|return type|
|`parse`|try to parse a value of the type that implements the trait|`value: &mut CharStream`|`Result<Self, ParseError>`|
|`span`|get the `Span` of the current object|`&self`|`Span`|
