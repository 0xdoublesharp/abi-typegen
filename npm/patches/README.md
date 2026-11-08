# Legacy dependency patches

These pnpm patches preserve the APIs used by the Hardhat and Ethers v5 test
projects while backporting fixes that have no compatible published release.
Each project's `pnpm-workspace.yaml` declares the patches it needs.

- `zod@3.25.76.patch` rejects arrays and sets above their declared upper bound
  before parsing elements, including asynchronous validation. It covers CommonJS,
  ESM, and TypeScript source. See [Zod issue 6653](https://github.com/colinhacks/zod/issues/6653)
  and [the proposed v3 backport](https://github.com/dokson/zod/commit/9f5b6c52).
- `elliptic@6.6.1.patch` preserves the fixed width of generated RFC 6979 nonces
  during truncation. It retains the existing behavior for caller-supplied nonces.
  See [elliptic issue 321](https://github.com/indutny/elliptic/issues/321) and
  [the proposed upstream fix](https://github.com/indutny/elliptic/pull/345).

Run the regression tests after installing dependencies:

```sh
node --test npm/tests/zod-security.test.mjs
cd e2e/foundry-sample && pnpm run test:security
```

Version-based audits still identify the original package versions. The patches
do not suppress those advisories. Replace a patch when the consuming SDK supports
a fixed upstream release, and keep its behavior tests.
