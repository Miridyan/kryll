# Rust Static Blog Generator

To those that think that the name of the this project is a clever reference to Jekyll, let me tell
you that you're wrong. The name is in fact a reference to [bashyll](https://github.com/unknownloner/bashyll)
which is itself a reference to Jekyll. I liked the simplicity but decided that I wanted to challenge
myself and rewrite it in rust for [my own blog](http://mdit.to).


Note to github users: this is a mirror of the [real repository](https://gitlab.com/Miridyan/kryll).
As of right now, that is the only place I am accepting pull requests and whatnot. I will eventually
make it so that you can commit to either and the changes will be reflected on both, but I haven't gotten
to that point yet.

## Rational

Other than a personal challenge to myself, there really isn't an good reason as to why I decided
to write this as opposed to using a project like bashyll. I wanted to create a simple static blog
generator that would only require a single command to generate all blog pages.

### Why Rust?

The rational of bashyll was to not require an external language in order to run and only required
pandoc for the markdown-to-html conversion stage. I chose rust as it is a fast systems langauge
and because of its formatting features. Rust has excellent build in string formatting and the
`strfmt` library takes it a step further and works as an excellent templating language.

### Goals

The goal of this project is to be as simple to use as possible. It will eventually have a command
interface, but the end-goal is to have everything configured by a config file in the root directory
of the blog project.
