#!/usr/bin/env node
/**
 * Configure repository rulesets (replaces classic branch protection).
 *
 * Modules publish OCI on push to main (no release-please tags).
 *
 * Usage (org admin):
 *   node .github/scripts/configure-repo-rulesets.mjs
 *
 * Variables:
 *   GITHUB_REPO               — owner/repo (default: PortakiApp/portaki-modules)
 *   CI_APP_ID                 — optional Integration bypass
 *   REQUIRED_REVIEWS          — PR approvals (default: 0)
 *   REMOVE_BRANCH_PROTECTION  — if 0, keep classic protection (default: 1)
 *   DRY_RUN=1                 — print payloads without calling the API
 */

const REPO = process.env.GITHUB_REPO ?? 'PortakiApp/portaki-modules';
const CI_APP_ID = Number.parseInt(process.env.CI_APP_ID ?? '', 10);
const REQUIRED_REVIEWS = Number(process.env.REQUIRED_REVIEWS ?? '0');
const REMOVE_BRANCH_PROTECTION = process.env.REMOVE_BRANCH_PROTECTION !== '0';
const DRY_RUN = process.env.DRY_RUN === '1';

const GITHUB_ACTIONS_APP_ID = 15368;
const CI_QUALITY_CHECK = 'quality';
const LONG_LIVED_BRANCHES = ['main'];

const RULESET_NAMES = {
  integrity: 'portaki-modules: branch integrity',
  main: 'portaki-modules: main integration',
};

const TOKEN = process.env.GITHUB_TOKEN ?? process.env.GH_TOKEN;
if (!TOKEN && !DRY_RUN) {
  console.error('GITHUB_TOKEN or GH_TOKEN required (repo + admin).');
  process.exit(1);
}

const CI_APP_ID_EFFECTIVE = Number.isFinite(CI_APP_ID) ? CI_APP_ID : 0;
const MAIN_INTEGRATION_CHECKS = [{ context: CI_QUALITY_CHECK, integration_id: GITHUB_ACTIONS_APP_ID }];

/**
 * Org admins bypass for direct push. Optional CI App.
 * @param {'always' | 'pull_request'} [mode]
 */
function bypassActors(mode = 'always') {
  /** @type {{ actor_id: number | null, actor_type: string, bypass_mode: string }[]} */
  const actors = [
    {
      actor_id: null,
      actor_type: 'OrganizationAdmin',
      bypass_mode: mode,
    },
  ];
  if (CI_APP_ID_EFFECTIVE > 0) {
    actors.push({
      actor_id: CI_APP_ID_EFFECTIVE,
      actor_type: 'Integration',
      bypass_mode: mode,
    });
  }
  return actors;
}

function pullRequestRule() {
  return {
    type: 'pull_request',
    parameters: {
      required_approving_review_count: REQUIRED_REVIEWS,
      dismiss_stale_reviews_on_push: true,
      require_code_owner_review: false,
      require_last_push_approval: false,
      required_review_thread_resolution: true,
      allowed_merge_methods: ['rebase'],
    },
  };
}

/** @param {{ context: string, integration_id: number }[]} checks @param {boolean} strict */
function requiredStatusChecksRule(checks, strict) {
  return {
    type: 'required_status_checks',
    parameters: {
      strict_required_status_checks_policy: strict,
      required_status_checks: checks,
    },
  };
}

function integrityRules() {
  return [{ type: 'non_fast_forward' }, { type: 'deletion' }];
}

function buildRulesetDefinitions() {
  const branchRefs = LONG_LIVED_BRANCHES.map((branch) => `refs/heads/${branch}`);

  return [
    {
      name: RULESET_NAMES.integrity,
      target: 'branch',
      bypass_actors: bypassActors(),
      conditions: {
        ref_name: {
          include: branchRefs,
          exclude: [],
        },
      },
      rules: integrityRules(),
    },
    {
      name: RULESET_NAMES.main,
      target: 'branch',
      bypass_actors: bypassActors('always'),
      conditions: {
        ref_name: {
          include: ['refs/heads/main'],
          exclude: [],
        },
      },
      rules: [
        ...integrityRules(),
        pullRequestRule(),
        requiredStatusChecksRule(MAIN_INTEGRATION_CHECKS, true),
      ],
    },
  ];
}

