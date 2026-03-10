import { describe, expect, it } from "vitest";
import {
  buildSetupChecklist,
  countCompletedSteps,
} from "@/components/dashboard/setupChecklist";

describe("setupChecklist", () => {
  it("marks incomplete setup items clearly", () => {
    const steps = buildSetupChecklist({
      hasLicenseAccess: true,
      hasApiKey: false,
      hasAgentProfile: false,
      propertyCount: 0,
    });

    expect(countCompletedSteps(steps)).toBe(1);
    expect(steps.find((step) => step.id === "license")?.complete).toBe(true);
    expect(steps.find((step) => step.id === "api-key")?.complete).toBe(false);
    expect(steps.find((step) => step.id === "profile")?.complete).toBe(false);
    expect(steps.find((step) => step.id === "property")?.complete).toBe(false);
  });

  it("marks the workspace ready when every step is complete", () => {
    const steps = buildSetupChecklist({
      hasLicenseAccess: true,
      hasApiKey: true,
      hasAgentProfile: true,
      propertyCount: 3,
    });

    expect(countCompletedSteps(steps)).toBe(4);
    expect(steps.every((step) => step.complete)).toBe(true);
  });
});
