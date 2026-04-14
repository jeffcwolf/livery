//! Stub implementation of the vitrine crate for CI environments.
//!
//! Provides the full public API surface with no-op implementations.
//! On developer machines, `.cargo/config.toml` overrides this with
//! the real crate via `paths = [...]`.

use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// ViewerId
// ---------------------------------------------------------------------------

/// Unique identifier for a viewer window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ViewerId(pub u64);

// ---------------------------------------------------------------------------
// ViewerConfig
// ---------------------------------------------------------------------------

/// Configuration for creating a new viewer window.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewerConfig {
    /// Window title.
    pub title: String,
    /// Initial width in logical pixels.
    pub width: u32,
    /// Initial height in logical pixels.
    pub height: u32,
    /// Initial X position.
    pub x: Option<i32>,
    /// Initial Y position.
    pub y: Option<i32>,
    /// Whether to open dev tools.
    pub dev_tools: bool,
    /// DPI for PDF rasterization.
    pub pdf_rasterize_dpi: u32,
}

impl Default for ViewerConfig {
    fn default() -> Self {
        Self {
            title: String::from("Vitrine Viewer"),
            width: 1024,
            height: 768,
            x: None,
            y: None,
            dev_tools: false,
            pdf_rasterize_dpi: 300,
        }
    }
}

// ---------------------------------------------------------------------------
// ServerConfig
// ---------------------------------------------------------------------------

/// Configuration for the local HTTP server.
#[derive(Debug, Clone)]
pub struct ServerConfig {
    /// Directory containing image files.
    pub image_directory: PathBuf,
    /// Directory for the IIIF tile disk cache.
    pub cache_dir: Option<PathBuf>,
}

// ---------------------------------------------------------------------------
// ImageSource
// ---------------------------------------------------------------------------

/// How to source an image for display in the viewer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ImageSource {
    /// A remote IIIF Image API info.json URL.
    Iiif {
        /// The full URL to the IIIF info.json descriptor.
        info_json_url: String,
    },
    /// A local file served through the built-in IIIF tile server.
    LocalTiled {
        /// Path to the image file on disk.
        file_path: PathBuf,
    },
    /// A local file served directly without tiling.
    LocalDirect {
        /// Path to the image file on disk.
        file_path: PathBuf,
    },
}

// ---------------------------------------------------------------------------
// PdfMode
// ---------------------------------------------------------------------------

/// PDF rendering mode.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum PdfMode {
    /// PDF.js interactive rendering.
    Interactive,
    /// Server-side rasterization through OpenSeadragon.
    Rasterized,
}

// ---------------------------------------------------------------------------
// Region
// ---------------------------------------------------------------------------

/// A rectangle in normalized coordinates (0.0–1.0, origin top-left).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Region {
    /// Horizontal offset from the left edge.
    pub x: f64,
    /// Vertical offset from the top edge.
    pub y: f64,
    /// Width as a fraction of the image width.
    pub w: f64,
    /// Height as a fraction of the image height.
    pub h: f64,
}

// ---------------------------------------------------------------------------
// OverlayGeometry
// ---------------------------------------------------------------------------

/// Geometric shape for an overlay, in normalized coordinates.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum OverlayGeometry {
    /// An axis-aligned rectangle.
    Rect {
        /// The rectangular bounds.
        region: Region,
    },
    /// A circle defined by center and radius.
    Circle {
        /// Horizontal center.
        cx: f64,
        /// Vertical center.
        cy: f64,
        /// Radius as a fraction of the image width.
        r: f64,
    },
    /// A closed polygon defined by vertex coordinates.
    Polygon {
        /// Ordered vertices as (x, y) pairs.
        points: Vec<(f64, f64)>,
    },
}

// ---------------------------------------------------------------------------
// OverlayStyle
// ---------------------------------------------------------------------------

/// Visual style for an overlay.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OverlayStyle {
    /// CSS color for the overlay border stroke.
    pub stroke_color: String,
    /// Optional CSS color for the overlay fill.
    pub fill_color: Option<String>,
    /// Width of the border stroke in logical pixels.
    pub stroke_width: f64,
}

// ---------------------------------------------------------------------------
// Overlay
// ---------------------------------------------------------------------------

