use flecs_ecs::core::World;
use hyperion_clap::{MinecraftCommand, hyperion_command::CommandRegistry};

use crate::command::{
    class::ClassCommand, fly::FlyCommand, raycast::RaycastCommand, replace::ReplaceCommand,
    speed::SpeedCommand, xp::XpCommand, zombie::ZombieCommand,
};

mod class;
mod fly;
mod raycast;
mod replace;
mod speed;
mod xp;

// spawn zombie
mod zombie;

pub fn register(registry: &mut CommandRegistry, world: &World) {
    ClassCommand::register(registry, world);
    FlyCommand::register(registry, world);
    RaycastCommand::register(registry, world);
    ReplaceCommand::register(registry, world);
    SpeedCommand::register(registry, world);
    XpCommand::register(registry, world);
    ZombieCommand::register(registry, world);
}
