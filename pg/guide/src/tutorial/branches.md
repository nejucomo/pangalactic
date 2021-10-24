# Branches

For more involved projects, it's useful to develop sequences of changes independently from other such sequences. This allows the developer, such as Alice in our case, to approach a goal or explore a certain direction without disrupting work towards different goals or explorations.

In `pg` we can separate these independent lines of development with `branches`.

## Viewing Alice's Branches

In the last chapter, we learned how to view changes against previous history with the `pg info workspace changes` command. If we have multiple independent sequencse of changes, what are those changes relative to?

To answer that, let's ask for all information about Alice's workspace:

```bash
$ pg info workspace
```

Now we see more information about the workspace, including the `branch`. The `branch` is a name for a given sequence of changes, and because we never specified one for Alice's project, we see it used `default`.

## Renaming Branches

Suppose Alice feels that `default` is rather bland, and wants to rename that history to something more descriptive. Let's do that for Alice as follows:

```bash
$ pg branch rename readme
```

This renames the sequence of changes we've created in Alice's project from `default` to `readme`. Let's see that now in the workspace info:

```bash
$ pg info workspace
```

## Creating a New Branch

Let's say Alice has a goal to improve the documentation in `README.md`, and separately wants to start developing a `rust` project. She isn't finished with the changes she intends for the `README.md`, while at the same time she wants to get started on the `rust` code development. This is the kind of situation separate branches help with, so let's create one for Alice:

```bash
$ pg branch new code
```

Let's see what has changed about our workspace:

```bash
$ pg info workspace
```

Now we see the `branch` is `code`, but we haven't made any changes to anything yet.

Let's create a new rust crate for our project:

```bash
$ cargo new mycrate
```

Let's commit all of the generated rust project files onto the `code` branch:

```bash
$ pg commit --message 'Start creating a rust crate.'
```

Now let's see what the history of our changes are:

```bash
$ pg log
```

Here we see the first two revisions, which created and modified the `README.md`. We also see that the `readme` branch currently ends with the second modification to `README.md`. Then, a further change on the `code` branch appears with Alice's latest changes.

## Listing Branches

So far we've learned how to record changes into revisions, view information about the workspace, and create a new branch. We'll need to be able to discover what branches are available either because we've forgotten about previous work or our collaborators have introduced new ones:

```bash
$ pg info branches
```

Here we see both `readme` and `code`.

## Switching Branches

Imagine Alice has worked to develop the code a bit, and now wishes to switch tasks and return to improving the documentation. She would do this by switching branches. Let's do this now:

```bash
$ pg checkout readme
```

We've switched back to the `readme` branch. If we examine the workspace directory, we'll see that the rust code is absent.

Let's simulate Alice improving the docs a bit more:

```bash
$ echo 'This project demonstrates the basic operation of pg.' >> README.md
$ pg commit --message 'Describe even more in the README.md'
```

## Merging Branches

So Alice has a sequence of changes to documentation, and independently a sequence of changes to the code. Let's say Alice realizes that to make further improvements to the documentation, it will need to match changes to the code, so these goals can no longer be developed independently. This is one case where `merging` branches comes in handy.

Alice has decided that the documentation is all technical documentation about the code itself, and it's easier to update that documentation while she's developing the code in tandem. Let's help her by merging the independent documentation changes into the `code` branch:

```bash
$ pg merge --into code
```

Now that we've merged the `readme` branch which was active in our workspace into the `code` branch, let's examine the state of the workspace:

```bash
$ pg info workspace
```

Now we see that the current `branch` is `code`. Let's examine Alice's branches also:

```bash
$ pg info branches
```

Now we only see `code`. What happened to the `readme` branch?

Because merging a branch is often (but not always) the conclusion of a particular line of development, `pg` considers the merged branch to no longer be active. The `pg info branches` command only displays active branches. If you want to see history branches, we need to request those specifically:

```bash
$ pg info branches --merged
```

Now we see `readme` was merged into `code` at a particular revision.

## Next Step

Now we've seen how to create, independently develop, switch, query, and merge branches. With these basic revision control actions in mind, we're ready to turn our attention to collaborating with others, an area in which `pg` is uniquely different from predecessors like `git`.