/// A programmatic annotation overlay placed on an image.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Overlay {
    /// Unique identifier for this overlay.
    pub id: String,
    /// Optional group name for batch operations.
    pub group: Option<String>,
    /// The geometric shape and position.
    pub geometry: OverlayGeometry,
    /// Visual styling.
    pub style: OverlayStyle,
    /// Optional text label.
    pub label: Option<String>,
    /// Optional arbitrary JSON payload.
    pub data: Option<serde_json::Value>,
    /// Whether clicking emits an OverlayClicked event.
    pub interactive: bool,
}

// ---------------------------------------------------------------------------
// ImageSetEntry
// ---------------------------------------------------------------------------

/// A single image within a batch image set.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageSetEntry {
    /// Application-defined identifier.
    pub id: String,
    /// How to load the image.
    pub source: ImageSource,
    /// Display label for the thumbnail strip.
    pub label: Option<String>,
    /// Explicit thumbnail URL.
    pub thumbnail_url: Option<String>,
}

// ---------------------------------------------------------------------------
// MetadataSection / MetadataEntry
// ---------------------------------------------------------------------------

/// A named group of metadata key-value pairs.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetadataSection {
    /// Section heading.
    pub heading: String,
    /// Ordered list of key-value pairs.
    pub entries: Vec<MetadataEntry>,
}

/// A single key-value pair in a metadata section.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetadataEntry {
    /// Display label.
    pub key: String,
    /// Display value.
    pub value: String,
}

// ---------------------------------------------------------------------------
// ColumnType
// ---------------------------------------------------------------------------

/// How a table column's values should be interpreted for sorting,
/// filtering, and display.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ColumnType {
    /// Free-form text. Uses text filter and lexicographic sort.
    Text,
    /// Numeric values. Uses number filter and numeric sort.
    Number,
    /// Date strings. Uses date filter.
    Date,
    /// Boolean values.
    Boolean,
    /// Categorical tags. Uses text filter.
    Tag,
}

// ---------------------------------------------------------------------------
// TableColumn
// ---------------------------------------------------------------------------

/// Definition of a single column in a data table.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TableColumn {
    /// Unique identifier for this column.
    pub id: String,
    /// Human-readable header text.
    pub label: String,
    /// Data type for sorting, filtering, and display.
    pub column_type: ColumnType,
    /// Initial column width in pixels.
    pub width: Option<u32>,
    /// Whether the column can be sorted.
    pub sortable: bool,
    /// Whether the column shows a filter input.
    pub filterable: bool,
    /// Whether cells in this column can be edited inline.
    pub editable: bool,
    /// Whether the column is visible.
    pub visible: bool,
}

impl Default for TableColumn {
    fn default() -> Self {
        Self {
            id: String::new(),
            label: String::new(),
            column_type: ColumnType::Text,
            width: None,
            sortable: true,
            filterable: true,
            editable: false,
            visible: true,
        }
    }
}

// ---------------------------------------------------------------------------
// TableRow
// ---------------------------------------------------------------------------

/// A single row of table data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TableRow {
    /// Unique identifier for this row.
    pub id: String,
    /// Cell values keyed by column id.
    pub cells: HashMap<String, String>,
}

// ---------------------------------------------------------------------------
// TableConfig
// ---------------------------------------------------------------------------

/// Configuration for table display behavior.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TableConfig {
    /// Row height in pixels.
    pub row_height: Option<u32>,
    /// Whether to show a row-number column.
    pub show_row_numbers: bool,
    /// Whether multiple rows can be selected simultaneously.
    pub enable_multi_select: bool,
    /// Whether inline cell editing is enabled globally.
    pub enable_inline_edit: bool,
    /// Number of leading columns to pin on the left.
    pub frozen_columns: Option<u32>,
}

// ---------------------------------------------------------------------------
// TableColumnState
// ---------------------------------------------------------------------------

/// Current state of a table column reported by the viewer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TableColumnState {
    /// The column identifier.
    pub column_id: String,
    /// Whether the column is currently visible.
    pub visible: bool,
    /// Current column width in pixels.
    pub width: u32,
}

// ---------------------------------------------------------------------------
// ViewerCommand
// ---------------------------------------------------------------------------

