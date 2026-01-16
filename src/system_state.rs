use leptos::prelude::*;

/// Represents clipboard content types
#[derive(Clone, Debug, Default)]
pub enum ClipboardContent {
    #[default]
    Empty,
    Text(String),
    Files(Vec<ClipboardFile>),
}

/// Represents a file in the clipboard
#[derive(Clone, Debug)]
pub struct ClipboardFile {
    pub name: String,
    pub path: String,
    pub is_folder: bool,
    pub is_cut: bool, // true for cut, false for copy
}

/// Global clipboard state
#[derive(Clone, Debug, Default)]
pub struct Clipboard {
    pub content: ClipboardContent,
}

impl Clipboard {
    pub fn new() -> Self {
        Self {
            content: ClipboardContent::Empty,
        }
    }

    pub fn copy_text(&mut self, text: String) {
        self.content = ClipboardContent::Text(text);
    }

    pub fn copy_files(&mut self, files: Vec<ClipboardFile>) {
        self.content = ClipboardContent::Files(files);
    }

    pub fn cut_files(&mut self, files: Vec<ClipboardFile>) {
        let cut_files: Vec<ClipboardFile> = files
            .into_iter()
            .map(|mut f| {
                f.is_cut = true;
                f
            })
            .collect();
        self.content = ClipboardContent::Files(cut_files);
    }

    pub fn get_text(&self) -> Option<String> {
        match &self.content {
            ClipboardContent::Text(s) => Some(s.clone()),
            _ => None,
        }
    }

    pub fn get_files(&self) -> Option<Vec<ClipboardFile>> {
        match &self.content {
            ClipboardContent::Files(f) => Some(f.clone()),
            _ => None,
        }
    }

    pub fn clear(&mut self) {
        self.content = ClipboardContent::Empty;
    }

    pub fn is_empty(&self) -> bool {
        matches!(self.content, ClipboardContent::Empty)
    }

    pub fn has_text(&self) -> bool {
        matches!(self.content, ClipboardContent::Text(_))
    }

    pub fn has_files(&self) -> bool {
        matches!(self.content, ClipboardContent::Files(_))
    }
}

/// Provide clipboard context to the app
pub fn provide_clipboard_context() {
    let (clipboard, set_clipboard) = signal(Clipboard::new());
    provide_context(clipboard);
    provide_context(set_clipboard);
}

/// Get the clipboard read signal
pub fn use_clipboard() -> ReadSignal<Clipboard> {
    expect_context::<ReadSignal<Clipboard>>()
}

/// Get the clipboard write signal
pub fn use_clipboard_setter() -> WriteSignal<Clipboard> {
    expect_context::<WriteSignal<Clipboard>>()
}
