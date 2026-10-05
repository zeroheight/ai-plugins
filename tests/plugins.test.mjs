import { test } from "node:test";
import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { dirname, join, resolve } from "node:path";

const root = resolve(import.meta.dirname, "..");
const readJson = (path) => JSON.parse(readFileSync(join(root, path), "utf8"));
const trackedFiles = (pattern) =>
  execFileSync("git", ["ls-files", pattern], { cwd: root, encoding: "utf8" })
    .split("\n")
    .filter(Boolean);

test("every JSON file parses", () => {
  for (const file of trackedFiles("*.json")) {
    assert.doesNotThrow(() => readJson(file), file);
  }
});

const marketplaces = [
  {
    file: ".claude-plugin/marketplace.json",
    pluginDir: (plugin) => plugin.source,
    manifest: ".claude-plugin/plugin.json",
  },
  {
    file: ".agents/plugins/marketplace.json",
    pluginDir: (plugin) => plugin.source.path,
    manifest: ".codex-plugin/plugin.json",
  },
];

for (const { file, pluginDir, manifest } of marketplaces) {
  test(`${file} lists plugins that exist under the same name`, () => {
    for (const plugin of readJson(file).plugins) {
      const manifestPath = join(pluginDir(plugin), manifest);
      assert.ok(existsSync(join(root, manifestPath)), `missing ${manifestPath}`);
      // Install commands are `<plugin name>@<marketplace name>`, so a mismatch breaks them.
      assert.equal(readJson(manifestPath).name, plugin.name, manifestPath);
    }
  });
}

const pluginManifests = [
  { file: ".cursor-plugin/plugin.json", base: "." },
  { file: "plugins/zeroheight-mcp/.codex-plugin/plugin.json", base: "plugins/zeroheight-mcp" },
];

for (const { file, base } of pluginManifests) {
  test(`${file} points at files that exist`, () => {
    const manifest = readJson(file);
    for (const key of ["logo", "rules", "skills", "mcpServers"]) {
      if (manifest[key] === undefined) continue;
      assert.ok(existsSync(join(root, base, manifest[key])), `${key}: ${manifest[key]}`);
    }
  });
}

// `claude plugin validate` doesn't follow this symlink, so CI validates skills/ directly and this checks the link.
test("claude-code/skills resolves to the shared skills", () => {
  assert.deepEqual(
    readdirSync(join(root, "claude-code/skills")),
    readdirSync(join(root, "skills")),
  );
});

test("files referenced from each SKILL.md exist", () => {
  for (const skill of trackedFiles("skills/*/SKILL.md")) {
    const body = readFileSync(join(root, skill), "utf8");
    for (const [, path] of body.matchAll(/`((?:reference|templates)\/[^`\s]+)`/g)) {
      assert.ok(existsSync(join(root, dirname(skill), path)), `${skill} → ${path}`);
    }
  }
});

test("every MCP config points at the same server", () => {
  const urls = trackedFiles("*mcp.json").map((file) => [
    file,
    readJson(file).mcpServers.zeroheight.url,
  ]);
  const [[, expected]] = urls;
  for (const [file, url] of urls) assert.equal(url, expected, file);
});
