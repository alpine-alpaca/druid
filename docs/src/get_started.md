# Get started with Druid
*this is outdated, and should be replaced with a walkthrough of getting a simple
app built and running*.

This chapter will walk you through setting up a simple Druid application from start to finish.

## Set up a Druid project
Setting up a project is a simple as creating a new Rust project;
```bash
> cargo new druid-example
```

And then adding Druid as a dependency to Cargo.toml
```toml
[dependencies]
druid = "0.7.0"
```

To show a minimal window with a label replace `main.rs` with this;
```rust, noplaypen
{{#include ../book_examples/src/get_started/hello.rs:hello}}
```
In our main function we create an `AppLauncher`, pass it a `WindowDesc` that wraps build_ui function and launch it. Druid will use our `build_ui` function to build and rebuild our main window every time it needs to refresh. `build_ui` returns a tree of widgets. For now this tree consists of one simple label widget.

This is a very simple example application and it's missing some important pieces. We will add these in the coming few paragraphs.

## Draw more widgets
The first thing we could do to make our example application more interesting is to draw more than one widget. Unfortunately `WindowDesc::new` expects a function that returns only one Widget. We also need a way to tell Druid how to lay-out our widgets.
We solve both these problems by passing in a widget-tree with one single widget at the top. Widgets can have children and widgets higher up in the tree know how to lay-out their children. That way we describe a window as a widget-tree with layout containers as the branches and widgets as the leaves. Our `build_ui` function is then responsible for building this widget tree.

To see how this works we will divide our window in four. We'll have two rows and two columns with a single label in each of the quadrants. We can lay-out our labels using the `Flex` widget.

```rust, noplaypen
{{#include ../book_examples/src/get_started/flex.rs:flex}}
```

This looks nice but the labels on the left are drawn right against the window edge, so we needs some padding. Lets say we also want to center the two bottom labels. Unlike many other UI frameworks, widgets in Druid don't have padding or alignment properties themselves. Widgets are kept as simple as possible.

Features like padding or alignment are implemented in separate widgets. To add padding you simply wrap the labels in a `Padding` widget. Centering widgets is done using the `Align` widget set to `centered`.

```rust, noplaypen
{{#include ../book_examples/src/get_started/padding.rs:padding}}
```

Do not forget to import the new widgets;
```rust, noplaypen
{{#include ../book_examples/src/get_started/padding.rs:padding_imports}}
```

## Application state
We can display a window and draw and position widgets in it. Now it's time to find out how we can tie these widgets to
the rest of our application. First lets see how we can display information from our application in the user interface.
For this we need to define what our application's state looks like.

...

## Handle user input

...

## Putting it all together

...
