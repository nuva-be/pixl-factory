---
name: migration-plan
version: 0.1.0
description: "Plan safe database and code migrations with rollback strategy, dependency analysis, and phased steps, output to MIGRATION-PLAN.md with a pre-flight checklist. Use to \"plan this migration\" or \"write a rollback strategy\"."
allowed-tools: Read, Write, Bash, Glob, Grep
argument-hint: "<description of the migration or path to schema changes>"
---

## Capability matrix

```
┌─────────────────────────────────────────────────────────────────┐
│                      /migration-plan                            │
├─────────────────────────────────────────────────────────────────┤
│  STANDALONE (always works)                                      │
│  ✓ Read local schema files (Prisma, Alembic, SQLAlchemy, SQL)   │
│  ✓ Grep callsites and codepaths that depend on changed entities │
│  ✓ Produce phased plan + rollback strategy in MIGRATION-PLAN.md │
├─────────────────────────────────────────────────────────────────┤
│  SUPERCHARGED (when these are available)                        │
│  + Supabase MCP (`list_tables`, `list_migrations`, `get_logs`,  │
│    `get_advisors`): introspect live schema, prior migrations,   │
│    and runtime advisors so the plan accounts for production     │
│    drift rather than just repo state                            │
│  + GitHub MCP / `gh` CLI: link the migration plan to its PR     │
│    and flag prior migration PRs touching the same tables        │
└─────────────────────────────────────────────────────────────────┘
```

When no live-DB connector is available, planning is sound but conservative — it can't see production drift, only repo schema. Supabase MCP narrows that gap.

## Decision rules from the SWE classics

This skill applies decision rules distilled from the following books (vendored under `references/books/`):

- **Designing Data-Intensive Applications** (Kleppmann) — [mini](../../references/books/designing-data-intensive-applications/designing-data-intensive-applications.mini.md): schema evolution (backward + forward compatibility), encoding formats, dual-write hazards. Use when designing expand-migrate-contract phases and judging breaking-vs-additive changes.
- **Release It!** (Nygard) — [mini](../../references/books/release-it/release-it.mini.md): Decouple Releases from Deployment, feature toggles, dark launches. Use when sequencing code + schema phases so neither blocks rollback.

Cite the book + technique (e.g. "Decouple Releases from Deployment — Nygard") when phases differ from the naive deploy-and-migrate order.

## Overview

Migration planning pipeline: discovery → impact analysis → dependency mapping → phase design → rollback strategy → output plan. Produces a comprehensive `MIGRATION-PLAN.md` document.

## Required References

Before starting, read `references/backend/migration-safety.md` for safety patterns and rollback strategies.

## Step 1: Discovery

1. **Current state**:
   - Identify the database system (Postgres, MySQL, SQLite, MongoDB)
   - Identify the ORM/migration tool (Prisma, Alembic, Knex, TypeORM, raw SQL)
   - List existing migrations and their status
   - Read current schema definition
2. **Target state**:
   - Understand the desired end state from the user's description
   - Identify all affected tables, columns, indexes, and constraints
3. **Environment**:
   - Check for multi-environment setup (dev, staging, prod)
   - Check for read replicas or multi-region
   - Check for concurrent access patterns

## Step 2: Impact Analysis

1. **Schema changes**:
   - New tables/columns to add
   - Columns to modify (type, nullability, default)
   - Columns/tables to remove
   - Index changes
   - Constraint changes
2. **Code changes**:
   - ORM models that need updating
   - Queries that reference changed columns
   - API endpoints that expose changed fields
   - Validation schemas that need updating
3. **Data changes**:
   - Rows that need backfilling
   - Data transformations required
   - Estimated row counts for affected tables

## Step 3: Dependency Mapping

1. **Database dependencies**:
   - FK relationships to/from changed tables
   - Views or materialized views that reference changed columns
   - Stored procedures or triggers affected
   - Indexes that need rebuilding
2. **Code dependencies**:
   - Files that import/use changed models
   - Tests that reference changed schema
   - Seed data that needs updating
3. **Service dependencies**:
   - Other services that read/write to affected tables
   - Message queue consumers that depend on schema
   - Cron jobs or background workers affected

## Step 4: Phase Design

Break the migration into safe, incremental phases:

### Phase Pattern: Expand-Migrate-Contract

1. **Expand** (backwards compatible):
   - Add new columns (nullable or with defaults)
   - Add new tables
   - Add new indexes (CONCURRENTLY)
   - Deploy code that writes to BOTH old and new
2. **Migrate** (data movement):
   - Backfill new columns from old data
   - Run data transformations in batches
   - Verify data integrity
3. **Contract** (cleanup):
   - Deploy code that reads from new only
   - Remove old columns/tables
   - Remove temporary dual-write code

For each phase, specify:
- Pre-conditions (what must be true before starting)
- Steps (ordered list of operations)
- Verification (how to confirm success)
- Estimated duration and risk level

## Step 5: Rollback Strategy

For each phase, define:

1. **Rollback trigger**: What conditions indicate failure
2. **Rollback steps**: Exact commands/operations to undo
3. **Data recovery**: How to restore data if needed
4. **Time limit**: Point of no return (if any)

## Step 6: Output MIGRATION-PLAN.md

Write a `MIGRATION-PLAN.md` file:

```markdown
# Migration Plan: {title}

## Summary
- **Target**: {one-line description}
- **Risk level**: Low / Medium / High / Critical
- **Estimated phases**: N
- **Affected tables**: {list}
- **Affected services**: {list}

## Pre-flight Checklist
- [ ] Database backup verified
- [ ] Rollback scripts tested in staging
- [ ] All dependent services identified
- [ ] Monitoring/alerting in place
- [ ] Maintenance window scheduled (if needed)
- [ ] Team notified

## Phase 1: Expand
### Pre-conditions
- ...
### Steps
1. ...
### Verification
- ...
### Rollback
- ...

## Phase 2: Migrate
[same structure]

## Phase 3: Contract
[same structure]

## Post-Migration
- [ ] Verify all data integrity
- [ ] Confirm no error rate increase
- [ ] Remove feature flags
- [ ] Update documentation
- [ ] Archive rollback scripts
```
