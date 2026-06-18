---
name: council
description: "Four voices (Pragmatist, Purist, Security, Operator) argue a trade-off independently, then a verdict records the majority AND the dissent plus when the dissent wins. Use when a decision has multiple credible paths and no clear winner."
version: 1.0.0
allowed-tools: Read, Grep, Glob, Bash, Task
argument-hint: "<decision question>"
---

## When to use

Pattern ported from ECC (github.com/affaan-m/ECC, MIT). Use council for **decision-making under ambiguity** — not code review, implementation planning, or architecture design. Trigger phrases: "ship now or hold", "monorepo vs polyrepo", "feature flag vs full rollout", "give me dissent", "second opinions", "what are the trade-offs", "go / no-go on X", "which path", "is this worth it".

Use it when ALL of these hold:

- The decision has more than one credible path and no obvious winner.
- The trade-offs cut across delivery, correctness, risk, and operations — not a single axis.
- Conversational anchoring is a real risk (you've been arguing one side for a while).
- A go / no-go call would benefit from adversarial challenge before it's locked in.

# /council — four-voice structured disagreement

A decision-hardening skill. The goal is to make disagreement **legible** before a choice is locked in — surface the strongest argument from four fixed lenses, then record a verdict that names the majority position AND the dissent explicitly, with the conditions under which the dissent would win. Unanimity is not the goal; a decision you can defend (and revisit) is.

## The four voices

Each voice is a fixed lens. They are deliberately in tension — that tension is the product.

| Voice | Optimizes for | The question it always asks |
| --- | --- | --- |
| **Pragmatist** | Shipping value now | "What's the fastest credible path to value, and what does delay actually cost us?" |
| **Purist** | Correctness & maintainability | "What will we regret in six months? Is this the right model, or the convenient one?" |
| **Security** | Risk & attack surface | "How does this get abused? What's the blast radius, the new attack surface, the data exposure?" |
| **Operator** | Run / debug / cost in production | "When this pages at 3am, can we debug it? What does it cost to run, scale, and observe?" |

## Core discipline — argue independently, no peeking

Each voice argues its position **without seeing the other voices' arguments**. This is the anti-anchoring mechanism — if the Operator reads the Pragmatist's case first, it softens its own. Independent arguments keep each lens at full strength.

Two ways to run this, in order of preference:

1. **Parallel subagents (preferred).** Launch each voice as a fresh `Task` with ONLY the decision question + compact context + its role prompt — never the ongoing conversation transcript. The four run concurrently and cannot see each other.
2. **Sequential self-roleplay (fallback when Task is unavailable).** Write each voice's full argument before reading or writing the next. Do not revise an earlier voice after writing a later one. Commit to each position as you write it.

Whichever path, the synthesizer must NOT collapse a voice into agreement to make the verdict tidier. A voice that genuinely agrees says so; a voice with a real objection keeps it.

## Subagent prompt shape

When launching voices as subagents, give each one exactly this shape:

```text
You are the [Pragmatist | Purist | Security | Operator] on a four-voice decision council.

Decision question:
[the one explicit question]

Context:
[only the compact snippets, constraints, or metrics needed — no conversation history]

Your lens: [one line from the table above — what this voice optimizes for]

Respond with:
1. Position — ship / hold / which-path, in 1-2 sentences. Take a side.
2. Reasoning — 3 concise bullets from YOUR lens only.
3. Biggest risk — the one failure mode you most want on record.
4. What would change your mind — the condition under which you'd switch positions.

Be direct. No hedging. Do not try to be balanced — that's the synthesizer's job. Under 250 words.
```

## Workflow

1. **Extract the real question.** Reduce the decision to one explicit prompt: what are we deciding, what constraints bind, what counts as success? If it's vague, ask ONE clarifying question before convening — a fuzzy question produces four fuzzy answers.
2. **Gather only necessary context.** Codebase-specific decision → collect the relevant files/snippets/metrics via Read/Grep, keep it compact. Strategic decision → skip repo snippets unless they change the answer. Anchor on `CONTEXT.md` vocabulary if it exists at repo root.
3. **Launch the four voices independently** (parallel subagents preferred; sequential self-roleplay as fallback). Each gets the question, compact context, and its role prompt — nothing else.
4. **Collect the four raw positions.** Keep them visible. Do not edit them for tone or to manufacture consensus.
5. **Synthesize the verdict** under the bias guardrails below.
6. **Persist only if it changes something real** (see Persistence rule).

## Synthesis — the verdict must record dissent

The synthesizer is both judge and scribe. Apply these guardrails:

- **Count the positions.** State the majority position plainly (e.g. "3 of 4 say ship behind a flag").
- **Record the dissent explicitly — never bury it.** Name which voice dissents and quote its strongest argument. A verdict that hides the minority view has failed, even if the majority is right.
- **State the conditions under which the dissent wins.** This is the most important output. The dissent is not noise — it's the early-warning system. Write the concrete trigger: "Security's hold position wins the moment this endpoint handles unauthenticated input" or "the Operator's objection becomes decisive if traffic exceeds ~10x current volume." If those conditions later come true, the decision should flip.
- **Don't dismiss a voice without saying why.** If you reject the Purist's correctness argument, name the trade-off you're accepting.
- **Two voices aligned against the lean is a real signal.** If two of four converge against your initial instinct, treat it as evidence, not an outvote to ignore.

## Output shape

```markdown
## Council: <short decision title>

**Pragmatist** — <position, 1-2 sentences>
<one line on the why>

**Purist** — <position, 1-2 sentences>
<one line on the why>

**Security** — <position, 1-2 sentences>
<one line on the why>

**Operator** — <position, 1-2 sentences>
<one line on the why>

### Verdict
- **Majority:** <the position N-of-4 voices hold, and the chosen path>
- **Dissent:** <which voice(s) disagree + their strongest single argument — stated, not softened>
- **Dissent wins when:** <concrete, checkable condition(s) under which the minority view becomes correct and the decision should flip>
- **Recommendation:** <the synthesized path, with the trade-off being accepted named out loud>
```

Keep it scannable. The "Dissent wins when" line is mandatory — if you can't write a condition, the dissent was either fully addressed (say so) or not taken seriously (re-run that voice).

## Persistence rule

Do NOT write ad-hoc notes to shadow paths. Persist only when the council **changes** something real:

- The decision becomes long-lived system policy → consider an ADR (apply the three-conditions rule: costly to reverse, non-obvious reasoning, real alternatives evaluated — see `/grill`).
- The outcome belongs in durable memory → record a decision via the memory protocol / `/session-wrap`.
- The decision changes active execution → update the relevant Linear / GitHub issue directly.

Most council sessions end without persistence, and that's correct.

## Multi-round follow-up

Default is one round. If the user wants another, keep the new question focused, include the prior verdict only if necessary, and re-launch the voices clean to preserve the anti-anchoring value. Do not let round two inherit round one's arguments verbatim.

## What this skill is NOT

- Not code review — see `/code-review` for diffs and `/cto-review` for architectural simplification.
- Not implementation planning — see `/task-plan` to break work down once the decision is made.
- Not architecture design — see the `architect` agent for system structure.
- Not adversarial verification of an answer's correctness — see `/grill` to stress-test a plan's assumptions.
- Not a vote-counting machine — the value is legible disagreement, not a tally.

## Related

- `/grill` — stress-test the surviving recommendation's assumptions against the actual code before committing
- `/cto-review` — architectural simplification pass once the path is chosen
- `/task-plan` — decompose the chosen path into ordered work
- `references/patterns/agent-patterns.md` — how parallel-subagent fan-out maps to Anthropic's named agent patterns
