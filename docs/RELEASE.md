# Making a release

- in a branch:
  - update [CHANGELOG.md](../CHANGELOG.md)
  - update all occurrences of `0.9.0`
  - ship into `main`
- create a new tag:

  ```bash
  git tag v0.9.0 && git push --tags
  ```
- the CI server creates the release fully automatically
