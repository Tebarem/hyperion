#![feature(downcast_unchecked)]
#![feature(ptr_metadata)]

use std::{any::Any, fmt::Debug, ptr::DynMetadata};

use bumpalo::Bump;
use flecs_ecs::{
    core::{Entity, EntityViewGet, IdOperations, World},
    macros::Component,
    prelude::Module,
};
use hyperion::valence_protocol::packets::play::command_tree_s2c::NodeData;
use serde::Deserialize;
use snafu::Snafu;
use tracing::warn;
use valence_protocol::{
    packets::play::command_tree_s2c::{Node, Parser},
    VarInt,
};

mod execute;
pub use execute::execute;

#[derive(Debug, Snafu)]
pub enum CommandContextError<'a> {
    #[snafu(display("Argument '{name}' not found"))]
    ArgumentNotFound { name: &'a str },

    #[snafu(display("Failed to deserialize argument '{name}': {source}"))]
    DeserializationError {
        name: &'a str,
        source: serde_json::Error,
    },
}

pub type ExecutorFn = fn(&World, id: Entity, &CommandContext<'_, '_>) -> eyre::Result<()>;

pub mod dsl;

#[derive(Default)]
pub struct CommandContext<'a, 'b> {
    names: heapless::Vec<&'a str, 64>,

    // todo: use poitner metadata so data is not repeated twice
    data: heapless::Vec<&'b dyn Any, 64>,
    dbg: heapless::Vec<&'b dyn Debug, 64>,
}

impl std::fmt::Debug for CommandContext<'_, '_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map = f.debug_map();

        for i in 0..self.names.len() {
            let name = self.names[i];
            let dbg = self.dbg[i];
            map.entry(&name, &dbg);
        }

        map.finish()
    }
}

impl<'a, 'b> CommandContext<'a, 'b> {
    fn push<T: 'static + Debug + Clone>(&mut self, name: &'a str, bump: &'b Bump, data: T) {

        let alloc: &mut dyn Any = bump.alloc(data.clone());
        let dbg_meta: &'b dyn Debug = bump.alloc(data);

        
        self.names.push(name).unwrap();
        self.data.push(alloc).unwrap();
        self.dbg.push(dbg_meta).unwrap();
    }

    pub fn get<'c: 'b, 'd, T: 'static>(
        &'c self,
        name: &'d str,
    ) -> Result<&'b T, CommandContextError<'d>> {
        let index = self
            .names
            .iter()
            .enumerate()
            .find_map(|(idx, elem_name)| elem_name.eq_ignore_ascii_case(name).then_some(idx))
            .ok_or(CommandContextError::ArgumentNotFound { name })?;
        
        tracing::trace!("Getting argument {name} at index {index}");

        let result = *self.data.get(index).unwrap();
        Ok(unsafe { result.downcast_ref_unchecked() })
    }
}

#[derive(Component, Clone, Debug)]
pub struct Structure {
    data: NodeData,
}

#[derive(Component)]
struct Help {
    text: String,
}

#[derive(Component)]
pub struct Executor {
    executor: ExecutorFn,
}

#[derive(Component)]
pub struct HyperionCommandModule;

impl Module for HyperionCommandModule {
    fn module(world: &World) {
        world.component::<Structure>();
        world.component::<Executor>();
        world.component::<Help>();

        let root_command = world.entity().set(Structure::ROOT);

        ROOT_COMMAND.set(root_command.id()).unwrap();
    }
}

pub(crate) static ROOT_COMMAND: once_cell::sync::OnceCell<Entity> =
    once_cell::sync::OnceCell::new();

pub fn get_root_command() -> Entity {
    *ROOT_COMMAND.get().unwrap()
}

impl Structure {
    pub const ROOT: Self = Self {
        data: NodeData::Root,
    };

    #[must_use]
    pub fn literal(name: &str) -> Self {
        Self {
            data: NodeData::Literal {
                name: name.to_string(),
            },
        }
    }

    #[must_use]
    pub fn argument(name: &str, parser: Parser) -> Self {
        Self {
            data: NodeData::Argument {
                name: name.to_string(),
                parser,
                suggestion: None,
            },
        }
    }
}

// we want a get command packet

const MAX_DEPTH: usize = 64;

