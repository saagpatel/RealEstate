import "@testing-library/jest-dom/vitest";

// Mock Tauri APIs for testing
vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
  convertFileSrc: vi.fn((path: string) => path),
  Channel: vi.fn().mockImplementation(() => ({
    onmessage: null,
    id: 0,
    __TAURI_CHANNEL_MARKER__: true,
    toJSON() {
      return `__CHANNEL__:${this.id}`;
    },
  })),
}));

vi.mock("@tauri-apps/plugin-clipboard-manager", () => ({
  writeText: vi.fn(),
}));

vi.mock("@tauri-apps/plugin-dialog", () => ({
  save: vi.fn(),
  open: vi.fn(),
}));
