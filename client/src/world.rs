// Library
use coord::prelude::*;

// Project
use region::{
    chunk::{Chunk, ChunkContainer, ChunkConverter, ChunkFile},
    Container, Key, PersState, VolContainer, VolConverter,
};

// Local
use Client;
use Payloads;
use CHUNK_SIZE;

use std::path::Path;

pub(crate) fn gen_chunk<P: Send + Sync + 'static>(pos: Vec3<i64>, con: &Container<ChunkContainer, P>) {
    let filename = pos.print() + ".dat";
    let filepath = "./saves/".to_owned() + &(filename);
    let path = Path::new(&filepath);
    if path.exists() {
        let mut vol = ChunkFile::new(vec3!(CHUNK_SIZE));
        *vol.file_mut() = filepath;
        con.vols_mut().insert(vol, PersState::File);
    } else {
        let mut vol = Chunk::test(
            vec3!(pos.x * CHUNK_SIZE[0], pos.y * CHUNK_SIZE[1], pos.z * CHUNK_SIZE[2]),
            vec3!(CHUNK_SIZE),
        );
        con.vols_mut().insert(vol, PersState::Raw);
    }
}

impl<P: Payloads> Client<P> {
    pub(crate) fn load_unload_chunks(&self) {
        // Only update chunks if the player exists
        if let Some(player_entity) = self.player_entity() {
            // Find the chunk the player is in
            let player_chunk = player_entity.read().pos().map(|e| e as i64).div_euc(vec3!(CHUNK_SIZE));

            // Generate chunks around the player
            let mut chunks = vec![];
            for i in player_chunk.x - self.view_distance..player_chunk.x + self.view_distance + 1 {
                for j in player_chunk.y - self.view_distance..player_chunk.y + self.view_distance + 1 {
                    for k in player_chunk.z - self.view_distance..player_chunk.z + self.view_distance + 1 {
                        let pos = vec3!(i, j, k);
                        let diff = (player_chunk - pos).snake_length();
                        chunks.push((diff, pos));
                    }
                }
            }

            chunks.sort_by(|a, b| a.0.cmp(&b.0));
            for (_diff, pos) in chunks.iter() {
                if !self.chunk_mgr().contains(*pos) {
                    self.chunk_mgr().gen(*pos);
                } else {
                    if self.chunk_mgr().loaded(*pos) {
                        self.chunk_mgr().persistence().generate(&pos, PersState::Raw);
                        if let Some(con) = self.chunk_mgr().persistence().get(&pos) {
                            if con.payload().is_none() {
                                self.chunk_mgr().gen_payload(*pos);
                            }
                        }
                    }
                }
            }

            const DIFF_TILL_UNLOAD: i64 = 5;
            let unload_chunk_diff = chunks.last().unwrap().0 + DIFF_TILL_UNLOAD;

            //drop old chunks
            {
                let chunks = self.chunk_mgr().persistence().hot();
                for (pos, container) in chunks.iter() {
                    let diff = (player_chunk - *pos).snake_length();
                    if diff > unload_chunk_diff {
                        let mut lock = container.vols_mut();
                        ChunkConverter::convert(pos, &mut lock, PersState::File);
                        lock.remove(PersState::Raw);
                        lock.remove(PersState::Rle);
                        *container.payload_mut() = None;
                    }
                }
            }
        }
    }
}
