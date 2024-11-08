use bumpalo::Bump;
use eyre::bail;
use flecs_ecs::core::{Entity, EntityViewGet, IdOperations, World};
use valence_protocol::packets::play::command_tree_s2c::{NodeData, Parser};

use crate::{get_root_command, CommandContext, Executor, Structure};

pub fn execute(world: &World, entity: Entity, command_raw: &str, bump: &Bump) -> eyre::Result<()> {
    tracing::trace!("Executing command: {}", command_raw);
    let mut command = command_raw.split_ascii_whitespace();

    let mut context = CommandContext::default();

    let mut on = get_root_command().entity_view(world);

    loop {
        let on_structure = on.try_get::<&Structure>(Clone::clone);
        let on_span = tracing::trace_span!("on", on = ?on_structure);
        let _enter = on_span.enter();
        // sequential search is probably (hopefully) sufficient assuming there are not over about 64
        // command branches probably about the same speed as a HashMap

        let Some(arg) = command.next() else {
            if let Some(executor) =
                on.try_get::<&Executor>(|executor| unsafe { extend_lifetime(executor) })
            {
                tracing::debug!("Found executor, executing command");
                tracing::info!("Running with context: {context:?}");
                let executor = executor.executor;
                return executor(world, entity, &context);
            }

            tracing::debug!("Expected next command argument but found none");
            let structure = on.try_get::<&Structure>(Clone::clone);
            let node_data = structure.map_or(NodeData::Root, |s| s.data);
            bail!("Expecting a next command since no executor. Current node: {:?}", node_data);
        };

        tracing::trace!("Processing argument: {}", arg);

        // todo: this is hacky
        let mut children = heapless::Vec::<_, 64>::new();

        on.each_child(|child| {
            children.push(child.id()).unwrap();
        });

        for child in children {
            let child = child.entity_view(world);

            let structure =
                child.get::<&Structure>(|structure| unsafe { extend_lifetime(structure) });

            match &structure.data {
                NodeData::Root => {
                    tracing::warn!("Encountered unexpected root node");
                    bail!("somehow got a root node");
                }
                NodeData::Literal { name } => {
                    if name.eq_ignore_ascii_case(arg) {
                        tracing::trace!("Matched literal: {}", name);
                        on = child;
                        break;
                    }
                }
                NodeData::Argument {
                    name,
                    parser,
                    suggestion: _,
                } => match parser {
                    Parser::Bool => {
                        let Ok(value) = serde_json::from_str::<bool>(arg) else {
                            continue;
                        };
                        context.push(name, bump, value);
                        tracing::trace!("Parsed bool argument {}: {}", name, value);

                        on = child;
                        break;
                    }
                    Parser::Float { .. } => {
                        let Ok(value) = serde_json::from_str::<f32>(arg) else {
                            continue;
                        };
                        let ptr: &mut dyn std::any::Any = bump.alloc(value);
                        context.push(name, bump, value);
                        tracing::trace!("Parsed float argument {}: {}", name, value);
                        on = child;
                        break;
                    }
                    Parser::Double { .. } => {
                        let Ok(value) = serde_json::from_str::<f64>(arg) else {
                            continue;
                        };
                        context.push(name, bump, value);
                        tracing::trace!("Parsed double argument {}: {}", name, value);
                        on = child;
                        break;
                    }
                    Parser::Integer { .. } => {
                        let Ok(value) = serde_json::from_str::<i32>(arg) else {
                            continue;
                        };
                        context.push(name, bump, value);
                        tracing::trace!("Parsed integer argument {}: {}", name, value);
                        on = child;
                        break;
                    }
                    Parser::Long { .. } => {
                        let Ok(value) = serde_json::from_str::<i64>(arg) else {
                            continue;
                        };
                        context.push(name, bump, value);
                        tracing::trace!("Parsed long argument {}: {}", name, value);
                        on = child;
                        break;
                    }
                    Parser::String(_) => {
                        context.push(name, bump, arg.to_string());
                        tracing::trace!("Parsed string argument {}: {}", name, arg);
                        on = child;
                        break;
                    }
                    Parser::Entity { .. }
                    | Parser::GameProfile
                    | Parser::BlockPos
                    | Parser::ColumnPos
                    | Parser::Vec3
                    | Parser::Vec2
                    | Parser::BlockState
                    | Parser::BlockPredicate
                    | Parser::ItemStack
                    | Parser::ItemPredicate
                    | Parser::Color
                    | Parser::Component
                    | Parser::Message
                    | Parser::NbtCompoundTag
                    | Parser::NbtTag
                    | Parser::NbtPath
                    | Parser::Objective
                    | Parser::ObjectiveCriteria
                    | Parser::Operation
                    | Parser::Particle
                    | Parser::Angle
                    | Parser::Rotation
                    | Parser::ScoreboardSlot
                    | Parser::ScoreHolder { .. }
                    | Parser::Swizzle
                    | Parser::Team
                    | Parser::ItemSlot
                    | Parser::ResourceLocation
                    | Parser::Function
                    | Parser::EntityAnchor
                    | Parser::IntRange
                    | Parser::FloatRange
                    | Parser::Dimension
                    | Parser::GameMode
                    | Parser::Time
                    | Parser::ResourceOrTag { .. }
                    | Parser::ResourceOrTagKey { .. }
                    | Parser::Resource { .. }
                    | Parser::ResourceKey { .. }
                    | Parser::TemplateMirror
                    | Parser::TemplateRotation
                    | Parser::Uuid => {
                        tracing::debug!("Encountered unimplemented parser: {:?}", parser);
                        bail!("unimplemented parser");
                    }
                },
            }
        }
    }
}

unsafe fn extend_lifetime<T>(elem: &'_ T) -> &'static T {
    unsafe { core::mem::transmute(elem) }
}
