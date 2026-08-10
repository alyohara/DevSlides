// The release version lives in three authoritative places with no sync
// mechanism: package.json (drives release tags/names), src-tauri/tauri.conf.json
// (drives artifact filenames), and src-tauri/Cargo.toml (the crate version). If
// they drift, the release tags one version while artifacts build under another,
// and update-nix-sources downloads a file that never exists. Fail the build so
// a mismatch is caught before release.
import fs from "node:fs";
import path from "node:path";
import { execFileSync } from "node:child_process";
import JSON5 from "json5";
import { repoRoot } from "./lib/root.mjs";

const pkg = JSON.parse(
  fs.readFileSync(path.join(repoRoot, "package.json"), "utf8"),
);

// Tauri parses tauri.conf.json as JSON5, so accept comments and trailing
// commas rather than throwing on configs that Tauri itself accepts.
const tauri = JSON5.parse(
  fs.readFileSync(path.join(repoRoot, "src-tauri", "tauri.conf.json"), "utf8"),
);

const cargoToml = fs.readFileSync(
  path.join(repoRoot, "src-tauri", "Cargo.toml"),
  "utf8",
);
const cargoVersion = cargoToml.match(/^version\s*=\s*"([^"]+)"/m)?.[1];

const pkgVersion = pkg.version;
const tauriVersion = tauri.version;

const mismatches = [];
if (pkgVersion !== tauriVersion) {
  mismatches.push(
    `package.json=${pkgVersion} src-tauri/tauri.conf.json=${tauriVersion}`,
  );
}
if (cargoVersion && cargoVersion !== pkgVersion) {
  mismatches.push(
    `package.json=${pkgVersion} src-tauri/Cargo.toml=${cargoVersion}`,
  );
}

// On a tag push, CI checks out the tagged commit detached, so the exact git
// tag is the version the release will be cut under — it must agree too. Branch
// and PR pushes are not on a tagged commit; `git describe` fails there.
let tag = "";
try {
  tag = execFileSync("git", ["describe", "--tags", "--exact-match"], {
    encoding: "utf8",
    stdio: ["ignore", "pipe", "ignore"],
  }).trim();
} catch {
  // Not on a tagged commit — nothing to compare.
}
if (tag.startsWith("v") && tag.slice(1) !== pkgVersion) {
  mismatches.push(`git tag=${tag.slice(1)} package.json=${pkgVersion}`);
}

if (mismatches.length > 0) {
  for (const m of mismatches) {
    console.error(`Version mismatch: ${m}`);
  }
  console.error(
    "Bump all sources together so release tags and artifact filenames agree.",
  );
  process.exit(1);
}

console.log(`Versions in sync: ${pkgVersion}`);
