# A Webframework for Rust

`Webframework` is born from a simple idea: **Make developing with it fun!**

To achieve this it tries to maximize these concepts:

- Productivity
- Fun
- Safety

And tries to minimize these:

- Extensive Configuration for usual cases
- Repetitive Boilerplate

Of course, it is not possible to create a perfect framework in this situation,
but we try to get as close as possible.

Webframework is heavily opinionated, you are free to open an issue if you wish
to discuss a decision that has been made, but I hope you can understand that not
every proposition can or should be implemented.

## Inspiration

This framework is heavily inspired by the "Ruby on Rails" feeling. It however
does not try to be a carbon copy or even be feature complete out of the box.
However the core concepts of the whole project are something that should
resonate here as well.

# Technical Overview

If you are looking for the documentation you can check that out here:
[**Documentation**](https://docs.rs/webframework). There you will find a
tutorial, alternatively you can look at the
[examples](https://github.com/webframework-rs/webframework/tree/master/examples)
to see how it could be used.

--------

## Routing

The Webframework tries to make routing as easy as possible. To specify a single
'Routing Entry' one has to specify three things:

- List of Restrictions (Is it a GET request? Is it a POST request? Is the user
  logged in?)
- Path ("/about", "/tasks/new")
- Router _or_ Controller

When a request is sent to the Webframework, it first checks whether it matches
the path and passes all the restrictions. Only then is it passed to the next
router or controller.

This means that routing can be seen as a tree of routing paths. Each leaf is a
controller, and all nodes inbetween are router.


# Contributing

If you wish to contribute to the project you can do so by checking the issues,
answering those that have questions, implementing a new feature or by writing
some more documentation.

# Roadmap

Currently the Webframework is still in its inception stage, however here are the
next 'big' milestones it needs in the near future:

- [ ] Complete Routing
    - [ ] Path matching with extracted items
    - [ ] Extend the controller macros to support extracting arguments from a
      request
- [ ] Various extension points
- [ ] Some configurability for logging/other hardcoded items
- [ ] Integrating templating
