import { describe, it, expect, beforeEach, vi } from "vitest";
import { renderHook } from "@testing-library/react";
import { useKeyboardShortcuts } from "@/hooks/useKeyboardShortcuts";

const mockNavigate = vi.fn();
const mockUseLocation = vi.fn(() => ({ pathname: "/" }));

vi.mock("react-router-dom", async () => {
  const actual =
    await vi.importActual<typeof import("react-router-dom")>(
      "react-router-dom",
    );
  return {
    ...actual,
    useNavigate: () => mockNavigate,
    useLocation: () => mockUseLocation(),
  };
});

describe("useKeyboardShortcuts", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockUseLocation.mockReturnValue({ pathname: "/" });
    document.body.innerHTML = "";
  });

  it("should navigate to /property/new on Cmd+N", () => {
    renderHook(() => useKeyboardShortcuts());

    const event = new KeyboardEvent("keydown", {
      key: "n",
      metaKey: true,
      bubbles: true,
    });

    window.dispatchEvent(event);

    expect(mockNavigate).toHaveBeenCalledWith("/property/new");
  });

  it("should navigate to /property/new on Ctrl+N (Windows)", () => {
    renderHook(() => useKeyboardShortcuts());

    const event = new KeyboardEvent("keydown", {
      key: "n",
      ctrlKey: true,
      bubbles: true,
    });

    window.dispatchEvent(event);

    expect(mockNavigate).toHaveBeenCalledWith("/property/new");
  });

  it("should not trigger Cmd+N when typing in input", () => {
    renderHook(() => useKeyboardShortcuts());

    const input = document.createElement("input");
    document.body.appendChild(input);

    const event = new KeyboardEvent("keydown", {
      key: "n",
      metaKey: true,
      bubbles: true,
    });

    input.dispatchEvent(event);

    expect(mockNavigate).not.toHaveBeenCalled();
  });

  it("should not trigger Cmd+N when typing in textarea", () => {
    renderHook(() => useKeyboardShortcuts());

    const textarea = document.createElement("textarea");
    document.body.appendChild(textarea);

    const event = new KeyboardEvent("keydown", {
      key: "n",
      metaKey: true,
      bubbles: true,
    });

    textarea.dispatchEvent(event);

    expect(mockNavigate).not.toHaveBeenCalled();
  });

  it("should not trigger Cmd+N when typing in contenteditable", () => {
    renderHook(() => useKeyboardShortcuts());

    const div = document.createElement("div");
    div.contentEditable = "true";
    Object.defineProperty(div, "isContentEditable", {
      configurable: true,
      value: true,
    });
    document.body.appendChild(div);

    const event = new KeyboardEvent("keydown", {
      key: "n",
      metaKey: true,
      bubbles: true,
    });

    div.dispatchEvent(event);

    expect(mockNavigate).not.toHaveBeenCalled();
  });

  it("should not trigger without meta or ctrl key", () => {
    renderHook(() => useKeyboardShortcuts());

    const event = new KeyboardEvent("keydown", {
      key: "n",
      bubbles: true,
    });

    window.dispatchEvent(event);

    expect(mockNavigate).not.toHaveBeenCalled();
  });

  it("should not trigger Cmd+Shift+N", () => {
    renderHook(() => useKeyboardShortcuts());

    const event = new KeyboardEvent("keydown", {
      key: "n",
      metaKey: true,
      shiftKey: true,
      bubbles: true,
    });

    window.dispatchEvent(event);

    expect(mockNavigate).not.toHaveBeenCalled();
  });

  it("should click generate button on Cmd+Enter on generation page", () => {
    mockUseLocation.mockReturnValue({ pathname: "/property/123/listing" });

    renderHook(() => useKeyboardShortcuts());

    const button = document.createElement("button");
    button.className = "bg-blue-600";
    button.disabled = false;
    const clickSpy = vi.fn();
    button.addEventListener("click", clickSpy);
    document.body.appendChild(button);

    const event = new KeyboardEvent("keydown", {
      key: "Enter",
      metaKey: true,
      bubbles: true,
    });

    window.dispatchEvent(event);

    expect(clickSpy).toHaveBeenCalled();
  });

  it("should work on social generation page", () => {
    mockUseLocation.mockReturnValue({ pathname: "/property/123/social" });

    renderHook(() => useKeyboardShortcuts());

    const button = document.createElement("button");
    button.className = "bg-blue-600";
    const clickSpy = vi.fn();
    button.addEventListener("click", clickSpy);
    document.body.appendChild(button);

    const event = new KeyboardEvent("keydown", {
      key: "Enter",
      metaKey: true,
      bubbles: true,
    });

    window.dispatchEvent(event);

    expect(clickSpy).toHaveBeenCalled();
  });

  it("should work on email generation page", () => {
    mockUseLocation.mockReturnValue({ pathname: "/property/123/email" });

    renderHook(() => useKeyboardShortcuts());

    const button = document.createElement("button");
    button.className = "bg-blue-600";
    const clickSpy = vi.fn();
    button.addEventListener("click", clickSpy);
    document.body.appendChild(button);

    const event = new KeyboardEvent("keydown", {
      key: "Enter",
      metaKey: true,
      bubbles: true,
    });

    window.dispatchEvent(event);

    expect(clickSpy).toHaveBeenCalled();
  });

  it("should not click generate button on non-generation pages", () => {
    mockUseLocation.mockReturnValue({ pathname: "/dashboard" });

    renderHook(() => useKeyboardShortcuts());

    const button = document.createElement("button");
    button.className = "bg-blue-600";
    const clickSpy = vi.fn();
    button.addEventListener("click", clickSpy);
    document.body.appendChild(button);

    const event = new KeyboardEvent("keydown", {
      key: "Enter",
      metaKey: true,
      bubbles: true,
    });

    window.dispatchEvent(event);

    expect(clickSpy).not.toHaveBeenCalled();
  });

  it("should not click disabled buttons", () => {
    mockUseLocation.mockReturnValue({ pathname: "/property/123/listing" });

    renderHook(() => useKeyboardShortcuts());

    const button = document.createElement("button");
    button.className = "bg-blue-600";
    button.disabled = true;
    const clickSpy = vi.fn();
    button.addEventListener("click", clickSpy);
    document.body.appendChild(button);

    const event = new KeyboardEvent("keydown", {
      key: "Enter",
      metaKey: true,
      bubbles: true,
    });

    window.dispatchEvent(event);

    expect(clickSpy).not.toHaveBeenCalled();
  });
});
