# Blaze note

Note taking and flashcard genertion using a _minimal_ superset of markdow. All of the markdown magic you love combined with automatic card prompt 
generation for optimal learning from your notes.

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

### Front-back card
```md
{{front of card | back of card}}
```

**Compiles to**:

```html
<span class="card front-back">
    <span class="front">
        front of card
    </span>
    <span class="back">
        back of card
    </span>
</span>
```

--- 

### Reveal card

```md
{{A reveal card is slightly more complicated but can have text 
before ||and|| after the promp}}
```

**Compiles to**:

```html
<span class="card reveal">
    A reveal card is slightly more complicated but can have text before
    <span class="hidden">
        and
    </span>
    after the prompt
</span>
```

---

### List card

```md
{{Some times you wanna have a question that:|>

1. can have

2. several different

3. ordered items}}
```

**Compiles to**:

```html
<span class="card list">
    Some times you wanna have a question that:
    <ol>
        <li><p>can have</p></li>
        <li><p>several different</p></li>
        <li><p>ordered items</p></li>
    </ol>
</span>
```


## Info about this library

This library uses `#[forbid(unsafe_code)]`.
