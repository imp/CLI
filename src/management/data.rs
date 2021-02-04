pub mod telemetry;
pub mod launches;

pub struct RenderFrame {
    pub view: usize,
    pub launch: Option<launches::Launch>,
    pub telemetry: Option<telemetry::Snapshot>,
    pub error: Option<(String, bool)>,
}