async function gh(method, path, body) {
  const url = path.startsWith('http') ? path : `https://api.github.com${path}`;
  if (DRY_RUN) {
    console.log(`[dry-run] ${method} ${path}`, body ? JSON.stringify(body, null, 2) : '');
    return body?.__dryRunResponse ?? {};
  }
  const res = await fetch(url, {
    method,
    headers: {
      Authorization: `Bearer ${TOKEN}`,
      Accept: 'application/vnd.github+json',
      'X-GitHub-Api-Version': '2022-11-28',
      ...(body ? { 'Content-Type': 'application/json' } : {}),
    },
    body: body ? JSON.stringify(body) : undefined,
  });
  if (!res.ok) {
    const text = await res.text();
    throw new Error(`${method} ${path} → ${res.status}: ${text}`);
  }
  if (res.status === 204) return {};
  return res.json();
}

async function listRulesets() {
  const response = await gh('GET', `/repos/${REPO}/rulesets`);
  if (DRY_RUN) return [];
  return Array.isArray(response) ? response : [];
}

async function upsertRuleset(definition) {
  const existing = (await listRulesets()).find((ruleset) => ruleset.name === definition.name);
  const payload = {
    name: definition.name,
    target: definition.target,
    enforcement: 'active',
    bypass_actors: definition.bypass_actors,
    conditions: definition.conditions,
    rules: definition.rules,
  };

  console.log(`→ Ruleset ${definition.name}`);
  console.log(`   target: ${definition.target}`);
  console.log(
    `   bypass: ${
      definition.bypass_actors.length
        ? definition.bypass_actors
            .map((a) => `${a.actor_type}${a.actor_id != null ? `:${a.actor_id}` : ''}(${a.bypass_mode})`)
            .join(', ')
        : 'none'
    }`,
  );
  console.log(`   rules: ${definition.rules.map((rule) => rule.type).join(', ')}`);

  if (existing?.id) {
    await gh('PUT', `/repos/${REPO}/rulesets/${existing.id}`, payload);
    console.log(`   updated (#${existing.id})`);
    return;
  }

  const created = await gh('POST', `/repos/${REPO}/rulesets`, payload);
  console.log(`   created (#${created.id ?? '?'})`);
}

async function removeBranchProtection(branch) {
  console.log(`→ Remove classic branch protection: ${branch}`);
  try {
    await gh('DELETE', `/repos/${REPO}/branches/${encodeURIComponent(branch)}/protection`);
    console.log('   removed');
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    if (message.includes('404')) {
      console.log('   absent — skip');
      return;
    }
    throw error;
  }
}

async function patchRepoSettings() {
  console.log('→ Repo merge settings');
  await gh('PATCH', `/repos/${REPO}`, {
    delete_branch_on_merge: true,
    allow_squash_merge: false,
    allow_merge_commit: false,
    allow_rebase_merge: true,
    allow_update_branch: true,
  });
}

async function main() {
  console.log(`Repo: ${REPO}`);

  await patchRepoSettings();

  for (const definition of buildRulesetDefinitions()) {
    await upsertRuleset(definition);
  }

  if (REMOVE_BRANCH_PROTECTION) {
    for (const branch of LONG_LIVED_BRANCHES) {
      await removeBranchProtection(branch);
    }
  }

  console.log('\nRulesets active — rebase merge only, quality required on main.');
  console.log('OrganizationAdmin can push directly to main.');
  console.log('Done. Settings → Rules → verify portaki-modules:*.');
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
