use leptos::prelude::*;

/// Represents the item being dragged
#[derive(Clone, Debug, Default)]
pub struct DraggedItem {
    /// The path of the file/folder being dragged
    pub path: String,
    /// The name of the item for display
    pub name: String,
    /// Whether it's a folder
    pub is_folder: bool,
}

/// Global drag state for file operations
#[derive(Clone, Copy)]
pub struct DragDropState {
    /// The currently dragged item (if any)
    pub dragged_item: RwSignal<Option<DraggedItem>>,
    /// The current drop target path (for highlighting)
    pub drop_target: RwSignal<Option<String>>,
}

impl DragDropState {
    pub fn new() -> Self {
        Self {
            dragged_item: RwSignal::new(None),
            drop_target: RwSignal::new(None),
        }
    }

    /// Start dragging an item
    pub fn start_drag(&self, path: String, name: String, is_folder: bool) {
        self.dragged_item.set(Some(DraggedItem {
            path,
            name,
            is_folder,
        }));
    }

    /// End the drag operation
    pub fn end_drag(&self) {
        self.dragged_item.set(None);
        self.drop_target.set(None);
    }

    /// Set the current drop target for visual feedback
    pub fn set_drop_target(&self, path: Option<String>) {
        self.drop_target.set(path);
    }

    /// Check if we're currently dragging
    pub fn is_dragging(&self) -> bool {
        self.dragged_item.get().is_some()
    }

    /// Check if a path is a valid drop target for the current dragged item
    pub fn is_valid_drop_target(&self, target_path: &str) -> bool {
        if let Some(dragged) = self.dragged_item.get() {
            // Can't drop on self
            if dragged.path == target_path {
                return false;
            }
            // Can't drop a folder into itself or its descendants
            if dragged.is_folder && target_path.starts_with(&format!("{}/", dragged.path)) {
                return false;
            }
            true
        } else {
            false
        }
    }
}

impl Default for DragDropState {
    fn default() -> Self {
        Self::new()
    }
}

/// Provider component for drag-drop context
#[component]
pub fn DragDropProvider(children: Children) -> impl IntoView {
    let state = DragDropState::new();
    provide_context(state);
    children()
}

/// Hook to use the drag-drop state
pub fn use_drag_drop() -> DragDropState {
    expect_context::<DragDropState>()
}
