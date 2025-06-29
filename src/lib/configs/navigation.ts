export const NAV_LINKS = {
  home: {
    id: "home",
    label: "HOME",
    href: "/",
    icon: "home",
		disabled: false,
    visible: true
  },
  settings: {
    id: "settings",
    label: "USTAWIENIA",
    href: "/settings",
    icon: "settings",
		disabled: false,
    visible: true
  },
  notepad: {
    id: "notes",
    label: "NOTATKI",
    href: "/notes",
    icon: "notepad",
		disabled: false,
    visible: true
  }
} as const;

export const get_nav_links_as_array = () => Object.values(NAV_LINKS);
