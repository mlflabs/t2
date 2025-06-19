use std::fmt::Debug;
use bevy_turborand::prelude::*;

use bevy::prelude::*;

use crate::utils::DebugPrint;

use super::thinkboard::ThinkBoard;
use super::thinker::{Thinker, ThinkerExecutingTag, ThinkerStage, ActionStage};

pub fn score_management_system(
    mut cmd: Commands,
    mut global_rng: ResMut<GlobalRng>,
    time: Res<Time>,
    //map: Res<ActionScorerMap>,
    //actions_resource: Res<ActionsResource>,
    mut query: Query<(
        Entity,
        &mut Thinker,
        &ThinkBoard,
        Option<&ThinkerExecutingTag>,
        Option<&DebugPrint>,
    )>,
) {

    let mut rng = global_rng.as_mut();

    for (e, mut thinker, board, executing, debug) in query.iter_mut() {
        // Are we executing or picking

        let print = if let Some(debug) = debug {
            true
        } else {
            false
        };

        if let Some(_executing) = executing {
            if print {
                println!("Thinker is executing an action, stage: {:?}", thinker.stage);
            }

            match thinker.stage {
                ThinkerStage::LoadAction => {
                    let current_state_index = thinker.state_index;
                    let state = &mut thinker.states[current_state_index];

                    

                    if state.set_next_action_cloned() {
                        if let Some(action_to_load) = state.get_current_action_mut() {
                            action_to_load.on_enter(&mut cmd, &board, print);
                            thinker.stage = ThinkerStage::RunningAction;
                        } else {}
                    }
                    else {
                            if print {
                                println!("Error: Action was set but could not be retrieved mutably.");
                            }
                            thinker.stage = ThinkerStage::CleaningUp;
                    }
                }

                ThinkerStage::RunningAction => {

                    match thinker.action_stage {
                        ActionStage::Running => {
                            // If the action is running, we update it
                            let current_state_index = thinker.state_index;
                            let state = &mut thinker.states[current_state_index];

                            let mut next_thinker_action_stage: ActionStage = ActionStage::Running;

                            if let Some(action) = state.get_current_action_mut() {
                                
                                let stage = action.get_stage();
                                next_thinker_action_stage = action.on_update(
                                    time.delta_secs(), &mut cmd, &board, print);

                                if next_thinker_action_stage != ActionStage::Running {
                                    match stage {
                                        ActionStage::Finished => {
                                            next_thinker_action_stage = ActionStage::Finished;
                                        }
                                        ActionStage::Failed => {
                                            next_thinker_action_stage = ActionStage::Failed;
                                        }
                                        _ => {}
                                    }
                                }
                            } else {
                                thinker.stage = ThinkerStage::ActionCleanup;
                            }

                            if next_thinker_action_stage != ActionStage::Running {
                                thinker.action_stage = next_thinker_action_stage;
                            }
                        }
                        ActionStage::Finished => {
                            // If the action is finished, we clean it up
                            thinker.stage = ThinkerStage::ActionCleanup;
                        }
                        ActionStage::Failed => {
                            // If the action failed, we also clean it up
                            thinker.stage = ThinkerStage::ActionCleanup;
                        }
                        _ => {
                            // Handle other action stages if necessary
                        }
                    }
                }

                ThinkerStage::ActionCleanup => {
                    let current_state_index = thinker.state_index;
                    let state = &mut thinker.states[current_state_index];

                    if let Some(action_to_cleanup) = state.get_current_action_mut() {
                        action_to_cleanup.on_exit(&mut cmd, &board, print);
                        
                        if print {
                            println!(
                                "Thinker cleaned up action: {:?}",
                                action_to_cleanup.as_debug()
                            );
                        }


                        // Transition to the next stage
                        thinker.stage = ThinkerStage::LoadAction;
                        thinker.action_stage = ActionStage::Running;
                    } else {
                        if print {
                            println!("Error: Action was set but could not be retrieved mutably.");
                        }
                        thinker.stage = ThinkerStage::CleaningUp;
                    }
                }

                ThinkerStage::CleaningUp => {
                    cmd.entity(e).remove::<ThinkerExecutingTag>();
                }

                _ => {
                    // If we are in a stage other than LoadAction, we should not be executing
                    if print {
                        println!("Thinker is in an unexpected stage: {:?}", thinker.stage);
                    }
                }
            }
        } else {
            if print {      
                println!("Thinker is not executing an action, stage: {:?}", thinker.stage);
            }

            match thinker.stage {
                ThinkerStage::Init => {
                    thinker.previous_state2 = thinker.previous_state;
                    thinker.previous_state = thinker.state_index;
                    thinker.stage = ThinkerStage::Evaluating;
                }
                ThinkerStage::Evaluating => {
                    // Evaluate the scorers
                    let mut max_value = f32::MIN;
                    let mut max_index: usize = 0;
                    let mut index: usize = 0;
                    for state in &mut thinker.states {
                        let mut score: f32 = 1.0;

                        for scorer in &state.scorers {
                            if print {
                                println!("*****Scorer: {:?}", scorer);
                            }
                            score *= scorer.score(&board, &mut rng);
                        }
                        if score > max_value {
                            max_value = score;
                            max_index = index;
                        }
                        index += 1;
                    }
                    thinker.state_index = max_index;
                    thinker.state_value = max_value;
                    thinker.stage = ThinkerStage::PrepareState;
                    if print {
                        println!("Thinker Evaluated: {:?}, value: {}", thinker.state_index, thinker.state_value );
                    }
                }
                ThinkerStage::PrepareState => {
                    let current_state_index = thinker.state_index;
                    let state = &mut thinker.states[current_state_index];

                    state.reset();



                    thinker.stage = ThinkerStage::LoadAction;
                    cmd.entity(e).insert(ThinkerExecutingTag);
                }
                ThinkerStage::CleaningUp => {
                    thinker.stage = ThinkerStage::Finished;
                }
                ThinkerStage::Finished => {
                    thinker.stage = ThinkerStage::Init;
                }

                _ => {
                    thinker.stage = ThinkerStage::Finished;
                }
            }
        }
    }
}