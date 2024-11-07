use std::ops::{Range, RangeFrom, RangeFull};

use flecs_ecs::core::{Entity, IdOperations, World};
use rustc_hash::FxHashMap;
use valence_protocol::packets::play::command_tree_s2c::{Parser as ValenceParser, StringArg};

pub enum Parser {
    String,
    Integer { min: Option<i32>, max: Option<i32> },
}

impl From<Range<i32>> for Parser {
    fn from(value: Range<i32>) -> Self {
        Self::Integer {
            min: Some(value.start),
            max: Some(value.end),
        }
    }
}

impl From<RangeFrom<i32>> for Parser {
    fn from(value: RangeFrom<i32>) -> Self {
        Self::Integer {
            min: Some(value.start),
            max: None,
        }
    }
}

impl From<Parser> for ValenceParser {
    fn from(value: Parser) -> Self {
        match value {
            Parser::String => Self::String(StringArg::SingleWord),
            Parser::Integer { min, max } => Self::Integer { min, max },
        }
    }
}

use crate::{CommandContext, Executor, ExecutorFn, Structure, ROOT_COMMAND};

#[allow(clippy::must_use_candidate)]
pub fn add_command(world: &World, command: Structure, parent: Entity) -> Entity {
    world.entity().set(command).child_of_id(parent).id()
}

pub fn add_executor(world: &World, executor: ExecutorFn, parent: Entity) -> Entity {
    world
        .entity()
        .set(Executor { executor })
        .child_of_id(parent)
        .id()
}

/// Entry point for defining commands using the DSL.
pub fn cmd_with<'a, F>(world: &'a World, name: &str, f: F)
where
    F: FnOnce(&mut CommandScope<'a>),
{
    let mut scope = CommandScope::new(world);
    scope.literal_with(name, f);
}

pub fn cmd(world: &World, name: &str) {
    cmd_with(world, name, |_| {});
}

#[must_use]
pub struct CommandScope<'a> {
    world: &'a World,
    current: Entity,
    parents: FxHashMap<Entity, Entity>,
}

impl<'a> CommandScope<'a> {
    fn new(world: &'a World) -> Self {
        Self {
            world,
            current: *ROOT_COMMAND.get().expect("Root command not initialized"),
            parents: FxHashMap::default(),
        }
    }

    /// Adds a literal command. Accepts an optional closure to define nested commands.
    pub fn literal_with<F>(&mut self, name: &str, f: F) -> &mut Self
    where
        F: FnOnce(&mut Self),
    {
        let command = Structure::literal(name);
        let entity = add_command(self.world, command, self.current);
        self.parents.insert(entity, self.current);
        self.current = entity;

        // Execute the closure to define nested commands
        f(self);

        // Return to the parent command
        self.end()
    }

    pub fn literal(&mut self, name: &str) -> &mut Self {
        self.literal_with(name, |_| {})
    }

    pub fn argument(&mut self, name: &str, parser: impl Into<Parser>) -> &mut Self {
        self.argument_with(name, parser, |_| {})
    }

    /// Adds an argument command. Accepts an optional closure to define nested commands.
    pub fn argument_with(
        &mut self,
        name: &str,
        parser: impl Into<Parser>,
        f: impl FnOnce(&mut Self),
    ) -> &mut Self {
        let parser = parser.into();
        let parser = ValenceParser::from(parser);
        let command = Structure::argument(name, parser);
        let entity = add_command(self.world, command, self.current);
        self.parents.insert(entity, self.current);
        self.current = entity;

        // Execute the closure to define nested commands
        f(self);

        // Return to the parent command
        self.end()
    }

    /// Adds an executor function to the current command.
    pub fn executor(&self, executor: ExecutorFn) {
        add_executor(self.world, executor, self.current);
    }

    /// Ends the current command scope, returning to the parent command.
    pub fn end(&mut self) -> &mut Self {
        if let Some(parent) = self.parents.get(&self.current).copied() {
            self.current = parent;
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use flecs_ecs::core::{EntityViewGet, World};
    use valence_protocol::packets::play::command_tree_s2c::NodeData;

    use super::*;
    use crate::{dsl, get_root_command, HyperionCommandModule};

    #[test]
    fn test_add_command() {
        let world = World::new();

        world.import::<HyperionCommandModule>();

        cmd_with(&world, "test", |scope| {
            println!("called!!!!!");
            scope.literal("example").executor(|world, entity, context| {
                println!("Executing example command");
                Ok(())
            });
        });

        // Verify the command was added
        let root_command = get_root_command().entity_view(&world);
    }
}
