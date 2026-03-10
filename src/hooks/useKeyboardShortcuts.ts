import { useEffect } from "react";
import { useNavigate, useLocation } from "react-router-dom";

export function useKeyboardShortcuts() {
  const navigate = useNavigate();
  const location = useLocation();

  useEffect(() => {
    function handleKeyDown(e: KeyboardEvent) {
      const isMeta = e.metaKey || e.ctrlKey;
      if (!isMeta) return;

      // Cmd+N → New Property (anywhere)
      if (e.key === "n" && !e.shiftKey) {
        // Don't hijack if user is typing in an input
        const target = e.target as HTMLElement;
        if (
          target.tagName === "INPUT" ||
          target.tagName === "TEXTAREA" ||
          target.isContentEditable
        ) {
          return;
        }
        e.preventDefault();
        navigate("/property/new");
        return;
      }

      // Cmd+Enter → Generate (on generation pages)
      if (e.key === "Enter") {
        const isOnGenerationPage =
          location.pathname.endsWith("/listing") ||
          location.pathname.endsWith("/social") ||
          location.pathname.endsWith("/email");

        if (isOnGenerationPage) {
          // Find and click the generate button
          const btn = document.querySelector<HTMLButtonElement>(
            'button:not(:disabled)[class*="bg-blue-600"]',
          );
          if (btn) {
            e.preventDefault();
            btn.click();
          }
        }
      }
    }

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [navigate, location.pathname]);
}
