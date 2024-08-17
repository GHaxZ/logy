



# Logy

- [Idea](#idea)
- [Features](#features)
- [Implementation](#implementation)
  

## Idea

**_Logy_** is supposed to be a **_simple_** and **_easy to use_** logging crate.

It achieves this by following these principles:
1. Being usable out of the box, with a sensible default configuration
2. The exposed API to the user is simple
3. The crate has all the features you need and nothing more

The motivation behind this project is, to make logging **_as painless as possible_**.
Need logging? Add this crate and you're good to go!


## Features

Logy is supposed to have the **_basic, required functionality_**. None of the fancy unnecessary stuff. If you're looking for that, this is the **_wrong crate_**.

Current features:
- Console logging
- File logging
- Easy log message styling
- Custom logging hooks

Planned features:
- Multithreading functionality


## Implementation

### Pre-configured logging 

The easiest way to get started with logging is by using the **global, pre-configured logger**.

This can be done by simply calling a set of **macros**.

The global logger can also be accessed via the `logger()` function.


### Custom loggers

Custom loggers can also easily be created by using a builder pattern.

The global and custom created loggers are **_identical_** in their functionality.


### Log customization

Logs can be customized in two ways:
1. [Log components](#log-components)
2. [Log style](#log-style)


#### Log components

Log components are specified on a **_logger basis_**. This means, each logger has a specific set of components from which their log messages are built.

A good way to explain this is by showing which components currently exist with a short explanation:

Currently available:
- **Prefix**
    - Shows which type of log message this is: `[ERROR]`

- **Message**
    - The log message itself: `Something has gone wrong!`

- **Time**
    - The time at which this log occurred: `18:03:19`

- **String**
    - A simple custom string component: `-> example`

- **Spacer**
    - A single whitespace, useful for spacing other components apart.

- **Newline**
    - A newline character (just a n).

So from all of these components, a log message like this can be built:

`18:03:19 [ERROR] Something has gone wrong!
-> example`

So every time something is logged using this logger. The logs will have this formatting


#### Log style

The log style is specified on a **_single log basis_**. This means that each individual log can have its own styling properties.

To explain this, here are all styling properties which can be currently defined:
- color: `Color`
    - The color of the log message in a console output: 
- prefix: `&'static str`
    - The prefix for this log
- color_message: `bool`
    - If the message itself should be colored (by default, only the prefix is colored)

The crate comes with a set of log styles out of the box:
- Info
- Warning
- Error
- Fatal

These are used by the pre-configured logger, but can also be used by custom loggers. 

If desired, custom log styles can be created by utilizing a builder pattern, much like custom loggers work.
