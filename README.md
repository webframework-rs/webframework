Webframework
============

[![](https://img.shields.io/crates/d/rustc-serialize.svg?style=popout)](https://crates.io/crates/webframework)
[![](https://docs.rs/webframework/badge.svg)](https://docs.rs/webframework/)


`webframework` is a fun to use web-application Rust framework. It's main goals
are: **Safety**, Fun, *Speed*. It tries to achieve this through a combination of
conventions, derive-macros and cli tools.

For a complete example check out [the todo example](https://github.com/webframework-rs/webframework/tree/master/todo-example).

If you are looking for documentation you can find that here: **TODO**

The complete docs can be found here:
[**Documentation**](https://docs.rs/webframework/)

Structure
---------

The `webframework` project is composed of different crates:

- `webframework`: The main crate, and the only one an end-user should need to
  import. It re-exports all needed internal types and also exports a prelude
  users can use.
- `webframework-core`: One of the inner crates, here the basic types of the
  project are defined. This is mainly useful for the developers of the project
  itself as well as addon crates, that want to interface with those defined
  here.
- `webframework-derive`: Here the main derive macros are exported from. It is
  here, where the ergonomic macros are defined.

New crates will surely appear at a later point, however `webframework` should
always be the one that ties everything together.


Contributing
------------

Anyone can potentially contribute to the `webframework` project, be it by
writing features, adding tests or writing documentation. Reporting bugs or
asking for help is also encouraged. Do keep in mind though that this is a
community project, meaning that any person you run into, does this in their free
time. So, be sure to stay respectful and bear with potential longer reply-times.

### Contributing Code

Code Contribution are the most welcome way of adding new features. If you need a
feature yourself, and could add it to the repository, feel free to open a new
pull-request or issue. If you need help, we will try to guide or mentor you on
how this could be done.

### Contributing Tests

Tests are important. They allow us to be certain that features work as intended
and that no breakage occurred after changes. This is why adding new tests is
always welcomed. This can be as simple as a small unit-test or a whole
integration test. If you are unsure what could be tested, you can write a
doc-test! Simply pick any method or type without a test and write an example for
it, demonstrating what effect it has. If you need help, we will guide you.

### Writing Documentation

Documentation allows anyone to learn, or review, how to use this
framework. Thus having a lot of correct documentation is important. Thus, if you
notice any typo, ommission or otherwise unclear documentation, please open an
issue, or even better open a pull request to fix it. If you feel like writing a
bit more, guides or blog posts are also appreciated.


Repository Conventions
----------------------

All new features should enter the master branch through a pull request merged by
bors. This allows us to have an ever-green, building master.

Git commit messages should be in the imperative form, but this is not a hard
rule. However, all commit messages have to be properly formatted with the
following format:

`<type of change>(<changed system>): <description>`

The allowed types are: feat, fix, chore, test, refactor, style, docs

Examples here are:

- `feat(webframework): Add new Frobnarizer component`
- `chore(webframework-core): Rename variables to be consistent in
  Frobnarizer#foo`

Licence
=======

The `webframework` project is licensed under the MIT license.
