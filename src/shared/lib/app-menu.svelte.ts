/**
 * Subscribe to native menu events emitted from app-menu.ts, and on Windows and
 * Linux also deliver the menu-only shortcuts through a window keydown listener.
 */
import { listen } from "@tauri-apps/api/event";
import type { AppMenuEvent } from "$lib/lib/app-menu";
import { SHORTCUTS } from "$lib/lib/shortcuts";
import { isMacOS } from "$lib/lib/platform";
import { isModKey } from "$lib/lib/keyboard";

export type AppMenuHandlers = Partial<Record<AppMenuEvent, () => void>>;

/**
 * Menu commands whose only in-app delivery path is the native menu's
 * accelerator. The macOS menu bar always shows and its accelerators fire, but
 * on Windows/Linux the menu is often hidden or its accelerators never reach the
 * webview — so these "promised" shortcuts (advertised in the help dialog) die.
 * On those platforms the keydown fallback in subscribeToAppMenu routes the same
 * chords into the exact same handlers.
 */
const MENU_ONLY_EVENTS: ReadonlyArray<{
  event: AppMenuEvent;
  keys: readonly string[];
}> = [
  { event: "menu://new-project", keys: SHORTCUTS.newProject.keys },
  { event: "menu://open-dashboard", keys: SHORTCUTS.openDashboard.keys },
  { event: "menu://export", keys: SHORTCUTS.export.keys },
  { event: "menu://present", keys: SHORTCUTS.present.keys },
  { event: "menu://settings", keys: SHORTCUTS.settings.keys },
  { event: "menu://add-slide", keys: SHORTCUTS.addSlide.keys },
  { event: "menu://duplicate-slide", keys: SHORTCUTS.duplicateSlide.keys },
];

/** Does a KeyboardEvent match a SHORTCUTS key list like ["mod","Shift","N"]? */
function matchesShortcut(e: KeyboardEvent, keys: readonly string[]): boolean {
  const wantsMod = keys.includes("mod");
  const wantsShift = keys.includes("Shift");
  if (wantsMod !== isModKey(e)) return false;
  if (wantsShift !== e.shiftKey) return false;
  if (e.altKey) return false;
  const key = keys.find((k) => k !== "mod" && k !== "Shift");
  if (!key) return false;
  // Letters match case-insensitively; symbols (",") match exactly.
  return e.key.toLowerCase() === key.toLowerCase();
}

export function subscribeToAppMenu(handlers: () => AppMenuHandlers) {
  $effect(() => {
    const unsubs: Array<() => void> = [];
    // Snapshot the handler map for this subscription round — listeners read
    // through it so replacements only take effect on resubscribe.
    const current = handlers();
    const events = Object.keys(current) as AppMenuEvent[];

    // A chord can be delivered by the native menu AND (on Windows/Linux) by
    // the keydown fallback within milliseconds; keep only the first firing so
    // a working native menu never double-fires alongside the webview.
    const lastFired = new Map<AppMenuEvent, number>();
    const dispatch = (ev: AppMenuEvent) => {
      const now = Date.now();
      if (now - (lastFired.get(ev) ?? 0) < 200) return;
      lastFired.set(ev, now);
      current[ev]?.();
    };

    let cancelled = false;

    (async () => {
      for (const ev of events) {
        try {
          const un = await listen(ev, () => {
            dispatch(ev);
          });
          if (cancelled) un();
          else unsubs.push(un);
        } catch {
          /* not in tauri */
        }
      }
    })();

    if (!isMacOS()) {
      const onKeyDown = (e: KeyboardEvent) => {
        for (const { event, keys } of MENU_ONLY_EVENTS) {
          if (matchesShortcut(e, keys) && current[event]) {
            e.preventDefault();
            dispatch(event);
            return;
          }
        }
      };
      window.addEventListener("keydown", onKeyDown);
      unsubs.push(() => window.removeEventListener("keydown", onKeyDown));
    }

    return () => {
      cancelled = true;
      unsubs.forEach((u) => u());
    };
  });
}
