use bumpalo::Bump;
use flecs_ecs::core::World;
use hyperion_command::{
    dsl, dsl::Parser, execute, get_command_packet, get_root_command, print_command_tree,
    HyperionCommandModule,
};

#[test]
fn test_complex_command() {
    color_eyre::install().unwrap();
    tracing_subscriber::fmt::init();

    let world = World::new();
    world.import::<HyperionCommandModule>();

    use std::sync::atomic::{AtomicBool, Ordering};

    static GAMEMODE_EXECUTED: AtomicBool = AtomicBool::new(false);
    static GAMEMODE_WITH_PLAYER_EXECUTED: AtomicBool = AtomicBool::new(false);
    static TP_EXECUTED: AtomicBool = AtomicBool::new(false);
    static TP_WITH_DEST_EXECUTED: AtomicBool = AtomicBool::new(false);
    static TIME_SET_EXECUTED: AtomicBool = AtomicBool::new(false);

    // Create a command: /gamemode <mode> [player]
    dsl::cmd_with(&world, "gamemode", |scope| {
        scope.argument("mode", Parser::String, |scope| {
            scope
                .executor(|_world, _entity, context| {
                    let mode = context.get::<String>("mode")?;
                    println!("oiii");
                    assert_eq!(mode, "creative");
                    GAMEMODE_EXECUTED.store(true, Ordering::SeqCst);
                    Ok(())
                })
                .argument("player", Parser::String, |scope| {
                    scope.executor(|_world, _entity, context| {
                        let mode = context.get::<String>("mode")?;
                        let player = context.get::<String>("player")?;
                        assert_eq!(mode, "creative");
                        assert_eq!(player, "player");
                        GAMEMODE_WITH_PLAYER_EXECUTED.store(true, Ordering::SeqCst);
                        Ok(())
                    });
                });
        });
    });

    // Create a command: /tp <target> [destination]
    dsl::cmd_with(&world, "tp", |scope| {
        scope.argument("target", Parser::String, |scope| {
            scope
                .executor(|_world, _entity, context| {
                    let target = context.get::<String>("target")?;
                    assert_eq!(target, "target");
                    TP_EXECUTED.store(true, Ordering::SeqCst);
                    Ok(())
                })
                .argument("destination", Parser::String, |scope| {
                    scope.executor(|_world, _entity, context| {
                        println!("CONTEXT: {context:?}");
                        let target = context.get::<String>("target")?;
                        let destination = context.get::<String>("destination")?;
                        assert_eq!(target, "target");
                        assert_eq!(destination, "destination");
                        TP_WITH_DEST_EXECUTED.store(true, Ordering::SeqCst);
                        Ok(())
                    });
                });
        });
    });

    // Create a command: /time set <value>
    dsl::cmd_with(&world, "time", |scope| {
        scope.literal("set", |scope| {
            scope.argument("value", 0_i32.., |scope| {
                scope.executor(|_world, _entity, context| {
                    let value = context.get::<i32>("value")?;
                    assert_eq!(*value, 6000);
                    assert_eq!(value / 20, 300); // Verify seconds calculation
                    TIME_SET_EXECUTED.store(true, Ordering::SeqCst);
                    Ok(())
                });
            });
        });
    });

    print_command_tree(&world);

    let mock_player = world.entity();

    let command = "gamemode creative";
    let mut bump = Bump::new();
    execute(&world, *mock_player, command, &bump).unwrap();
    assert!(GAMEMODE_EXECUTED.load(Ordering::SeqCst));
    GAMEMODE_EXECUTED.store(false, Ordering::SeqCst);

    let command = "tp target destination";
    bump.reset();
    execute(&world, *mock_player, command, &bump).unwrap();
    assert!(TP_WITH_DEST_EXECUTED.load(Ordering::SeqCst));
    TP_WITH_DEST_EXECUTED.store(false, Ordering::SeqCst);

    let command = "time set 6000";
    bump.reset();
    execute(&world, *mock_player, command, &bump).unwrap();
    assert!(TIME_SET_EXECUTED.load(Ordering::SeqCst));
    TIME_SET_EXECUTED.store(false, Ordering::SeqCst);
}
