//! Sim Metaverse node -- UE5 metaverse camera simulator.
//!
//! Subscribes to SimRequest, simulates simple physics by linearly
//! interpolating position and quaternion, publishes SimStatus at 60 Hz,
//! and sends a UDP pose packet to UE5.
//!
//! If `BEHAVE_UE_PROJECT` is set, automatically launches UE5 in standalone
//! game mode (`-game`) unless it is already running.

use std::net::UdpSocket;
use std::process::Command;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result};
use iceoryx2::prelude::*;
use log::{info, warn};

use behave::topics;
use behave::topics::sim::CameraPose;
use behave::topics::sim::status::SimStatus;

const UE5_UDP_PORT: u16 = 9876;
const DEFAULT_UE_RES_X: u32 = 1920;
const DEFAULT_UE_RES_Y: u32 = 1080;
const DEFAULT_UE_FPS: u32 = 30;

const TICK_HZ: u64 = 60;
const TICK_DT: f64 = 1.0 / TICK_HZ as f64;
const DEFAULT_LINEAR_SPEED: f64 = 2.0;
const DEFAULT_ANGULAR_SPEED: f64 = 1.0;

/// Flat pose packet sent over UDP to UE5 (64 bytes, little-endian).
#[repr(C, packed)]
struct UdpPosePacket {
    x: f64, y: f64, z: f64,
    qw: f64, qx: f64, qy: f64, qz: f64,
    utime: u64,
}

fn env_f64(key: &str, default: f64) -> f64 {
    std::env::var(key).ok().and_then(|v| v.parse().ok()).unwrap_or(default)
}

fn env_u32(key: &str, default: u32) -> u32 {
    std::env::var(key).ok().and_then(|v| v.parse().ok()).unwrap_or(default)
}

// ── UE5 launch helpers ──────────────────────────────────────────

