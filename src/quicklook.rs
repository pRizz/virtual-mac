use leptos::prelude::*;

/// Information about a file to preview in Quick Look
#[derive(Clone, Debug, PartialEq)]
pub struct QuickLookFile {
    pub name: String,
    pub icon: String,
    pub file_type: FileType,
}

#[derive(Clone, Debug, PartialEq)]
pub enum FileType {
    Folder,
    Image,
    Document,
    Application,
    Archive,
    Other,
}

impl FileType {
    pub fn from_name(name: &str, is_folder: bool) -> Self {
        if is_folder {
            return FileType::Folder;
        }

        let lower = name.to_lowercase();
        if lower.ends_with(".png")
            || lower.ends_with(".jpg")
            || lower.ends_with(".jpeg")
            || lower.ends_with(".gif")
            || lower.ends_with(".webp")
        {
            FileType::Image
        } else if lower.ends_with(".pdf")
            || lower.ends_with(".doc")
            || lower.ends_with(".docx")
            || lower.ends_with(".txt")
            || lower.ends_with(".xlsx")
            || lower.ends_with(".xls")
        {
            FileType::Document
        } else if lower.ends_with(".app") || lower.ends_with(".dmg") {
            FileType::Application
        } else if lower.ends_with(".zip") || lower.ends_with(".tar") || lower.ends_with(".gz") {
            FileType::Archive
        } else {
            FileType::Other
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            FileType::Folder => "Folder",
            FileType::Image => "Image",
            FileType::Document => "Document",
            FileType::Application => "Application",
            FileType::Archive => "Archive",
            FileType::Other => "File",
        }
    }
}

/// Context for managing Quick Look state
#[derive(Clone, Copy)]
pub struct QuickLookContext {
    pub file: ReadSignal<Option<QuickLookFile>>,
    pub set_file: WriteSignal<Option<QuickLookFile>>,
    pub visible: ReadSignal<bool>,
    pub set_visible: WriteSignal<bool>,
}

impl QuickLookContext {
    /// Show Quick Look for a file
    pub fn show(&self, file: QuickLookFile) {
        self.set_file.set(Some(file));
        self.set_visible.set(true);
    }

    /// Hide Quick Look
    pub fn hide(&self) {
        self.set_visible.set(false);
    }

    /// Toggle Quick Look visibility
    pub fn toggle(&self, file: Option<QuickLookFile>) {
        if self.visible.get() {
            self.hide();
        } else if let Some(f) = file {
            self.show(f);
        }
    }
}

/// Provider component for Quick Look context
#[component]
pub fn QuickLookProvider(children: Children) -> impl IntoView {
    let (file, set_file) = signal(None::<QuickLookFile>);
    let (visible, set_visible) = signal(false);

    let context = QuickLookContext {
        file,
        set_file,
        visible,
        set_visible,
    };

    provide_context(context);

    children()
}

/// Get the Quick Look context
pub fn use_quicklook() -> QuickLookContext {
    expect_context::<QuickLookContext>()
}

/// Quick Look modal overlay component
#[component]
pub fn QuickLookModal() -> impl IntoView {
    let ctx = use_quicklook();

    let on_backdrop_click = move |_| {
        ctx.hide();
    };

    let on_close_click = move |_| {
        ctx.hide();
    };

    // Prevent click propagation on modal content
    let on_content_click = move |e: web_sys::MouseEvent| {
        e.stop_propagation();
    };

    view! {
        <Show when=move || ctx.visible.get()>
            <div class="quicklook-overlay" on:click=on_backdrop_click>
                <div class="quicklook-modal" on:click=on_content_click>
                    <Show when=move || ctx.file.get().is_some()>
                        {move || {
                            let file = ctx.file.get().unwrap();
                            view! {
                                <div class="quicklook-header">
                                    <div class="quicklook-title">{file.name.clone()}</div>
                                    <button class="quicklook-close" on:click=on_close_click>
                                        "×"
                                    </button>
                                </div>
                                <div class="quicklook-content">
                                    <div class="quicklook-preview">
                                        <div class="quicklook-icon">{file.icon.clone()}</div>
                                    </div>
                                    <div class="quicklook-info">
                                        <div class="quicklook-filename">{file.name.clone()}</div>
                                        <div class="quicklook-filetype">{file.file_type.description()}</div>
                                    </div>
                                </div>
                                <div class="quicklook-footer">
                                    <span class="quicklook-hint">"Press Space to close"</span>
                                </div>
                            }
                        }}
                    </Show>
                </div>
            </div>
        </Show>
    }
}
