# Bonds | Contributing | Pull Requests

This page covers what maintainers will generally expect from a pull request.

## Before you open a pull request

Try to make sure your change is:

- focused in scope
- tested appropriately
- documented when needed
- ready for review without unrelated cleanup mixed in

If your change affects the app UI, include screenshots or short recordings when helpful.

## Base branch

Open pull requests against `master` unless a maintainer asks for a different base branch.

## What to include

A strong pull request description usually includes:

- a short summary of the change
- why the change was made
- implementation notes
- how the change was tested
- any follow-up work or known limitations

If the PR is related to an issue or discussion, link it.

## Review-friendly changes

Pull requests are much easier to review when they:

- solve one problem at a time
- keep formatting changes scoped to touched files
- separate refactors from behavior changes when possible
- explain tradeoffs clearly when there are multiple valid approaches

## Review expectations

Please expect review feedback and iteration. That is normal.

When responding to feedback:

- keep the discussion technical and specific
- explain your reasoning when making a tradeoff
- update tests or docs if review changes the implementation
- avoid force-pushing away context reviewers may still need unless the branch is still very early

## Common reasons a PR slows down

These are the usual gremlins:

- unclear scope
- missing tests for behavior changes
- missing docs for user-facing changes
- unrelated changes bundled together
- platform-specific assumptions that were not accounted for