/// Commands sent from the Iced app to a viewer window.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ViewerCommand {
    /// Load and display an image.
    LoadImage {
        /// How to source the image.
        source: ImageSource,
    },
    /// Load and display a PDF document.
    LoadPdf {
        /// Path to the PDF file.
        path: PathBuf,
        /// Rendering mode.
        mode: PdfMode,
        /// Optional initial page (1-based).
        initial_page: Option<u32>,
    },
    /// Navigate to a specific PDF page.
    PdfGotoPage {
        /// Page number (0-indexed).
        page: u32,
    },
    /// Search for text within a PDF.
    PdfSearch {
        /// The search query string.
        query: String,
        /// Whether to highlight all matches.
        highlight_all: bool,
    },
    /// Add a single overlay.
    AddOverlay(Overlay),
    /// Add multiple overlays.
    AddOverlays {
        /// The overlays to add.
        overlays: Vec<Overlay>,
    },
    /// Remove a single overlay by id.
    RemoveOverlay {
        /// The id of the overlay to remove.
        id: String,
    },
    /// Remove all overlays, optionally filtered by group.
    ClearOverlays {
        /// If Some, only remove overlays in this group.
        group: Option<String>,
    },
    /// Pan and zoom to a specific region.
    NavigateTo {
        /// The target region.
        region: Region,
        /// Optional zoom level override.
        zoom: Option<f64>,
    },
    /// Briefly highlight a region.
    HighlightRegion {
        /// The region to highlight.
        region: Region,
        /// CSS color for the highlight.
        color: String,
        /// Duration in milliseconds.
        duration_ms: Option<u64>,
    },
    /// Enable or disable drawing mode.
    SetDrawingMode {
        /// Whether drawing mode is active.
        enabled: bool,
        /// Style for user-drawn overlays.
        style: OverlayStyle,
        /// Group name for drawn overlays.
        group: Option<String>,
    },
    /// Load a batch of images with thumbnail strip.
    LoadImageSet {
        /// The ordered list of images.
        images: Vec<ImageSetEntry>,
        /// Which image to display initially.
        initial_id: Option<String>,
    },
    /// Load images from a IIIF Presentation manifest.
    LoadIiifManifest {
        /// Full URL to the IIIF manifest.
        manifest_url: String,
        /// Optional initial canvas ID.
        initial_canvas_id: Option<String>,
    },
    /// Switch the active image in the set.
    SelectImage {
        /// The id of the image to display.
        id: String,
    },
    /// Set metadata key-value pairs.
    SetMetadata {
        /// Ordered list of metadata sections.
        sections: Vec<MetadataSection>,
    },
    /// Show or hide the metadata panel.
    SetMetadataVisible {
        /// Whether the metadata panel should be visible.
        visible: bool,
    },
    /// Show or hide the controls panel.
    SetControlsVisible {
        /// Whether the controls panel should be visible.
        visible: bool,
    },
    /// Set preprocessing slider values programmatically.
    SetPreprocessingValues {
        /// Contrast multiplier.
        contrast: f64,
        /// Brightness offset.
        brightness: f64,
        /// Rotation in degrees.
        rotation: f64,
    },
    /// Notify the viewer that extraction completed.
    ExtractionComplete {
        /// Whether extraction succeeded.
        success: bool,
        /// Result message.
        message: String,
    },
    /// Apply CSS custom properties to the viewer's document root.
    SetTheme {
        /// CSS custom property key-value pairs to apply.
        tokens: HashMap<String, String>,
    },
    /// Load a data table into the viewer.
    LoadTable {
        /// Column definitions.
        columns: Vec<TableColumn>,
        /// Row data to display.
        rows: Vec<TableRow>,
        /// Table display configuration.
        config: TableConfig,
    },
    /// Replace all row data in the current table.
    UpdateTableData {
        /// New row data.
        rows: Vec<TableRow>,
    },
    /// Programmatically select rows by their IDs.
    SetTableSelection {
        /// IDs of the rows to select.
        row_ids: Vec<String>,
    },
    /// Show or hide a table column by its ID.
    SetColumnVisibility {
        /// The column id to show or hide.
        column_id: String,
        /// Whether the column should be visible.
        visible: bool,
    },
    /// Set the quick filter text for the current table.
    SetQuickFilter {
        /// The search text to filter by.
        text: String,
    },
    /// Close the viewer window.
    Close,
}

