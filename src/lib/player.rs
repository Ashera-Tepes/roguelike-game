use rltk::{Rltk, VirtualKeyCode::*};
use specs::prelude::*;
use std::cmp::{max, min};

use crate::{Map, Player, Position, RunState, State, TileType, Viewshed};

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let players = ecs.read_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let map = ecs.fetch::<Map>();

    for (_player, pos, viewshed) in (&players, &mut positions, &mut viewsheds).join() {
        let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);
        if map.tiles[destination_idx] != TileType::Wall {
            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));

            viewshed.dirty = true;
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    // Player Movement
    match ctx.key {
        None => return RunState::Paused, // Nothing happened
        Some(key) => match key {
            Left | Numpad4 | A | H => try_move_player(-1, 0, &mut gs.ecs),

            Right | Numpad6 | D | L => try_move_player(1, 0, &mut gs.ecs),

            Up | Numpad8 | W | K => try_move_player(0, -1, &mut gs.ecs),

            Down | Numpad2 | S | J => try_move_player(0, 1, &mut gs.ecs),

            _ => return RunState::Paused,
        },
    }
    RunState::Running
}
