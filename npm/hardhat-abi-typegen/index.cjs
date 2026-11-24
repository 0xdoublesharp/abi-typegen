/**
 * Hardhat 2 plugin entry point for @0xdoublesharp/hardhat-abi-typegen.
 *
 * Usage in hardhat.config.ts:
 *   import "@0xdoublesharp/hardhat-abi-typegen";
 *
 * This hooks into the "compile" task to auto-generate TypeScript types
 * after every `npx hardhat compile`.
 *
 * Optional config in hardhat.config.ts:
 *   typegen: {
 *     out: "types",          // default: "src/generated"
 *     target: "viem",        // default: "viem"; comma-separated values allowed
 *     wrappers: true,        // default: true
 *     contracts: ["MyToken"], // default: [] (all)
 *   }
 */

/// <reference path="./type-extensions.d.ts" />

const { execFileSync } = require("child_process");
const { createRequire } = require("module");
const { existsSync } = require("fs");
const path = require("path");

const projectRequire = createRequire(path.join(process.cwd(), "package.json"));

function requireHardhatModule(moduleName) {
  try {
    return projectRequire(moduleName);
  } catch {
    return require(moduleName);
  }
}

const { extendConfig, subtask } = requireHardhatModule("hardhat/config");
const { TASK_COMPILE_SOLIDITY_COMPILE_JOBS } = requireHardhatModule(
  "hardhat/builtin-tasks/task-names",
);

// Resolve defaults for the typegen config so hre.config.typegen is always populated.
extendConfig((config, userConfig) => {
  const typegen = userConfig.typegen || {};
  config.typegen = {
    out: typegen.out || "src/generated",
    target: typegen.target !== undefined ? typegen.target : "viem",
    wrappers: typegen.wrappers !== false,
    contracts: typegen.contracts || [],
    exclude: typegen.exclude || [],
    package: typegen.package,
  };
});

const BINARY_NAME = process.platform === "win32" ? "abi-typegen.exe" : "abi-typegen";

// Resolve the binary from @0xdoublesharp/abi-typegen package
const BINARY = (() => {
  try {
    // Try to find the binary via the companion package
    const binPkg = path.dirname(require.resolve("@0xdoublesharp/abi-typegen/package.json"));
    const packagedBinary = path.join(binPkg, "bin", BINARY_NAME);
    if (existsSync(packagedBinary)) {
      return packagedBinary;
    }
  } catch {
    // Continue to local-development and PATH fallbacks below.
  }

  const sourceCheckoutBinary = path.join(__dirname, "..", "..", "target", "debug", BINARY_NAME);
  if (existsSync(sourceCheckoutBinary)) {
    return sourceCheckoutBinary;
  }

  // Fallback: assume it's on PATH.
  return BINARY_NAME;
})();

subtask(TASK_COMPILE_SOLIDITY_COMPILE_JOBS).setAction(async (args, hre, runSuper) => {
  const result = await runSuper(args);

  const config = hre.config.typegen;
  const artifactsDir = hre.config.paths.artifacts;
  const targetValue = Array.isArray(config.target) ? config.target.join(",") : config.target;
  if (typeof targetValue !== "string" || targetValue.trim() === "") {
    throw new Error("abi-typegen: typegen.target must specify at least one target");
  }

  const cliArgs = [
    "generate",
    "--hardhat",
    "--artifacts",
    artifactsDir,
    "--out",
    config.out,
    "--target",
    targetValue,
  ];

  if (!config.wrappers) {
    cliArgs.push("--no-wrappers");
  }

  if (config.package) {
    cliArgs.push("--package", config.package);
  }

  for (const name of config.contracts) {
    cliArgs.push("--contracts", name);
  }

  if (config.exclude.length > 0) {
    cliArgs.push("--exclude", config.exclude.join(","));
  }

  try {
    execFileSync(BINARY, cliArgs, { cwd: hre.config.paths.root, stdio: "inherit" });
  } catch (err) {
    console.error("abi-typegen: generation failed");
    throw err;
  }

  return result;
});
