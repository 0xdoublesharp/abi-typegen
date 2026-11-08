const { lstatSync, readFileSync } = require("node:fs");
const path = require("node:path");

const repo = path.resolve(__dirname, "..");
const settingsFile = path.join(repo, ".cargo", "scratch.local.json");

function contains(parent, child) {
  const relative = path.relative(parent, child);
  return relative !== ".." && !relative.startsWith(`..${path.sep}`) && !path.isAbsolute(relative);
}

/** Apply optional local storage paths while retaining shared pnpm policy. */
module.exports = function storageHooks(projectDir) {
  return {
    hooks: {
      updateConfig(config) {
        let settings;
        try {
          if (!lstatSync(path.dirname(settingsFile)).isDirectory()) {
            throw new Error("Disposable-storage settings directory must be a real directory");
          }
          if (!lstatSync(settingsFile).isFile()) {
            throw new Error("Disposable-storage settings must be a regular file");
          }
          settings = JSON.parse(readFileSync(settingsFile, "utf8"));
        } catch (error) {
          if (error.code === "ENOENT") return config;
          throw error;
        }
        if (typeof settings?.root !== "string" || !path.isAbsolute(settings.root)) {
          throw new Error("Disposable-storage root must be an absolute path");
        }
        const storage = path.resolve(settings.root);
        if (contains(repo, storage) || contains(storage, repo)) {
          throw new Error("Disposable storage must be outside the checkout, not its ancestor");
        }
        if (!contains(repo, projectDir)) {
          throw new Error("The pnpm project must be inside the checkout");
        }
        return {
          ...config,
          storeDir: path.join(storage, "cache", "pnpm"),
          cacheDir: path.join(storage, "cache", "pnpm-metadata"),
          virtualStoreDir: path.join(
            storage,
            path.relative(repo, projectDir),
            "node_modules",
            ".pnpm",
          ),
        };
      },
    },
  };
};
