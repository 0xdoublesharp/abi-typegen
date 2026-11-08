import assert from "node:assert/strict";
import { copyFileSync, mkdirSync, mkdtempSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { test } from "node:test";

const require = createRequire(import.meta.url);
const source = fileURLToPath(new URL("../pnpm-storage.cjs", import.meta.url));

function fixture(t) {
  const base = mkdtempSync(join(tmpdir(), "abi-pnpm-storage-"));
  t.after(() => rmSync(base, { recursive: true, force: true }));
  const repo = join(base, "checkout");
  const module = join(repo, "npm", "pnpm-storage.cjs");
  mkdirSync(dirname(module), { recursive: true });
  copyFileSync(source, module);
  const settings = join(repo, ".cargo", "scratch.local.json");
  return {
    base,
    repo,
    settings,
    hooks: require(module),
    save(value) {
      mkdirSync(dirname(settings), { recursive: true });
      writeFileSync(settings, JSON.stringify(value));
    },
  };
}

test("unconfigured clones keep pnpm defaults and shared overrides", (t) => {
  const data = fixture(t);
  const config = { overrides: { minimatch: "^11.0.5" }, storeDir: "default-store" };
  assert.equal(data.hooks(join(data.repo, "npm")).hooks.updateConfig(config), config);
});

test("saved storage derives a separate virtual store for each pnpm project", (t) => {
  const data = fixture(t);
  const root = join(data.base, "custom $storage's #1");
  data.save({ root });
  const config = { overrides: { ws: "^8.22.2" }, registry: "https://registry.npmjs.org/" };
  for (const project of [
    "npm",
    "e2e/foundry-sample",
    "e2e/hardhat-sample",
    "e2e/hardhat3-sample",
  ]) {
    const resolved = data.hooks(join(data.repo, project)).hooks.updateConfig(config);
    assert.deepEqual(resolved, {
      ...config,
      storeDir: join(root, "cache", "pnpm"),
      cacheDir: join(root, "cache", "pnpm-metadata"),
      virtualStoreDir: join(root, project, "node_modules", ".pnpm"),
    });
  }
});

test("malformed or overlapping storage settings fail before pnpm runs", (t) => {
  const data = fixture(t);
  const hook = data.hooks(join(data.repo, "npm")).hooks;
  for (const value of [
    null,
    {},
    { root: 1 },
    { root: "relative" },
    { root: data.repo },
    { root: join(data.repo, "cache") },
    { root: data.base },
  ]) {
    data.save(value);
    assert.throws(() => hook.updateConfig({}), /absolute path|outside the checkout/);
  }
  writeFileSync(data.settings, "invalid JSON");
  assert.throws(() => hook.updateConfig({}), SyntaxError);
});

test("settings symlinks and projects outside the checkout are rejected", (t) => {
  const data = fixture(t);
  data.save({ root: join(data.base, "storage") });
  assert.throws(() => data.hooks(data.base).hooks.updateConfig({}), /inside the checkout/);
  rmSync(data.settings);
  const external = join(data.base, "external.json");
  writeFileSync(external, JSON.stringify({ root: join(data.base, "storage") }));
  symlinkSync(external, data.settings);
  assert.throws(() => data.hooks(join(data.repo, "npm")).hooks.updateConfig({}), /regular file/);
});

test("a redirected settings directory is rejected", (t) => {
  const data = fixture(t);
  const external = join(data.base, "external");
  mkdirSync(external);
  writeFileSync(
    join(external, "scratch.local.json"),
    JSON.stringify({ root: join(data.base, "storage") }),
  );
  symlinkSync(external, dirname(data.settings), "dir");
  assert.throws(() => data.hooks(join(data.repo, "npm")).hooks.updateConfig({}), /directory/);
});
