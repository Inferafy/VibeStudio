import { describe, expect, it } from "vitest";

import sessionCatalogFixture from "@/sessions/fixtures/session-catalog.json";
import { sessionCatalogSchema } from "@/sessions/types";

describe("session catalog contract", () => {
  it("accepts the shared Rust and TypeScript fixture", () => {
    expect(sessionCatalogSchema.parse(sessionCatalogFixture)).toEqual(
      sessionCatalogFixture,
    );
  });
});
