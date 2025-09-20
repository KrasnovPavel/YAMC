use bevy::prelude::*;
use super::Chunk;

pub struct ChunkInteractionPlugin;

impl Plugin for ChunkInteractionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(First, reset_updated);
    }
}

fn reset_updated(mut chunks: Query<&mut Chunk>)
{
    for mut chunk in chunks.iter_mut() {
        chunk.is_updated = false;
    }
}