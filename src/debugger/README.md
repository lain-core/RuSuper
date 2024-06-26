# Debugger

The core of the debugger is as follows:

```
.
├── breakpoints.rs
├── misc.rs
├── out.txt
├── parser.rs
├── parser_data.rs
├── step.rs
└── utils.rs
```

## Parser
`parser.rs` contains the token parser for all debug commands. 

The types of tokens parseable at present are:
1. `HexValue`, as `$`: A marker indicating the value that it is directly attached to is a hex value.
2. `Offset`, as `+`: A marker indicating the value it is directly attached to is an offset to a prior value, this indicates an arithmetic operation.
   1. Currently, only `+` is used. In the future, I'd like to include all of the basic arithmetic operations (`-`, `*`, `/`, `%`).
3. `Divider`: A way to capture white space, delineating marker between two tokens in a stream.
4. `Value(String)`: A contained numeric value represented as a string input from the user.
5. `Tag(String)`: A non-numeric name or indicator represented as a string input from the user, to be interpreted as a variable.

It achieves token parsing by performing several consquent passes; An example flow is as follows:

Assuming that the user inputs the value `"tagname + $0A"`, where `tagname` is a currently existing value of `$808000`:
1. `collect_args()` takes the string from the user, and digests it into a cursory vector of `TokenSeparator`s (A shorthand type definition `DebugTokenStream` is supplied):

```
[TokenSeparator::Value("tagname"), TokenSeparator::Offset, TokenSeparator::HexValue, TokenSeparator::Value("0A")]
```

2. `collect_tags()` takes the stream from `collect_args()`, and converts any non-numeric `Value`s into `Tag`s:

```
[TokenSeparator::Tag("tagname"), TokenSeparator::Offset, TokenSeparator::HexValue, TokenSeparator::Value("0A")]
```

3. `deref_tags()` looks into the provided table, and attempts to find any tags which correspond to a value. If it can find them, it will replace them with their associated value in-place in the stream. Any tags which cannot be dereferenced are assumed to be a tag which is desired to be set from the user, and is placed onto the end of the new stream.

```
[TokenSeparator::HexValue, TokenSeparator::Value("808000"), TokenSeparator::Offset, TokenSeparator::HexValue, TokenSeparator::Value("0A")]
```

4. `apply_modifiers()` condenses the stream of tokens by applying the operators to the values, and then returns it as a final numeric value:

```
Some(0x80800A)
```

The `compute_address_from_args()` function acts as a wrapper consuming a composed token list from `collect_tags()`, performs the calls to deref the tags and apply the modifiers where applicable, and does the error checking for when these are incorrectly managed.

## ParserData
`parser_data.rs` contains the underlying table for maintaining a list of values and tags and it's associated functions.
`breakpoint`, `step`, `watch`, all require use of the parser to perform address arithmetic, and all wish to maintain their own table. Each of them maintains their own `ParserData` to track this information independently.