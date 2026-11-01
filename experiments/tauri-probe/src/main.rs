use serde::{Deserialize, Serialize};
use tauri::State;
use tauri_probe::JobGate;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct LayoutJob {
    job_id: String,
    profile_id: String,
    generation: u64,
    input_sha256: String,
    settings_sha256: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct JobAcknowledgement {
    job_id: String,
    generation: u64,
    accepted: bool,
    reason: Option<String>,
}

#[tauri::command]
fn submit_layout_job(job: LayoutJob, gate: State<'_, JobGate>) -> JobAcknowledgement {
    let accepted = gate.accept(job.generation);
    JobAcknowledgement {
        job_id: job.job_id,
        generation: job.generation,
        accepted,
        reason: (!accepted).then(|| "stale generation".to_string()),
    }
}

#[tauri::command]
fn cancel_layout_job(generation: u64, gate: State<'_, JobGate>) -> bool {
    gate.cancel(generation)
}

fn main() {
    let result = tauri::Builder::default()
        .manage(JobGate::default())
        .invoke_handler(tauri::generate_handler![
            submit_layout_job,
            cancel_layout_job
        ])
        .run(tauri::generate_context!());

    if let Err(error) = result {
        eprintln!("Tauri probe failed: {error}");
        std::process::exit(1);
    }
}
