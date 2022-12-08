# Intro
whole project is based around the cursive root
```
let mut siv = cursive::default();
```

use .add_global_callback for doing call back functions. These functions act on an event like a key press.

TextView shows text

The cursive root uses a stack view, this means that views are stacked on top of eachother

Dialog is a wrapper around another view

Dialog and TextView is pretty common so use Dialog::text() as a short cut

methods are chainable

