export const LEFT_BAR_OPTIONS = {
    create_note: {
        id: "create_note",
        label: "Create new note",
        icon: "create_note",
        disabled: false,
        visible: true
    },
    delete_note: {
        id: "delete_note",
        label: "Delete note",
        icon: "delete_note",
        disabled: false,
        visible: true
    },
    change_notes_directory: {
        id: "change_notes_directory",
        label: "Change notes directory",
        icon: "change_directory",
        disabled: false,
        visible: true
    },
} as const;

export const get_left_bar_options_as_array = () => Object.values(LEFT_BAR_OPTIONS);
