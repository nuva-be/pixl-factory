import type { Principal } from "@qltysh/fabro-api-client";

/**
 * Shared frontend test fixture mirroring `fabro_types::test_support::test_principal()`.
 */
export const TEST_PRINCIPAL: Principal = {
  kind:        "user",
  identity:    { issuer: "fabro:test", subject: "test-user" },
  login:       "test",
  auth_method: "dev_token",
  avatar_url:  null,
};