fn ue5_already_running() -> bool {
    Command::new("pgrep")
        .arg("-f")
        .arg("UnrealEditor.*BehaveSim")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// macOS Retina display scale factor (2 on Retina, 1 otherwise).
fn macos_display_scale() -> u32 {
    Command::new("osascript")
        .arg("-e").arg("use framework \"AppKit\"")
        .arg("-e").arg("set sf to (current application's NSScreen's mainScreen()'s backingScaleFactor()) as integer")
        .arg("-e").arg("return sf")
        .output()
        .ok()
        .and_then(|o| String::from_utf8_lossy(&o.stdout).trim().parse().ok())
        .unwrap_or(1)
}

/// Find UnrealEditor binary on macOS via Spotlight.
fn find_ue5_editor() -> Option<std::path::PathBuf> {
    let output = Command::new("mdfind")
        .arg("kMDItemFSName == 'UnrealEditor.app' && kMDItemContentType == 'com.apple.application-bundle'")
        .output()
        .ok()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout.lines()
        .map(|line| std::path::PathBuf::from(line).join("Contents/MacOS/UnrealEditor"))
        .find(|p| p.exists())
}

/// Write Saved/Config/MacEditor/GameUserSettings.ini with the correct
/// resolution, windowed mode, and FPS cap. UE5 reads this file at startup
/// and it overrides command-line args.
fn write_game_user_settings(project_dir: &std::path::Path, res_x: u32, res_y: u32, fps: u32) -> Result<()> {
    let config_dir = project_dir.parent().unwrap().join("Saved/Config/MacEditor");
    std::fs::create_dir_all(&config_dir)?;

    let path = config_dir.join("GameUserSettings.ini");
    std::fs::write(&path, format!("\
[/Script/Engine.GameUserSettings]
bUseVSync=False
ResolutionSizeX={res_x}
ResolutionSizeY={res_y}
LastUserConfirmedResolutionSizeX={res_x}
LastUserConfirmedResolutionSizeY={res_y}
FullscreenMode=2
LastConfirmedFullscreenMode=2
PreferredFullscreenMode=2
Version=5
FrameRateLimit={fps}.000000
DesiredScreenWidth={res_x}
DesiredScreenHeight={res_y}
LastUserConfirmedDesiredScreenWidth={res_x}
LastUserConfirmedDesiredScreenHeight={res_y}
"))?;

    info!("wrote {}", path.display());
    Ok(())
}

/// Launch UE5 based on `BEHAVE_UE_MODE` ("game" or "editor").
/// Skips if UE5 is already running with BehaveSim.
fn launch_ue5() -> Result<()> {
    let project_rel = match std::env::var("BEHAVE_UE_PROJECT") {
        Ok(p) => p,
        Err(_) => {
            warn!("BEHAVE_UE_PROJECT not set -- launch UE5 manually");
            return Ok(());
        }
    };

    let project_path = std::path::Path::new(&project_rel)
        .canonicalize()
        .with_context(|| format!("UE5 project not found: {project_rel}"))?;

    if ue5_already_running() {
        info!("UE5 already running -- skipping launch");
        return Ok(());
    }

    let mode = std::env::var("BEHAVE_UE_MODE")
        .unwrap_or_else(|_| "game".to_string())
        .to_lowercase();

    match mode.as_str() {
        "editor" => {
            info!("UE5 mode: editor (manual launch)");
            info!("  open -a \"UnrealEditor\" {}", project_path.display());
        }
        _ => {
            let editor = std::env::var("BEHAVE_UE_EDITOR_CMD")
                .map(std::path::PathBuf::from)
                .ok()
                .or_else(find_ue5_editor)
                .context("could not find UnrealEditor -- set BEHAVE_UE_EDITOR_CMD")?;

            let res_x = env_u32("BEHAVE_UE_RES_X", DEFAULT_UE_RES_X);
            let res_y = env_u32("BEHAVE_UE_RES_Y", DEFAULT_UE_RES_Y);
            let fps = env_u32("BEHAVE_UE_FPS", DEFAULT_UE_FPS);
            let scale = macos_display_scale();
            let win_x = res_x / scale;
            let win_y = res_y / scale;

            write_game_user_settings(&project_path, win_x, win_y, fps)?;

            info!("UE5 mode: game (auto-launch)");
            info!("  editor:     {}", editor.display());
            info!("  project:    {}", project_path.display());
            info!("  resolution: {res_x}x{res_y} ({win_x}x{win_y} pts, {scale}x scale)");
            info!("  max FPS:    {fps}");

            Command::new(&editor)
                .arg(project_path.to_str().unwrap())
                .arg("-game")
                .arg("-windowed")
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .spawn()
                .with_context(|| format!("failed to launch {}", editor.display()))?;

            info!("UE5 launched");
        }
    }

    Ok(())
}

// ── Sim physics ─────────────────────────────────────────────────

struct Pose { x: f64, y: f64, z: f64, qw: f64, qx: f64, qy: f64, qz: f64 }

impl Pose {
    fn origin() -> Self { Self { x: 0.0, y: 0.0, z: 0.0, qw: 1.0, qx: 0.0, qy: 0.0, qz: 0.0 } }
}

fn move_toward(current: f64, target: f64, max_step: f64) -> f64 {
    let diff = target - current;
    if diff.abs() <= max_step { target } else { current + diff.signum() * max_step }
}

fn qdot(a: &Pose, b: &Pose) -> f64 { a.qw*b.qw + a.qx*b.qx + a.qy*b.qy + a.qz*b.qz }

fn qnorm(p: &mut Pose) {
    let len = (p.qw*p.qw + p.qx*p.qx + p.qy*p.qy + p.qz*p.qz).sqrt();
    if len > 1e-12 { p.qw /= len; p.qx /= len; p.qy /= len; p.qz /= len; }
}

fn slerp_step(cur: &mut Pose, tgt: &Pose, max_angle: f64) {
    let dot = qdot(cur, tgt).clamp(-1.0, 1.0);
    let angle = dot.abs().acos() * 2.0;
    if angle < 1e-6 { cur.qw = tgt.qw; cur.qx = tgt.qx; cur.qy = tgt.qy; cur.qz = tgt.qz; return; }
    let t = (max_angle / angle).min(1.0);
    let sign = if dot < 0.0 { -1.0 } else { 1.0 };
    cur.qw = cur.qw*(1.0-t) + tgt.qw*sign*t;
    cur.qx = cur.qx*(1.0-t) + tgt.qx*sign*t;
    cur.qy = cur.qy*(1.0-t) + tgt.qy*sign*t;
    cur.qz = cur.qz*(1.0-t) + tgt.qz*sign*t;
    qnorm(cur);
}

fn now_us() -> u64 { SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros() as u64 }

// ── Main loop ───────────────────────────────────────────────────

pub fn run() -> Result<()> {
    behave::logging::init("SimMeta");

    launch_ue5()?;

    let linear_speed = env_f64("BEHAVE_MAX_LINEAR_SPEED", DEFAULT_LINEAR_SPEED);
    let angular_speed = env_f64("BEHAVE_MAX_ANGULAR_SPEED", DEFAULT_ANGULAR_SPEED);
    info!("starting (60 Hz, lin={linear_speed:.1}m/s, ang={angular_speed:.2}rad/s)");

    let node = NodeBuilder::new().create::<iceoryx2::prelude::ipc::Service>()?;
    let req_sub = topics::sim::request::subscribe(&node)?;
    let status_pub = topics::sim::status::publish(&node)?;

    let udp = UdpSocket::bind("0.0.0.0:0")?;
    let ue5_addr = format!("127.0.0.1:{UE5_UDP_PORT}");
    info!("sending UDP pose to {ue5_addr}");

    let mut current = Pose::origin();
    let mut target = Pose::origin();
    let tick = Duration::from_secs(1) / TICK_HZ as u32;

    info!("ready");

    loop {
        let t0 = Instant::now();

        while let Some(req) = topics::receive_native(&req_sub)? {
            target = Pose {
                x: req.pose.x, y: req.pose.y, z: req.pose.z,
                qw: req.pose.qw, qx: req.pose.qx, qy: req.pose.qy, qz: req.pose.qz,
            };
        }

        let lin_step = linear_speed * TICK_DT;
        current.x = move_toward(current.x, target.x, lin_step);
        current.y = move_toward(current.y, target.y, lin_step);
        current.z = move_toward(current.z, target.z, lin_step);

        let ang_step = angular_speed * TICK_DT;
        slerp_step(&mut current, &target, ang_step);

        let utime = now_us();

        topics::publish(&status_pub, SimStatus {
            pose: CameraPose {
                x: current.x, y: current.y, z: current.z,
                qw: current.qw, qx: current.qx, qy: current.qy, qz: current.qz,
            },
            utime,
        })?;

        let pkt = UdpPosePacket {
            x: current.x, y: current.y, z: current.z,
            qw: current.qw, qx: current.qx, qy: current.qy, qz: current.qz,
            utime,
        };
        let bytes: &[u8] = unsafe {
            std::slice::from_raw_parts(&pkt as *const UdpPosePacket as *const u8, std::mem::size_of::<UdpPosePacket>())
        };
        let _ = udp.send_to(bytes, &ue5_addr);

        let elapsed = t0.elapsed();
        if elapsed < tick { std::thread::sleep(tick - elapsed); }
    }
}