// ---------------------------------------------------------------------------
// ViewerEvent
// ---------------------------------------------------------------------------

/// Events sent from a viewer window back to the Iced app.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ViewerEvent {
    /// Viewer is ready to receive commands.
    Ready,
    /// Image loaded with known dimensions.
    ImageLoaded {
        /// Image width in pixels.
        width: u32,
        /// Image height in pixels.
        height: u32,
    },
    /// Interactive overlay clicked.
    OverlayClicked {
        /// The overlay id.
        id: String,
        /// Optional JSON payload.
        data: Option<serde_json::Value>,
    },
    /// User selected a region.
    RegionSelected {
        /// The selected region.
        region: Region,
    },
    /// Viewport changed.
    ViewportChanged {
        /// Visible bounds.
        bounds: Region,
        /// Current zoom level.
        zoom: f64,
    },
    /// User drew a new overlay.
    OverlayCreated {
        /// Auto-generated id.
        id: String,
        /// The drawn geometry.
        geometry: OverlayGeometry,
    },
    /// Overlay resized or moved.
    OverlayResized {
        /// The overlay id.
        id: String,
        /// New geometry.
        geometry: OverlayGeometry,
    },
    /// Overlay deleted by user.
    OverlayDeleted {
        /// The overlay id.
        id: String,
    },
    /// PDF loaded.
    PdfLoaded {
        /// Total pages.
        page_count: u32,
        /// Has extractable text.
        has_text: bool,
    },
    /// PDF page changed.
    PdfPageChanged {
        /// Current page (0-indexed).
        page: u32,
        /// Total pages.
        total_pages: u32,
    },
    /// Text selected in PDF.
    PdfTextSelected {
        /// Selected text.
        text: String,
        /// Page number (0-indexed).
        page: u32,
    },
    /// IIIF manifest loaded.
    ManifestLoaded {
        /// Manifest label.
        label: String,
        /// Number of canvases.
        canvas_count: u32,
        /// Manifest URL.
        manifest_url: String,
    },
    /// Image set loaded.
    ImageSetLoaded {
        /// Total images.
        count: u32,
        /// Active image id.
        active_id: String,
    },
    /// Active image changed in set.
    ImageSelected {
        /// The now-active image id.
        id: String,
        /// 0-based index.
        index: u32,
    },
    /// Metadata panel toggled.
    MetadataPanelToggled {
        /// Whether visible.
        visible: bool,
    },
    /// Preprocessing sliders applied.
    PreprocessingApplied {
        /// Contrast multiplier.
        contrast: f64,
        /// Brightness offset.
        brightness: f64,
        /// Rotation in degrees.
        rotation: f64,
    },
    /// Preprocessing sliders reset.
    PreprocessingReset,
    /// Crop region confirmed.
    CropRequested {
        /// The crop region.
        region: Region,
    },
    /// Revert to original requested.
    RevertRequested,
    /// Display mode changed.
    DisplayModeChanged {
        /// The selected mode.
        mode: String,
    },
    /// Controls panel toggled.
    ControlsPanelToggled {
        /// Whether visible.
        visible: bool,
    },
    /// Auto-segment button clicked.
    AutoSegmentRequested,
    /// Extract features button clicked.
    ExtractFeaturesRequested,
    /// Clear all regions clicked.
    AllRegionsCleared,
    /// Table rendered and ready for interaction.
    TableReady {
        /// Number of rows loaded.
        row_count: u32,
        /// Number of columns.
        column_count: u32,
    },
    /// Single row clicked in the table.
    TableRowClicked {
        /// The id of the clicked row.
        row_id: String,
    },
    /// Row double-clicked in the table.
    TableRowDoubleClicked {
        /// The id of the double-clicked row.
        row_id: String,
    },
    /// Cell value edited inline in the table.
    TableCellEdited {
        /// The row id.
        row_id: String,
        /// The column id.
        column_id: String,
        /// Value before the edit.
        old_value: String,
        /// Value after the edit.
        new_value: String,
    },
    /// Selected rows changed in the table.
    TableSelectionChanged {
        /// IDs of all currently selected rows.
        row_ids: Vec<String>,
    },
    /// Column sort order changed in the table.
    TableSortChanged {
        /// The column id.
        column_id: String,
        /// Whether ascending.
        ascending: bool,
    },
    /// Column filter value changed in the table.
    TableFilterChanged {
        /// The column id.
        column_id: String,
        /// The filter value.
        value: String,
    },
    /// Column layout changed in the table.
    TableColumnsChanged {
        /// Current state of all columns.
        columns: Vec<TableColumnState>,
    },
    /// Non-recoverable error.
    Error {
        /// Error description.
        message: String,
    },
}

