use bumpalo::Bump;
use eyre::bail;
use flecs_ecs::core::{Entity, EntityViewGet, IdOperations, World};
use valence_protocol::packets::play::command_tree_s2c::{NodeData, Parser};

use crate::{get_root_command, CommandContext, Executor, Structure};

pub fn execute(world: &World, entity: Entity, command_raw: &str, bump: &Bump) -> eyre::Result<()> {
    let mut command = command_raw.split_ascii_whitespace();

    let mut context = CommandContext::default();

    let mut on = get_root_command().entity_view(world);

    loop {
        // sequential search is probably (hopefully) sufficient assuming there are not over about 64
        // command branches probably about the same speed as a HashMap

        let Some(arg) = command.next() else {
            bail!("command not found");
        };

        // todo: this is hacky
        let mut children = heapless::Vec::<_, 64>::new();

        on.each_child(|child| {
            children.push(child.id()).unwrap();
        });

        for child in children {
            let child = child.entity_view(world);

            if let Some(executor) =
                child.try_get::<&Executor>(|executor| unsafe { extend_lifetime(executor) })
            {
                let executor = executor.executor;
                return executor(world, entity, &context);
            }

            let structure =
                child.get::<&Structure>(|structure| unsafe { extend_lifetime(structure) });

            match &structure.data {
                NodeData::Root => {
                    bail!("somehow got a root node");
                }
                NodeData::Literal { name } => {
                    if name.eq_ignore_ascii_case(arg) {
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
                        let ptr: &mut dyn std::any::Any = bump.alloc(value);
                        context.push(name, ptr);
                    }
                    Parser::Float { .. } => {
                        let Ok(value) = serde_json::from_str::<f32>(arg) else {
                            continue;
                        };
                        let ptr: &mut dyn std::any::Any = bump.alloc(value);
                        context.push(name, ptr);
                    }
                    Parser::Double { .. } => {
                        let Ok(value) = serde_json::from_str::<f64>(arg) else {
                            continue;
                        };
                        let ptr: &mut dyn std::any::Any = bump.alloc(value);
                        context.push(name, ptr);
                    }
                    Parser::Integer { .. } => {
                        let Ok(value) = serde_json::from_str::<i32>(arg) else {
                            continue;
                        };
                        let ptr: &mut dyn std::any::Any = bump.alloc(value);
                        context.push(name, ptr);
                    }
                    Parser::Long { .. } => {
                        let Ok(value) = serde_json::from_str::<i64>(arg) else {
                            continue;
                        };
                        let ptr: &mut dyn std::any::Any = bump.alloc(value);
                        context.push(name, ptr);
                    }
                    Parser::String(_) => {
                        let ptr: &mut dyn std::any::Any = bump.alloc(arg.to_string());
                        context.push(name, ptr);
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
                    | Parser::Uuid => bail!("unimplemented parser"),
                },
            }
        }
    }
}

unsafe fn extend_lifetime<T>(elem: &'_ T) -> &'static T {
    unsafe { core::mem::transmute(elem) }
}
