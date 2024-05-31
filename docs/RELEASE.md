# Making a release

- in a branch:
  - update [CHANGELOG.md](CHANGELOG.md)
  - update all occurrences of `0.6.0`
  - ship into `main`
- create a new tag:

  ```bash
  git tag v0.6.0 && git push --tags
  ```
- the CI server creates a draft release - review and publish it