// ---------------------------------------------------------------------------
// VitrineError
// ---------------------------------------------------------------------------

/// Errors from the vitrine facade.
#[derive(Debug, thiserror::Error)]
pub enum VitrineError {
    /// The specified viewer id does not match an open window.
    #[error("unknown viewer: {0:?}")]
    UnknownViewer(ViewerId),

    /// An internal error occurred.
    #[error("internal error: {0}")]
    Internal(String),
}

// ---------------------------------------------------------------------------
// ViewerManager (stub — all methods are no-ops or return errors)
// ---------------------------------------------------------------------------

/// Stub viewer manager. All methods return errors or empty results.
pub struct ViewerManager {
    _private: (),
}

impl ViewerManager {
    /// Create a new viewer manager (stub — always succeeds but does nothing).
    pub fn new(_config: ServerConfig) -> Result<Self, VitrineError> {
        Ok(Self { _private: () })
    }

    /// Open a viewer window (stub — returns a dummy id).
    pub fn open(&mut self, _config: ViewerConfig) -> Result<ViewerId, VitrineError> {
        Ok(ViewerId(0))
    }

    /// Open an embedded viewer as a child of a parent window (stub — returns a dummy id).
    ///
    /// The parent must implement `HasWindowHandle`. The viewer renders as a
    /// child webview within the parent window at the position specified by
    /// `config.x`, `config.y`, `config.width`, `config.height`.
    pub fn open_embedded<W: raw_window_handle::HasWindowHandle>(
        &mut self,
        _parent: &W,
        _config: ViewerConfig,
    ) -> Result<ViewerId, VitrineError> {
        Ok(ViewerId(0))
    }

    /// Reposition and resize an embedded viewer (stub — no-op).
    pub fn set_embedded_frame(
        &mut self,
        _id: ViewerId,
        _x: i32,
        _y: i32,
        _width: u32,
        _height: u32,
    ) -> Result<(), VitrineError> {
        Ok(())
    }

    /// Show or hide an embedded viewer (stub — no-op).
    pub fn set_embedded_hidden(
        &mut self,
        _id: ViewerId,
        _hidden: bool,
    ) -> Result<(), VitrineError> {
        Ok(())
    }

    /// Reposition a viewer window on screen (stub — no-op).
    pub fn set_window_position(
        &mut self,
        _id: ViewerId,
        _x: i32,
        _y: i32,
    ) -> Result<(), VitrineError> {
        Ok(())
    }

    /// Resize a viewer window's content area (stub — no-op).
    pub fn set_window_size(
        &mut self,
        _id: ViewerId,
        _width: u32,
        _height: u32,
    ) -> Result<(), VitrineError> {
        Ok(())
    }

    /// Send a command to a viewer (stub — no-op).
    pub fn send(&self, _id: ViewerId, _command: ViewerCommand) -> Result<(), VitrineError> {
        Ok(())
    }

    /// Poll events from all viewers (stub — always empty).
    pub fn poll_events(&self) -> Vec<(ViewerId, ViewerEvent)> {
        Vec::new()
    }

    /// Close a viewer window (stub — no-op).
    pub fn close(&mut self, _id: ViewerId) -> Result<(), VitrineError> {
        Ok(())
    }

    /// Shut down the viewer subsystem (stub — no-op).
    pub fn shutdown(self) {}

    /// The port the local HTTP server is listening on (stub — returns 0).
    pub fn server_port(&self) -> u16 {
        0
    }
}
