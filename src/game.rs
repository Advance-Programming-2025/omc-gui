use omc_galaxy::Orchestrator;
use bevy::prelude::*;

#[derive(Resource)]
pub struct OrchestratorResource {
    pub orchestrator: Orchestrator,
}

#[derive(Clone, Default)]
pub struct GalaxySnapshot {
    pub edges: Vec<(u32, u32)>,
    pub planet_num: usize,
    pub planet_states: Vec<(usize, omc_galaxy::PlanetStatus)>
}

// Shared game snapshot object
#[derive(Resource, Default)]
pub struct GameSnapshot {
    pub snapshot: GalaxySnapshot,
}

#[derive(Resource, Deref, DerefMut)]
pub struct GameTimer(pub Timer);

#[derive(Message, Debug)]
pub enum GameEvent {
    StartGame,
    StopGame,
    ResetGame,
    EndGame,
}

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    WaitingStart,
    Running,
    Paused,
}

#[derive(Component, Deref, DerefMut)]
pub struct Game {
    state: GameState
}

pub fn setup_orchestrator(
    mut commands: Commands,
) {
    dotenv::dotenv().ok();

    let mut orchestrator = Orchestrator::new()
        .expect("Failed to create orchestrator");

    let file_path = std::env::var("INPUT_FILE")
        .expect("Set INPUT_FILE in .env or env vars");

    orchestrator
        .initialize_galaxy_by_file(file_path.as_str().trim())
        .expect("Failed to initialize galaxy");

    let (topology,planet_num) = 
        orchestrator.get_topology();

    commands.insert_resource(OrchestratorResource {
        orchestrator,
    });

    commands.insert_resource(GameState::WaitingStart);

    commands.insert_resource(GameSnapshot{
        snapshot:
            GalaxySnapshot{
                edges:topology,
                planet_num,
                ..default()
    }});

    commands.insert_resource(GameTimer(Timer::from_seconds(1.0, TimerMode::Once)));
}

pub fn snapshot_update(
    orchestrator_res: ResMut<OrchestratorResource>,
    mut snapshot_res: ResMut<GameSnapshot>,
) {
    let topology = orchestrator_res.orchestrator.get_topology();
    snapshot_res.as_mut().snapshot.edges = topology.0;
    snapshot_res.as_mut().snapshot.planet_num = topology.1;
}

pub fn game_loop(
    mut game_state: ResMut<GameState>,
    mut events: MessageReader<GameEvent>,
    mut orchestrator: ResMut<OrchestratorResource>,
    mut timer: ResMut<GameTimer>,
    time: Res<Time>,
) {
        timer.tick(time.delta());

    for event in events.read() {
        match (*game_state, event) {
            (_, GameEvent::EndGame) => {
                info!("Ending game");
                let _ = orchestrator.orchestrator.stop_all();
            }

            (GameState::WaitingStart, GameEvent::StartGame) => {
                info!("Starting game");
                *game_state = GameState::Running;
                timer.reset();
                let _ = orchestrator.orchestrator.start_all();
            }

            (GameState::Running, GameEvent::StopGame) => {
                info!("Pausing game");
                *game_state = GameState::Paused;
            }

            (_, GameEvent::ResetGame) => {
                info!("Reset game requested");
            }

            _ => {}
        }
    }
}
