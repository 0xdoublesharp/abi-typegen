import assert from "node:assert/strict";
import { createRequire } from "node:module";
import { dirname, join } from "node:path";
import { pathToFileURL } from "node:url";
import { test } from "node:test";

const require = createRequire(import.meta.url);
const hardhat = createRequire(require.resolve("hardhat/package.json"));
const zodRoot = dirname(hardhat.resolve("zod/package.json"));
const modules = [
  ["CommonJS", hardhat("zod")],
  ["ESM", await import(pathToFileURL(join(zodRoot, "v3/index.js")))],
];

// Backport of https://github.com/colinhacks/zod/issues/6653.
for (const [name, z] of modules) {
  for (const mode of ["sync", "async"]) {
    test(`${name} ${mode} upper-bound checks reject before walking elements`, async () => {
      for (const shape of ["array max", "array length", "set max", "set size"]) {
        let visits = 0;
        const visit = () => {
          visits += 1;
          return false;
        };
        const value = z.string().refine(mode === "async" ? async () => visit() : visit);
        const schema =
          shape === "array max"
            ? z.array(value).max(2, "bounded")
            : shape === "array length"
              ? z.array(value).length(2, "bounded")
              : shape === "set max"
                ? z.set(value).max(2, "bounded")
                : z.set(value).size(2, "bounded");
        const input = shape.startsWith("array") ? ["a", "b", "c"] : new Set(["a", "b", "c"]);
        const result =
          mode === "async" ? await schema.safeParseAsync(input) : schema.safeParse(input);
        assert.equal(result.success, false);
        assert.equal(visits, 0, `${shape} must not validate oversized input elements`);
        assert.equal(result.error.issues.length, 1);
        assert.equal(result.error.issues[0].code, "too_big");
        assert.equal(result.error.issues[0].message, "bounded");
      }
    });
  }

  test(`${name} bounded values and lower-bound errors retain their behavior`, () => {
    assert.deepEqual(z.array(z.string()).max(2).parse(["a", "b"]), ["a", "b"]);
    assert.deepEqual(z.array(z.string()).length(2).parse(["a", "b"]), ["a", "b"]);
    assert.deepEqual(
      z
        .set(z.string())
        .max(2)
        .parse(new Set(["a"])),
      new Set(["a"]),
    );
    assert.equal(z.array(z.string()).length(2).safeParse([]).error.issues[0].code, "too_small");
    assert.equal(z.set(z.string()).size(2).safeParse(new Set()).error.issues[0].code, "too_small");
  });
}
