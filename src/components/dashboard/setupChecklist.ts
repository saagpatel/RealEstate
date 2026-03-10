export interface SetupChecklistInput {
  hasLicenseAccess: boolean;
  hasApiKey: boolean;
  hasAgentProfile: boolean;
  propertyCount: number;
}

export interface SetupChecklistStep {
  id: string;
  label: string;
  description: string;
  complete: boolean;
}

export function buildSetupChecklist({
  hasLicenseAccess,
  hasApiKey,
  hasAgentProfile,
  propertyCount,
}: SetupChecklistInput): SetupChecklistStep[] {
  return [
    {
      id: "license",
      label: "License activated",
      description: "Desktop access is unlocked and ready to use.",
      complete: hasLicenseAccess,
    },
    {
      id: "api-key",
      label: "Anthropic API key connected",
      description: "Required before the app can generate listing content.",
      complete: hasApiKey,
    },
    {
      id: "profile",
      label: "Agent profile completed",
      description:
        "Helps exports and generated copy reflect your business identity.",
      complete: hasAgentProfile,
    },
    {
      id: "property",
      label: "At least one property loaded",
      description: "Create a flagship property or import a batch from CSV.",
      complete: propertyCount > 0,
    },
  ];
}

export function countCompletedSteps(steps: SetupChecklistStep[]) {
  return steps.filter((step) => step.complete).length;
}
