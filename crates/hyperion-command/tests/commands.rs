use bumpalo::Bump;
use flecs_ecs::core::World;
use hyperion_command::{dsl, dsl::Parser, execute, get_root_command, HyperionCommandModule};

#[test]
fn test_complex_command() {
    let world = World::new();
    world.import::<HyperionCommandModule>();

    // Create a command: /gamemode <mode> [player]
    dsl::cmd_with(&world, "gamemode", |scope| {
        scope
            .argument("mode", Parser::String)
            .argument_with("player", Parser::String, |scope| {
                scope.executor(|_world, _entity, context| {
                    let mode = context.get::<String>("mode")?;
                    let player = context.get::<String>("player")?;
                    println!("Setting gamemode {mode} for player {player}");
                    Ok(())
                });
            })
            .executor(|_world, _entity, context| {
                let mode = context.get::<String>("mode")?;
                println!("Setting gamemode {mode} for self");
                Ok(())
            });
    });

    // Create a command: /tp <target> [destination]
    dsl::cmd_with(&world, "tp", |scope| {
        scope
            .argument("target", Parser::String)
            .argument_with("destination", Parser::String, |scope| {
                scope.executor(|_world, _entity, context| {
                    let target = context.get::<String>("target")?;
                    let destination = context.get::<String>("destination")?;
                    println!("Teleporting {target} to {destination}");
                    Ok(())
                });
            })
            .executor(|_world, _entity, context| {
                let target = context.get::<String>("target")?;
                println!("Teleporting self to {target}");
                Ok(())
            });
    });

    // Create a command: /time set <value>
    dsl::cmd_with(&world, "time", |scope| {
        scope.literal_with("set", |scope| {
            scope
                .argument("value", 0_i32..)
                .executor(|_world, _entity, context| {
                    let value = context.get::<i32>("value")?;
                    let value_seconds = value / 20;
                    println!("Setting time to {value} which is {value_seconds} seconds");
                    Ok(())
                });
        });
    });

    let mock_player = world.entity();

    let command = "gamemode creative";
    let mut bump = Bump::new();
    execute(&world, *mock_player, command, &bump).unwrap();

    let command = "tp target destination";
    bump.reset();
    execute(&world, *mock_player, command, &bump).unwrap();

    let command = "time set 6000";
    bump.reset();
    execute(&world, *mock_player, command, &bump).unwrap();
}
