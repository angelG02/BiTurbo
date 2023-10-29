use bevy_ecs::system::{Res, ResMut, Resource};
use bevy_ecs::world::World;
use std::{collections::VecDeque, fmt::Debug};

use crate::app::{CommandSchedule, GState, UpdateSchedule};
use crate::plugin::Plugin;
use turbo_core::trace::tracing::info;

type Task = Box<dyn FnOnce(&mut World) -> () + Send + Sync>;

pub struct CmdQueuePlugin {
    // Cmd options (serializable)
}

impl Plugin for CmdQueuePlugin {
    fn build(&self, app: &mut crate::app::App) {
        fn init(_ctx: &mut World) {
            info!("INITIALIZED CMD QUEUE");
        }

        let cmd_init = Command {
            command_type: CommandType::Close,
            args: None,
            task: Some(Box::new(init)),
        };
        let cmd_queue = CommandQueue::new(vec![cmd_init]);
        app.world.insert_resource(cmd_queue);
        app.add_systems(CommandSchedule, run_cli);
        app.add_systems(UpdateSchedule, execute_commands);
    }
}

//use bevy_reflect::Reflect;

//#[derive(Reflect)]
#[derive(PartialEq, Eq, Debug)]
pub enum CommandType {
    Exit,
    Help,
    Get,
    Put,
    Querry,
    Open,
    Close,
    Other,
}

// "assetserver get -from_server 127.0.0.1:7878 shaders/shader_challenge.vert"
// "window open NewWindow 1080 720" -> Creates a window and an empty hall and inserts them into Renderer
// "Gallery open NewGallery shader.vert shader.frag" -> Creates a gallery and sets it as the current gallery of the hall
//
// Command has:
// context: the API behind it? How would this be used
// type
// function
// args: Vec<String>

#[allow(dead_code)]
pub struct Command {
    pub command_type: CommandType,

    pub args: Option<String>,
    pub task: Option<Task>,
}

impl Command {
    pub fn new(command_type: CommandType, args: Option<String>, task: Box<Task>) -> Command {
        Command {
            command_type,
            args,
            task: Some(task),
        }
    }

    pub fn from_args(args: Vec<&str>) -> Command {
        match args[0] {
            "close" | "exit" => Command::exit(),
            _ => Command {
                command_type: CommandType::Other,
                args: Some(args[1..].join(" ")),
                task: None,
            },
        }
    }

    pub fn exit() -> Command {
        fn set_exit(world: &mut World) {
            world.get_resource_mut::<GState>().unwrap().running = false;
        }

        Command {
            command_type: CommandType::Close,
            args: None,
            task: Some(Box::new(set_exit)),
        }
    }
}

impl Debug for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Command")
            .field("Type", &self.command_type)
            .field("args", &self.args)
            .finish()
    }
}

pub trait IntoCommand {
    fn into_command(self) -> Command;
}

impl IntoCommand for Command {
    fn into_command(self) -> Command {
        self
    }
}

#[derive(Default, Resource)]
pub struct CommandQueue {
    commands: VecDeque<Command>,
}

impl CommandQueue {
    pub fn new(startup_commands: Vec<impl IntoCommand>) -> CommandQueue {
        let mut commands: Vec<Command> = Vec::with_capacity(startup_commands.len());

        for cmd in startup_commands {
            commands.push(cmd.into_command());
        }

        CommandQueue {
            commands: commands.into(),
        }
    }

    pub fn add_command(&mut self, command: impl IntoCommand) {
        self.commands.push_back(command.into_command());
    }
}

pub fn execute_commands(context: &mut World) {
    info!("Called!");

    let mut commands: VecDeque<Command> = vec![].into();
    {
        let mut cmd_queue = context.get_resource_mut::<CommandQueue>().unwrap();
        commands.reserve(cmd_queue.commands.len());

        commands = std::mem::take(&mut cmd_queue.commands);
    }

    for _ in 0..commands.len() {
        let command = commands.pop_front();
        if let Some(command) = command {
            if let Some(task) = command.task {
                info!("Called!");
                task(context);
            }
        }
    }

    {
        let mut cmd_queue = context.get_resource_mut::<CommandQueue>().unwrap();
        cmd_queue.commands.clear();
    }
}

pub fn get_cli_command() -> Command {
    let mut buffer = String::new();

    buffer.clear();

    info!("Please enter command! (type 'help' for list of commands)");
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Could not read provided command!");

    let command = buffer.trim_end().to_string();

    info!("Command: {}", command);

    let args: Vec<&str> = command.split(' ').collect();

    match args[0].to_ascii_lowercase().as_str() {
        // "assetserver" => match args[1].to_ascii_lowercase().as_str() {
        //     "get" => AssetCommand::new(CommandType::Get, args[2..].join(" ")).into_command(),
        //     _ => AssetCommand::new(CommandType::Other, args[2..].join(" ")).into_command(),
        // },
        // "window" => match args[1].to_ascii_lowercase().as_str() {
        //     "open" => WindowCommand::new(eventloop_proxy, CommandType::Open, args[2..].join(" "))
        //         .into_command(),
        //     _ => WindowCommand::new(eventloop_proxy, CommandType::Other, args[2..].join(" "))
        //         .into_command(),
        // },
        _ => Command::from_args(args),
    }
}

pub fn run_cli(global_state: Res<GState>, mut cmd_queue: ResMut<CommandQueue>) {
    let next_command = get_cli_command();
    info!("Command: {:?}", next_command);
    info!("Command count: {}", cmd_queue.commands.len());
    info!("Running?: {}", global_state.running);
    cmd_queue.add_command(next_command);
}
