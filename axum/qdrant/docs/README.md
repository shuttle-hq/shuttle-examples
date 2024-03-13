# Shuttle Docs

## Contributing

If you found an error in our docs, or you simply want to make them better, contributions are always appreciated!

Our docs are powered by [mintlify](https://mintlify.com/). To run them for local development:

Install [their cli](https://www.npmjs.com/package/mintlify):

```bash
yarn install
```

And from the root of the repository, run:

```bash
yarn mintlify dev
```

*PS: This requires Node 18+*

## Contribution docs

Changes to `community/contribute.mdx` should be made in the [main repo](https://github.com/shuttle-hq/shuttle/blob/main/CONTRIBUTING.md),
since they will be automatically implemented here by the `.github/workflows/replicate.yml` workflow.
