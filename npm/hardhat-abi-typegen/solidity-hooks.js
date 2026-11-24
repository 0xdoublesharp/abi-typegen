import { execFileSync } from "node:child_process";
import { existsSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));

const BINARY_NAME = process.platform === "win32" ? "abi-typegen.exe" : "abi-typegen";

const BINARY = (() => {
  try {
    const binPkg = path.dirname(
      fileURLToPath(import.meta.resolve("@0xdoublesharp/abi-typegen/package.json")),
    );
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

  return BINARY_NAME;
})();

function targetArg(target) {
  const value = Array.isArray(target) ? target.join(",") : target;
  if (typeof value !== "string" || value.trim() === "") {
    throw new Error("abi-typegen: typegen.target must specify at least one target");
  }
  return value;
}

function runTypegen(config, artifactsDir, cwd) {
  const cliArgs = [
    "generate",
    "--hardhat",
    "--artifacts",
    artifactsDir,
    "--out",
    config.out,
    "--target",
    targetArg(config.target),
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
    execFileSync(BINARY, cliArgs, { cwd, stdio: "inherit" });
  } catch (err) {
    console.error("abi-typegen: generation failed");
    throw err;
  }
}

export default async () => ({
  processArtifactsAfterSuccessfulBuild: async (context, artifactPaths) => {
    if (artifactPaths.length === 0) {
      return;
    }

    runTypegen(context.config.typegen, context.config.paths.artifacts, context.config.paths.root);
  },
});
