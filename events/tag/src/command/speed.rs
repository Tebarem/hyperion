use clap::Parser;
use flecs_ecs::core::{Entity, EntityView, EntityViewGet, WorldGet, WorldProvider};
use hyperion::{
    net::{Compose, ConnectionId, DataBundle, agnostic},
    valence_protocol::packets::play::{
        PlayerAbilitiesS2c, player_abilities_s2c::PlayerAbilitiesFlags,
    },
};
use hyperion_clap::{CommandPermission, MinecraftCommand};

#[derive(Parser, CommandPermission, Debug)]
#[command(name = "speed")]
#[command_permission(group = "Moderator")]
pub struct SpeedCommand {
    amount: f32,
}

impl MinecraftCommand for SpeedCommand {
    fn execute(self, system: EntityView<'_>, caller: Entity) {
        let world = system.world();
        let msg = format!("Setting speed to {}", self.amount);
        let chat = agnostic::chat(msg);

        world.get::<&Compose>(|compose| {
            caller.entity_view(world).get::<&ConnectionId>(|stream| {
                let packet = speed_packet(self.amount);

                let mut bundle = DataBundle::new(compose, system);
                bundle.add_packet(&packet).unwrap();
                bundle.add_packet(&chat).unwrap();

                bundle.unicast(*stream).unwrap();
            });
        });
    }
}

fn speed_packet(amount: f32) -> PlayerAbilitiesS2c {
    PlayerAbilitiesS2c {
        flags: PlayerAbilitiesFlags::default()
            .with_flying(true)
            .with_allow_flying(true),
        flying_speed: amount,
        fov_modifier: 0.0,
    }
}