pub fn print_command_tree(world: &World) {
    fn print_node(world: &World, entity: Entity, depth: usize) {
        if depth >= MAX_DEPTH {
            warn!("command tree depth exceeded. Skipping subtree. Circular reference?");
            return;
        }

        let indent = "  ".repeat(depth);

        let entity = entity.entity_view(world);

        let hash_executor = entity.has::<Executor>();

        entity.try_get::<&Structure>(|structure| {
            let executor_marker = if hash_executor { "*" } else { "" };
            match &structure.data {
                NodeData::Root => println!("{indent}ROOT{executor_marker}"),
                NodeData::Literal { name } => println!("{indent}{name}{executor_marker}"),
                NodeData::Argument { name, parser, .. } => {
                    println!("{indent}<{name}: {parser:?}>{executor_marker}");
                }
            }

            world.entity_from_id(entity).each_child(|child| {
                print_node(world, child.id(), depth + 1);
            });
        });
    }

    print_node(world, get_root_command(), 0);
}

pub fn get_command_packet(world: &World) -> valence_protocol::packets::play::CommandTreeS2c {
    struct StackElement {
        depth: usize,
        ptr: usize,
        entity: Entity,
    }

    let root = get_root_command();


    let mut commands = Vec::new();

    let mut stack = vec![StackElement {
        depth: 0,
        ptr: 0,
        entity: root,
    }];

    commands.push(Node {
        data: NodeData::Root,
        executable: false,
        children: vec![],
        redirect_node: None,
    });

    while let Some(StackElement {
        depth,
        entity,
        ptr: parent_ptr,
    }) = stack.pop()
    {
        if depth >= MAX_DEPTH {
            warn!("command tree depth exceeded. Exiting early. Circular reference?");
            break;
        }

        world.entity_from_id(entity).each_child(|child| {
            child.try_get::<&Structure>(|command| {
                let ptr = commands.len();

                commands.push(Node {
                    data: command.data.clone(),
                    executable: true,
                    children: Vec::new(),
                    redirect_node: None,
                });

                let node = &mut commands[parent_ptr];
                node.children.push(i32::try_from(ptr).unwrap().into());

                stack.push(StackElement {
                    depth: depth + 1,
                    ptr,
                    entity: child.id(),
                });
            });
        });
    }

    valence_protocol::packets::play::CommandTreeS2c {
        commands,
        root_index: VarInt(0),
    }
}
#[cfg(test)]
mod tests {
    use flecs_ecs::prelude::*;

    use super::*;

    #[test]
    fn test_empty_command_tree() {
        let world = World::new();
        world.component::<Structure>();
        let root = world.entity();

        let packet = get_command_packet(&world);

        assert_eq!(packet.commands.len(), 1);
        assert_eq!(packet.root_index, VarInt(0));
        assert_eq!(packet.commands[0].data, NodeData::Root);
        assert!(packet.commands[0].children.is_empty());
    }

    #[test]
    fn test_single_command() {
        let world = World::new();
        world.component::<Structure>();
        let root = world.entity();

        world
            .entity()
            .set(Structure {
                data: NodeData::Literal {
                    name: "test".to_string(),
                },
            })
            .child_of_id(root);

        let packet = get_command_packet(&world);

        assert_eq!(packet.commands.len(), 2);
        assert_eq!(packet.root_index, VarInt(0));
        assert_eq!(packet.commands[0].children, vec![VarInt(1)]);
        assert_eq!(packet.commands[1].data, NodeData::Literal {
            name: "test".to_string()
        });
    }

    #[test]
    fn test_nested_commands() {
        let world = World::new();

        world.component::<Structure>();

        let root = world.entity();

        let parent = world
            .entity()
            .set(Structure {
                data: NodeData::Literal {
                    name: "parent".to_string(),
                },
            })
            .child_of_id(root);

        let _child = world
            .entity()
            .set(Structure {
                data: NodeData::Literal {
                    name: "child".to_string(),
                },
            })
            .child_of_id(parent);

        let packet = get_command_packet(&world);

        assert_eq!(packet.commands.len(), 3);
        assert_eq!(packet.root_index, VarInt(0));
        assert_eq!(packet.commands[0].children, vec![VarInt(1)]);
        assert_eq!(packet.commands[1].children, vec![VarInt(2)]);
        assert_eq!(packet.commands[1].data, NodeData::Literal {
            name: "parent".to_string()
        });
        assert_eq!(packet.commands[2].data, NodeData::Literal {
            name: "child".to_string()
        });
    }

    #[test]
    fn test_max_depth() {
        let world = World::new();
        world.component::<Structure>();

        let root = world.entity();

        let mut parent = root;
        for i in 0..=MAX_DEPTH {
            let child = world
                .entity()
                .set(Structure {
                    data: NodeData::Literal {
                        name: format!("command_{i}"),
                    },
                })
                .child_of_id(parent);
            parent = child;
        }

        let packet = get_command_packet(&world);

        assert_eq!(packet.commands.len(), MAX_DEPTH + 1);
    }
}
