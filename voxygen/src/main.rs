#![feature(nll, euclidean_division, arbitrary_self_types, duration_float)]

// Graphics
#[macro_use]
extern crate gfx;

#[macro_use]
extern crate log;

// Modules
mod camera;
mod game;
mod key_state;
mod keybinds;
mod tests;
mod ui;
mod window;

// > Rendering
mod consts;
mod hud;
mod pipeline;
mod renderer;
mod shader;

// > Pipelines
mod audio;
mod skybox;
mod tonemapper;
mod voxel;

// Standard
use std::{
    io::{self, Write},
    panic,
    path::{Path, PathBuf},
    thread,
    time::Duration,
};

// Library
use chrono::{DateTime, TimeZone, Utc};
use parking_lot::Mutex;

// Project
use client::PlayMode;
use common::get_version;

// Local
use crate::{game::Game, renderer::RendererInfo};

// START Environment variables
const GIT_HASH: Option<&'static str> = option_env!("GIT_HASH");
const GIT_TIME: Option<&'static str> = option_env!("GIT_TIME");
const PROFILE: Option<&'static str> = option_env!("PROFILE");
const BUILD_TIME: Option<&'static str> = option_env!("BUILD_TIME");

pub fn get_git_hash() -> &'static str { GIT_HASH.unwrap_or("UNKNOWN GIT HASH") }
pub fn get_git_time() -> DateTime<Utc> { Utc.timestamp(GIT_TIME.unwrap_or("-1").to_string().parse().unwrap(), 0) }
pub fn get_profile() -> &'static str { PROFILE.unwrap_or("UNKNOWN PROFILE") }

pub fn get_build_time() -> DateTime<Utc> { Utc.timestamp(BUILD_TIME.unwrap_or("-1").to_string().parse().unwrap(), 0) }
pub fn get_shader_dir() -> &'static Path { Path::new(option_env!("VOXYGEN_SHADERS").unwrap_or("shaders/")) }
// END Environment variables

pub fn get_shader_path(rpath: &str) -> PathBuf { get_shader_dir().join(rpath) }

static RENDERER_INFO: Mutex<Option<RendererInfo>> = Mutex::new(None);

fn set_panic_handler() {
    let default_hook = panic::take_hook();
    panic::set_hook(Box::new(move |details| {
        default_hook(details);
        if let Some(info) = &*RENDERER_INFO.lock() {
            println!(
                "Graphics card info - vendor: {} model: {} OpenGL: {}",
                info.vendor, info.model, info.gl_version
            );
        }
    }));
}

fn main() {
    pretty_env_logger::init();
    set_panic_handler();

    info!("Starting Voxygen... Version: {}", get_version());

    let mut args = std::env::args();
    let mut remote_addr = String::new();
    let mut remote_choice = String::new();

    // expects single command line argument that is the remote_addr
    if args.len() == 2 {
        remote_addr = args.nth(1).expect("No argument");
    } else {
        println!("");
        println!("Which server you want to connect to?");
        println!("    Press (1) to connect to the public veloren server (default)");
        println!("    Press (2) to connect to localhost");
        println!("    Press (3) to connect to another internet server");
        println!("");
        io::stdout().flush().expect("Failed to flush");
        io::stdin().read_line(&mut remote_choice).unwrap();
        let remote_choice = remote_choice.trim();
        if remote_choice == "2" {
            remote_addr = "127.0.0.1:59003".to_string();
        } else if remote_choice == "3" {
            // If args aren't correct then read from stdin
            print!("Enter address (e.g. 127.0.0.1:59003):");
            io::stdout().flush().expect("Failed to flush");
            io::stdin().read_line(&mut remote_addr).unwrap();
        } else {
            remote_addr = "veloren.pftclan.de:38888".to_string();
        }

        remote_addr = remote_addr.trim().to_string();
    }

    println!("What name do you want to use?");
    let mut name_choice = String::new();
    io::stdout().flush().expect("Failed to flush");
    io::stdin().read_line(&mut name_choice).unwrap();
    let mut name_choice = name_choice.trim();
    if name_choice.len() == 0 {
        println!("No name chosen, generating random one...");
        name_choice = common::util::names::generate();
    }

    println!("");
    println!("What view distance do you want to use?");
    println!("For a smooth experience on slower hardware, we recommend 80.");
    println!("For faster computers, 400 is advised.");
    println!("If you experience lag, restart Veloren and change this setting again.");
    println!("");
    let mut view_distance_choice = String::new();
    io::stdout().flush().expect("Failed to flush");
    io::stdin().read_line(&mut view_distance_choice).unwrap();
    let view_distance = view_distance_choice.trim().parse::<i64>().unwrap_or_else(|_| {
        println!("Invalid input, defaulting to 80.");
        80
    });
    println!("using a view distance of {}.", view_distance);

    println!("Connecting to {}", remote_addr);

    // wait 100ms to give the user time to lift their finger up from the enter key so the chat isn't opened immediately after start
    thread::sleep(Duration::from_millis(100));

    Game::new(PlayMode::Character, name_choice, remote_addr, view_distance).run();
}
