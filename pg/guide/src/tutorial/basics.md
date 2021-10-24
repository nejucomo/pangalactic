# Repository Basics

First let's create a new repository. Recall from the previous chapter we're in a directory called `alice` which represent our heroic hacker Alice's perspective.

## Creating a Repository

```bash
$ pg init
```

## Committing Alice's First Change

Now let's start developing a project:

```bash
$ echo '# My Project' > README.md
```

```bash
$ pg info workspace changes
```

The `info` command shows a lot of information about a repository. We've requested information about the workspace, which is the local directories and files on Alice's filesystem, and specifically the changes our edits have made compared to the prior revision history. Since this is a new project without any history, every file or directory present is a change from "nothing".

We see that it shows `README.md` is a new file. Let's save our results into a new `revision` by committing them:

```bash
$ pg commit --message 'Create a README.md for My Project.'
```

## Committing Subsequent Changes

Next, let's add some more detail to Alice's `README.md`:

```bash
$ echo 'An example project for the pg tutorial.' >> README.md
```

Let's check the workspace info again:

```bash
$ pg info workspace changes
```

This now shows that we've modified Alice's `README.md` compared to the previous `revision` which we created with `pg commit` above.

Let's commit this change into a new revision:

```bash
$ pg commit --message 'Add more detail to README.md.'
```

## Viewing Revision History

Now we've made two revisions, one creating the `README.md` file and one modifying it. Let's see a summary of that history:

```bash
$ pg log
```

This shows the most fundamental operation of revision control with `pg`: creating a repository, then tracking changes to files into a `revision history`.

## Next Steps

Now that we've created a couple of revisions, in the next chapter we'll see how Alice can develop independent sequences of development using `branches`.
