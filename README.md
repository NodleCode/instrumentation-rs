# `instrumentation-rs`: Simple `tracing-rs` initialization for GCP and local environments
Put very simply, add this crate to your project and call `instrumentation::init` to initlization `tracing-rs` correctly for each environment you run in.

Meaning:
- if you run as a GCP Cloud Run or Kubernetes pod, we will init with JSON based logging
- if you run locally, we will use plain text pretty logging

In either cases you can use `RUST_LOG` to control what is logged as usual.