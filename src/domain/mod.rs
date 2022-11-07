/*
This file is an experiment on building up a set of processes and functions in enums and then going from there.
Sort of like encoding your domain in functions and types.

Then a client will drive this, like hexagonal architecture.
*/

mod folder_heirarchy;

use crate::{containers::Readonly, domain::folder_heirarchy::Node};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub struct Domain {
    project_path: Readonly<PathBuf>,
    audio: Node,
}

impl Domain {
    fn audio_path(&self) -> PathBuf {
        let mut p = self.project_path.item().clone();
        p.push(AUDIO_FOLDER);
        p
    }

    /// Creates a new domain instance.
    pub fn new(project_path: PathBuf) -> Self {
        if !project_path.exists() {
            std::fs::create_dir_all(&project_path).unwrap();
        }

        // Init domain.
        let mut domain = Domain {
            project_path: Readonly::new(project_path.clone()),
            audio: default_nodes(project_path),
        };

        // Load up existing nodes if present.
        let nodes = Node::load(&domain.audio_path());
        if !nodes.is_empty() {
            domain.audio = nodes;
        }

        domain
    }

    /// Saves the domain to disk.
    pub fn save(&mut self) {
        self.audio.save();
    }
}

/// Reads all elements in the directory
fn get_files(path: &PathBuf) -> Vec<std::fs::DirEntry> {
    let mut entries = vec![];
    for directory in std::fs::read_dir(&path).into_iter() {
        for entry in directory.filter_map(|f| f.ok()) {
            entries.push(entry);
        }
    }

    entries
}

/// The default project nodes.
fn default_nodes(project_path: PathBuf) -> Node {
    let mut audio = {
        let mut one_shot = Node::new(
            "one-shot",
            "A collection of single samples that represent a single note.",
        );
        one_shot.push(("kd", "Kick drum").into());

        let mut one_shot = Node::new(
            "one-shot",
            "A collection of single samples that represent a single note.",
        );
        one_shot.push(("kd", "Kick drum").into());

        let loops = ("loops", "A collection of loops that can play continously.").into();

        let mut audio = Node::new(
            AUDIO_FOLDER,
            "A collection of audio files meant for sampling.",
        );
        audio.push(one_shot).push(loops);
        audio
    };

    audio.rebuild_path(Some(project_path));

    audio
}

const AUDIO_FOLDER: &'static str = "audio";
