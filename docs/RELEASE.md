# Making a release

- in a branch:
  - update [CHANGELOG.md](../CHANGELOG.md)
  - update all occurrences of `0.8.1`
  - ship into `main`
- create a new tag:

  ```bash
  git tag v0.8.1 && git push --tags
  ```
- the CI server creates the release fully automatically
