# Making a release

- in a branch:
  - update [CHANGELOG.md](CHANGELOG.md)
  - update all occurrences of `0.0.1`
  - ship into `main`
- create a new tag: `git tag v0.0.1`
- the CI server creates a draft release - review and publish it
