import { z } from "zod";

export const sessionProviderSchema = z.enum(["claude", "codex"]);
export type SessionProvider = z.infer<typeof sessionProviderSchema>;

export const providerAvailabilitySchema = z.enum([
  "available",
  "partial",
  "unavailable",
]);
export type ProviderAvailability = z.infer<typeof providerAvailabilitySchema>;

export const sessionSummarySchema = z
  .object({
    provider: sessionProviderSchema,
    sessionId: z.string().min(1),
    title: z.string().min(1),
    workingDirectory: z.string().min(1),
    sourcePath: z.string().min(1),
    lastActivityAt: z.number().int(),
  })
  .strict();
export type SessionSummary = z.infer<typeof sessionSummarySchema>;

export const projectSummarySchema = z
  .object({
    projectKey: z.string().min(1),
    displayName: z.string().min(1),
    workingDirectory: z.string().min(1),
    lastActivityAt: z.number().int(),
    sessions: z.array(sessionSummarySchema),
  })
  .strict();
export type ProjectSummary = z.infer<typeof projectSummarySchema>;

export const providerReportSchema = z
  .object({
    provider: sessionProviderSchema,
    availability: providerAvailabilitySchema,
    sessionCount: z.number().int().nonnegative(),
    warnings: z.array(z.string()),
  })
  .strict();
export type ProviderReport = z.infer<typeof providerReportSchema>;

export const sessionCatalogSchema = z
  .object({
    projects: z.array(projectSummarySchema),
    providers: z.array(providerReportSchema),
    scannedAt: z.number().int(),
  })
  .strict();
export type SessionCatalog = z.infer<typeof sessionCatalogSchema>;
