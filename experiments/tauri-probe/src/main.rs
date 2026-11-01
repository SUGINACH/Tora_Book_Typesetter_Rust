use serde::{Deserialize, Serialize};
use tauri::State;
use tauri_probe::JobGate;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct LayoutJob {
    job_id: String,
    profile_id: String,
    generation: u64,
    input_sha256: String,
    settings_sha256: String,
}

impl LayoutJob {
    fn validate(&self) -> Result<(), &'static str> {
        if self.job_id.is_empty()
            || self.job_id.len() > 128
            || !self
                .job_id
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || b"-_.".contains(&byte))
        {
            return Err("invalid job id");
        }
        if !matches!(
            self.profile_id.as_str(),
            "legacy-orange-v23"
                | "legacy-green-v23"
                | "legacy-gold-v21"
                | "legacy-yellow-v10"
                | "legacy-imud-v20x"
        ) {
            return Err("unknown compatibility profile");
        }
        if self.generation == 0 {
            return Err("generation must be positive");
        }
        if !is_sha256(&self.input_sha256) || !is_sha256(&self.settings_sha256) {
            return Err("input and settings fingerprints must be 64 hexadecimal characters");
        }
        Ok(())
    }
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct JobAcknowledgement {
    job_id: String,
    generation: u64,
    accepted: bool,
    reason: Option<String>,
}

fn process_layout_job(job: LayoutJob, gate: &JobGate) -> JobAcknowledgement {
    let validation = job.validate();
    let accepted = validation.is_ok() && gate.accept(job.generation);
    let reason = validation
        .err()
        .map(str::to_string)
        .or_else(|| (!accepted).then(|| "stale generation".to_string()));
    JobAcknowledgement {
        job_id: job.job_id,
        generation: job.generation,
        accepted,
        reason,
    }
}

#[tauri::command]
fn submit_layout_job(job: LayoutJob, gate: State<'_, JobGate>) -> JobAcknowledgement {
    process_layout_job(job, gate.inner())
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

#[cfg(test)]
mod tests {
    use super::{process_layout_job, LayoutJob};
    use tauri_probe::JobGate;

    fn valid_job(generation: u64) -> LayoutJob {
        LayoutJob {
            job_id: "job-0001".to_string(),
            profile_id: "legacy-orange-v23".to_string(),
            generation,
            input_sha256: "a".repeat(64),
            settings_sha256: "b".repeat(64),
        }
    }

    #[test]
    fn accepts_a_valid_immutable_profile_snapshot() {
        let gate = JobGate::default();
        let acknowledgement = process_layout_job(valid_job(1), &gate);
        assert!(acknowledgement.accepted);
        assert_eq!(acknowledgement.reason, None);
        assert_eq!(gate.current_generation(), 1);
    }

    #[test]
    fn rejects_unknown_profiles_before_advancing_generation() {
        let gate = JobGate::default();
        let mut job = valid_job(1);
        job.profile_id = "legacy-default".to_string();
        let acknowledgement = process_layout_job(job, &gate);
        assert!(!acknowledgement.accepted);
        assert_eq!(
            acknowledgement.reason.as_deref(),
            Some("unknown compatibility profile")
        );
        assert_eq!(gate.current_generation(), 0);
    }

    #[test]
    fn rejects_invalid_fingerprints_before_advancing_generation() {
        let gate = JobGate::default();
        let mut job = valid_job(1);
        job.input_sha256 = "../untrusted/path".to_string();
        let acknowledgement = process_layout_job(job, &gate);
        assert!(!acknowledgement.accepted);
        assert_eq!(
            acknowledgement.reason.as_deref(),
            Some("input and settings fingerprints must be 64 hexadecimal characters")
        );
        assert_eq!(gate.current_generation(), 0);
    }

    #[test]
    fn rejects_path_like_job_ids() {
        let gate = JobGate::default();
        let mut job = valid_job(1);
        job.job_id = "../project".to_string();
        let acknowledgement = process_layout_job(job, &gate);
        assert!(!acknowledgement.accepted);
        assert_eq!(acknowledgement.reason.as_deref(), Some("invalid job id"));
    }

    #[test]
    fn rejects_stale_generations_without_replacing_the_current_job() {
        let gate = JobGate::default();
        assert!(process_layout_job(valid_job(2), &gate).accepted);
        let stale = process_layout_job(valid_job(1), &gate);
        assert!(!stale.accepted);
        assert_eq!(stale.reason.as_deref(), Some("stale generation"));
        assert_eq!(gate.current_generation(), 2);
    }
}
