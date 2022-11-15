use std::sync::Arc;

use axum::{extract::State, response::IntoResponse};
use procfs::process::LimitValue;

use crate::app_state::AppState;

pub async fn metrics(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let mut metrics = String::new();
    let users = state.users.lock();
    let connected_users = users.iter().filter(|x| x.1.connected).count();
    let blacklisted_users = users.iter().filter(|x| x.1.irc_blacklisted).count();
    metrics.push_str(&prometheus_stat("Connected users", "connected_users", connected_users));
    metrics.push_str(&prometheus_stat(
        "Blocked users",
        "blocked_irc_users",
        &format!("{}", blacklisted_users),
    ));
    metrics.push_str(&prometheus_stat(
        "Cosmetics count",
        "cosmetics",
        &format!("{}", &state.cosmetics.lock().len()),
    ));
    metrics.push_str(&prometheus_stat(
        "Cosmetic User Count",
        "cosmetic_users",
        &format!("{}", &state.users.lock().len()),
    ));
    metrics.push_str(&prometheus_stat(
        "Messages per second",
        "messages_per_second",
        state.messages_sec.load(std::sync::atomic::Ordering::Relaxed),
    ));
    #[cfg(target_os = "linux")]
    add_process_stats(&mut metrics);
    metrics
}

pub fn prometheus_stat<T>(help: &str, name: &str, value: T) -> String
where
    T: std::fmt::Display,
{
    format!("# HELP {name} {help}\n{name} {value}\n\n")
}

#[allow(unused_variables)]
pub fn add_process_stats(r: &mut String) {
    let me = procfs::process::Process::myself().unwrap();
    let me_stat = me.stat().unwrap();
    let tps = procfs::ticks_per_second().unwrap();
    // im entirely unsure what that this is even accurate info.
    // this was all written by copilot

    r.push_str(&prometheus_stat(
        "Total user and system CPU time spent in seconds.",
        "process_cpu_seconds_total",
        me_stat.utime as f64 / tps as f64 + me_stat.stime as f64 / tps as f64,
    ));
    r.push_str(&prometheus_stat(
        "Number of open file descriptors",
        "process_open_fds",
        me.fd().unwrap().count(),
    ));
    r.push_str(&prometheus_stat(
        "Number of threads in this process",
        "process_threads",
        me_stat.num_threads,
    ));

    if let Ok(max) = me.limits() {
        if let LimitValue::Value(v) = max.max_open_files.hard_limit {
            r.push_str(&prometheus_stat(
                "Maximum number of open file descriptors",
                "process_max_fds",
                v,
            ));
        }
        if let LimitValue::Value(v) = max.max_locked_memory.hard_limit {
            r.push_str(&prometheus_stat(
                "Maximum amount of virtual memory available in bytes.",
                "process_virtual_memory_max_bytes",
                v,
            ));
        }
    }

    if let Ok(mem) = me.statm() {
        r.push_str(&prometheus_stat(
            "Virtual memory size in bytes.",
            "process_virtual_memory_bytes",
            mem.size * 4096,
        ));
        r.push_str(&prometheus_stat(
            "Resident set size: number of pages the process has in real memory.",
            "process_resident_memory_bytes",
            mem.resident * 4096,
        ));
    }
    if let Ok(io) = me.io() {
        r.push_str(&prometheus_stat(
            "Number of bytes read.",
            "process_io_read_bytes_total",
            io.rchar,
        ));
        r.push_str(&prometheus_stat(
            "Number of bytes written.",
            "process_io_write_bytes_total",
            io.wchar,
        ));
    }

    r.push_str(&prometheus_stat(
        "Start time of the process since unix epoch in seconds.",
        "process_start_time_seconds",
        me_stat.starttime as f64 / tps as f64,
    ));
    r.push_str(&prometheus_stat(
        "Process heap size in bytes.",
        "process_heap_bytes",
        me.statm().unwrap().size * 4096,
    ));
    r.push_str(&prometheus_stat(
        "Virtual memory size in bytes",
        "process_virtual_memory_bytes",
        me_stat.vsize,
    ));
    r.push_str(&prometheus_stat(
        "Resident set size: number of pages the process has in real memory.",
        "process_resident_memory_bytes",
        me_stat.rss * 4096,
    ));
}
