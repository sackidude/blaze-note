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
<div class="card front-back-card">
    <div class="front">
        <p>front of card<p>
    </div>
    <div class="back">
        <p>back of card</p>
    </div>
</div>
```

--- 

### Reveal card

```md
{{A reveal card is slightly more complicated but can have text 
before ||and|| after the promp}}
```

**Compiles to**:

```html
<div class="card reveal-card">
    <div class="shown">
        <p>A reveal card is slightly more complicated but can have text before</p>
    </div>
    <div class="hidden">
        <p>and</p>
    </div>
    <div class="shown">
        <p>after the prompt</p>
    </div>
</div>
```

---

### List card

```md
{{Some times you wanna have a question that:|||

1. can have

2. several different

3. ordered items}}
```

**Compiles to**:

```html
<div class="card list-card">
    <p>Some times you wanna have a question that:</p>
    <ol>
        <li><p>can have</p></li>
        <li><p>several different</p></li>
        <li><p>ordered items</p></li>
    </ol>
</div>
```


## Info about this library

This library uses `#[forbid(unsafe_code)]`.
