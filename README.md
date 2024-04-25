# Blaze note

Note taking and flashcard genertion using _minimal_ superset markdow. So feel 
free to use all of the markdown magic you love and automatic card prompt 
generation.

This repository contains blaze_note_parser which is the light and fast note 
parser which can be implemented into any service you wanna use it in. **If you 
want everything already configured for you** you should checkout the 
[website](https://blazenote.com) for a client where you can save your notes and
share with others.

## Getting started

```sh
cargo add blaze_note_parser
```

_Very_ basic usage could look something like this: 

```rust
use blaze_note_parser::parse;
use std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
    let note = parse("# {Hello,| _World_!}")?;
    Ok(())
}
```

## Syntax

**Front-back card**
```md
{{front of card | back of card}}
```

**Reveal card**
```md
{{A reveal card is slightly more complicated but can have text 
before ||and|| after of the promp}}
```

**List card**
```md
{{Some times you wanna have a question that:|||

1. can have

2. several different

3. ordered items}}
```

## Info about this library

This library uses `#[forbid(unsafe_code)]`.