# pixl-factory skills

Reusable prompt templates for pixl-factory's **native agent** (the `backend="api"`
LLM nodes and the interactive agent). They live here because pixl-factory discovers
skills from `{git_root}/skills/*/SKILL.md` (see `lib/crates/fabro-agent/src/skills.rs`,
`default_skill_dirs`). Invoke with `/name …` or let the agent call them via `use_skill`.

These are a **curated, engine-agnostic subset** of the [pixl-crew](https://github.com/)
skill library — the workflow templates that work for any coding agent driving a repo,
copied in so the fork is self-contained and portable (no host symlinks).

| Skill | What it does |
|---|---|
| `commit` | Quick conventional commit (+ optional push) |
| `pr-creation` | Branch → commit → PR with a structured body |
| `code-review` | Review a diff for correctness, quality, standards |
| `security-scan` | OWASP-style vulnerability sweep |
| `test-runner` | Run the suite, triage failures |
| `test-writer` | Write unit/integration tests TDD-style |
| `self-review-fix-loop` | Automated review-and-fix cycle before submitting |
| `code-reduction` | Delete dead code, shrink surface area |
| `changelog` | Generate user-facing changelog entries from git history |
| `investigate` | Structured root-cause investigation |
| `runbook` | Produce an operational runbook |
| `migration-plan` | Phased migration plan with checkpoints |
| `council` | Multi-perspective deliberation on a decision |
| `grill` | Adversarial critique of a proposal/plan |

## The full crew is already available via the ACP path

When a node runs `backend="acp"` with `acp.command="npx --yes @zed-industries/claude-code-acp"`,
Claude Code loads the **entire pixl-crew plugin** (14 agents, ~117 skills) natively from the
host config — nothing to bundle. This `skills/` directory is specifically for pixl-factory's
*own* native agent, which doesn't inherit the Claude Code plugin.

## Adding more

Drop any `agentskills.io`-format skill directory here (`<name>/SKILL.md` with at least a
`name:` frontmatter field). To pull another crew skill:

```bash
cp -R <pixl-crew>/plugins/<group>/skills/<name> skills/<name>
```
