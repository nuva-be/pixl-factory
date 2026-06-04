import type { Principal } from "@qltysh/fabro-api-client";

export function testPrincipal(): Principal {
  return {
    kind:        "user",
    identity:    { issuer: "fabro:test", subject: "test-user" },
    login:       "test",
    auth_method: "dev_token",
  };
}
