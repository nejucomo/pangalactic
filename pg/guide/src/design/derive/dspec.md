# Derivation Specification

A `derivation specification`, or `dspec` for short, defines how pangalactic deterministically creates a `derivation` from source. A `dspec` is a specific directory structure with two components:

- The `executor` defines the process for creating the derivation.
- The `input` provides application or usage specific input to the process.

## Executor Types

An `executor` can be one of two types:

- A `pgwasm` module, or
- An `interpreter spec`.

The `derive` system distinguishes between these two cases based on whether the `executor` link is to a file or to a directory.

### `pgwasm` Executors

A `pgwasm` executor is the fundamental component for all derivations. Ultimately every derivation relies on a `pgwasm` module to execute the derivation process.

A `pgwasm` module executes in a `WASM` environment with a specific constrained system API which only allows reading `store` contents or creating new `store` contents.

The module is initialized with a link to the `dspec` as input and on a successful execution produces an `output` link.

Execution of the `pgwasm` module can fail in two ways: deterministic errors and non-deterministic errors. Deterministic errors can be cached like successful results. Non-deterministic errors occur when the derivation process fails to complete due to unpredictable factors, such as if the process is interrupted or runs out of memory, or if retrieving a link fails to produce the referent data.

For more information about `pgwasm` see FIXME.

### `interpreter spec` Executors

Sometimes it's convenient to define a derivation using higher level constructs than just a bare `pgwasm` module, such as if there are configuration files or source code for a scripting language. The `interpreter spec` executor type facilitates this by bundling a `pgwasm` executor with auxillary files.

An `interpreter spec` is itself a `dspec` directory, where the `executor` refers to some kind of interpreter, and the `input` links to script sources, configuration files, or anything else needed.

When a `pgwasm` executor is executed, it is given a link to the top-level `dspec`. This provides a link to the application specific `input`. It can then traverse the `executor` link to retrieve the `interpreter spec` directory where it can access the `input` of that subdirectory which contains the interpreter script or configuration files.

#### Python Interpreter Example

Suppose we have a Python language interpreter ported into a `pgwasm` module, and we want to define a derivation using a Python script. In this case the application-level `dspec` contains the application `input` and then the `exector` links to an `interpreter spec` where its `executor` is the Python interpreter `pgwasm` module, and it's `input` is the python script source which controls the derivation.

#### Recursive Interpreters

An application's `executor` link may link to an `interpreter spec`. That `interpreter spec` itself may in turn use another interpreter as it's own `executor`. This enables functionality such as bundling a scripting language's standard library at one level, then a script to specify a derivation process at a higher level, and finally the application specific input at the top-level.

#### The `auto-derive` Executor Example

Because pangalactic provides a useful deterministic derivation feature, we can use that to create an `executor` from some source. The obvious approach is to use `derive` on the source code of the `executor` we wish to use, and then to use that result for a second `derive` command on our application-specific project.

Since the first step is deterministic, it would be convenient if users could simply link to the source code of their preferred `executor` rather than the built output. We can construct an `executor` to do this for us, removing the extra step and streamlining our process.

We'll call this hypothetical "meta-executor" `auto-derive`. It is used in a `interpreter spec` where the second-layer `input` is the source code of the desired `executor` itself. The pseudocode for `auto-derive` would look like this, assuming `appspec` is the top-level application-specific `dspec` link:

1. `let appinput = read_child(appspec, 'input')`
1. `let ispec = read_child(appspec, 'executor')`
1. `let execdspec = read_child(ispec, 'input')`
1. `let builtexec = derive(execdspec)`
1. `let newdspec = make_dspec(executor = builtexec, input = appinput)`
1. `let output = derive(newdspec)`

This process builds the actual `executor`, whose link is assigned to the `builtexec` variable from its source, and then uses that result to derive the application-specific output whose link is assigned to the `output` variable.

## Putting It All Together

By combining these compositional components, the full derivation process for a given application-specific `dspec` has two phases:

1. The first phase recursively walks along the `executor` links until a `pgwasm` module is encountered. For example, if the top-level application-specific `dspec` links directly to a `pgwasm` module, that is used directly. Meanwhile, if the top-level application `dspec` links to a Python script `interpreter spec`, and that in turn links to a Python standard library bundle `interpreter spec`, which in turn links to a hypothetical ported Python interpreter `pgwasm` module, then that final module is selected.
1. The second phase executes the found `pgwasm` module and passes a link to the top-level application-specific `dspec`.

Note that this process requires the `pgwasm` module to understand each layer of `interpreter spec` appropriately. In the example of a hypothetical Python interpreter module, it would need to distinguish between the application layer, the script layer, the standard library layer, and the interpreter itself.

## Composition

The example of the three-layer Python environment with application, script, and standard library is hypothetical. It's possible to imagine further extensions: what if different Python package dependencies might be encapsulated in further layers.

This begins to illustrate how the Pangalactic derivation system combines deterministic execution and composition in a powerful way which can provide both "build-like" functionality as well as dependency management.

The full power of Pangalactic dependency management through combining the derive system with `subscriptions` which are outside the scope of the `derivation system` itself.

## The Holy Grail: A Self-Hosted Compiler

The design of the Pangalactic Derivation System is effective at providing deterministic, composable build-like functionality. However, it is underpinned by the presence of the right set of `pgwasm` executor modules for a user's needs.

Currently, these can only be produced outside of the Pangalactic system, such as using the Rust compiler targeting the `pgwasm` platform. The hypothetical example of a Python interpreter would be a boon to the expressivity of derivation specifications, but even in that case the base interpreter itself needs to be produced somehow.

If there were a self-hosting compiler to the `pgwasm` platform, this would close the loop on the potential of the Pangalactic derivation system: the compiler executor could be deterministically derived from the compiler source all within the Pangalactic system. This would allow developers and users to examine, modify, or extend the compiler and surrounding toolchain in a hermetically sealed deterministic environment.
