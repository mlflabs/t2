use std::fmt::Debug;

use bevy::prelude::*;

use crate::utils::DebugPrint;

use super::thinkboard::ThinkBoard;
use super::thinker::{Thinker, ThinkerExecutingTag, ThinkerStage, ActionStage};

pub fn score_management_system(
    mut cmd: Commands,
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
    for (e, mut thinker, board, executing, debug) in query.iter_mut() {
        // Are we executing or picking
        if let Some(_executing) = executing {
            println!("Thinker is executing an action, stage: {:?}", thinker.stage);

            match thinker.stage {
                ThinkerStage::LoadAction => {
                    let current_state_index = thinker.state_index;
                    let state = &mut thinker.states[current_state_index];

                    

                    if state.set_next_action_cloned() {
                        if let Some(action_to_load) = state.get_current_action_mut() {
                            action_to_load.on_enter(&mut cmd);
                            // Print the action (using as_debug for proper trait object debug)
                            println!("Thinker loaded action: {:?}", action_to_load.as_debug());
                            // Transition to the next stage
                            thinker.stage = ThinkerStage::RunningAction;
                        } else {}
                    }
                    else {
                            println!("Error: Action was set but could not be retrieved mutably.");
                            thinker.stage = ThinkerStage::CleaningUp;
                    }
                }

                ThinkerStage::RunningAction => {

                    match thinker.action_stage {
                        ActionStage::Running => {
                            // If the action is running, we update it
                            let current_state_index = thinker.state_index;
                            let state = &mut thinker.states[current_state_index];

                            let mut next_thinker_action_stage: Option<ActionStage> = None;

                            if let Some(action) = state.get_current_action_mut() {
                                
                                let stage = action.get_stage();

                                match stage {
                                    ActionStage::Finished => {
                                        next_thinker_action_stage = Some(ActionStage::Finished);
                                    }
                                    ActionStage::Failed => {
                                        next_thinker_action_stage = Some(ActionStage::Failed);
                                    }
                                    _ => {}
                                }
                                action.on_update(&mut cmd);
                            } else {
                                thinker.stage = ThinkerStage::ActionCleanup;
                            }

                            if let Some(stage) = next_thinker_action_stage {
                                thinker.action_stage = stage; // This is now safe
                                // Also transition the main thinker stage based on the action's result
                                thinker.stage = ThinkerStage::ActionCleanup;
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
                        action_to_cleanup.on_exit(&mut cmd);
                        // Print the action (using as_debug for proper trait object debug)
                        println!(
                            "Thinker cleaned up action: {:?}",
                            action_to_cleanup.as_debug()
                        );
                        // Transition to the next stage
                        thinker.stage = ThinkerStage::LoadAction;
                        thinker.action_stage = ActionStage::Running;
                    } else {
                        // This case should ideally not be hit if set_next_action_cloned returns true
                        // but it's good defensive programming.
                        println!("Error: Action was set but could not be retrieved mutably.");
                        thinker.stage = ThinkerStage::CleaningUp;
                    }
                }

                ThinkerStage::CleaningUp => {
                    cmd.entity(e).remove::<ThinkerExecutingTag>();
                }

                _ => {
                    // If we are in a stage other than LoadAction, we should not be executing
                    println!("Thinker is in an unexpected stage: {:?}", thinker.stage);
                }
            }
        } else {
            println!(
                "Thinker is not executing an action, stage: {:?}",
                thinker.stage
            );

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
                        let mut score: f32 = 0.0;

                        for scorer in &state.scorers {
                            score *= scorer.score(&board);
                        }
                        if score > max_value {
                            max_value = score;
                            max_index = index;
                        }
                        index += 1;
                    }
                    thinker.state_index = max_index;
                    thinker.state_value = max_value;
                    thinker.stage = ThinkerStage::AssigningAction;
                    println!(
                        "Thinker Evaluated: {:?}, value: {}",
                        thinker.state_index, thinker.state_value
                    );
                }
                ThinkerStage::AssigningAction => {
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

/*
for (e, mut score, mut action,thinker) in scorers.iter_mut() {
    //Are we executing or picking
    // if let i = &map {
    //    println!("res:::::::::::::::::{:?}", i.map);
    // }

    if let Some(_thinker) = thinker {
        match action.state {
            ActionState::Init => {
                println!("thinker Action, Initial");
                //ActionState::Running
                action.state = ActionState::Running;
            },
            ActionState::Running => {
                //ActionState::Cleanup
            },
            ActionState::Cleanup => {
                println!("AcitonState::Cleanup");
                let i = map.map.get(&score.scorer).unwrap();

                //let mut index = 0;
                let mut idx = 0;

                for id in i.iter(){
                    println!("1, {:?}, {:?}", id, &action.action);
                    println!("Score: {:?}", &score);
                    println!("Action: {:?}", &action);
                    if id == &action.action {
                        println!("2");
                        if idx == i.len() - 1 {
                            println!("3");
                            //last record
                            cmd.entity(e).remove::<ThinkerExecutingActionTag>();
                            let c = ComponentId::new(*id);
                            cmd.entity(e).remove_by_id(c);
                        }
                        else {
                            println!("4");
                            //more action to go
                            let c = ComponentId::new(action.action);
                            cmd.entity(e).remove_by_id(c);

                            let cc = ComponentId::new(i[idx + 1]);
                            //cmd.entity(e).insert_by_id(cc, {});
                            //unsafe { cmd.entity(e).insert_by_id(cc, {}) };

                            //let ttt = RestAction::default();
                            //let sdf = cmd.(ttt);

                        }
                    }
                    else {
                        println!("5: {:?}", idx);
                        idx += 1;
                    }
                }

                //let t = map.map.get(&score.scorer);





                //ActionState::Init
            },
            _ => {

            }
            //ActionState::Finished => ActionState::Init
        };
    }
    else {
        score.step = match score.step {
            ScorerStep::Init => {
                //println!("Thinker INIT");
                score.previous_winner2 = score.previous_winner.clone();
                score.previous_winner = score.scorer.clone();
                score.value = 0.;
                score.scorer = usize::default();

                ScorerStep::Evaluating
            },
            ScorerStep::Evaluating => ScorerStep::AssigningAction,
            ScorerStep::AssigningAction => {
                println!("AssigningAction");
                println!("Score: {:?}", score);
                let i = map.map.get(&score.scorer);
                if let Some(v)  = i {
                    let cc = ComponentId::new(v[0]);
                    unsafe { cmd.entity(e).insert_by_id(cc, {}) };
                    action.action = v[0];

                    ScorerStep::CleaningUp
                }
                else {
                    println!("Scorer doesn't point to any action");
                    //nothing found, lets go back and try again
                    ScorerStep::Init
                }
            }
            ScorerStep::CleaningUp => {
                println!("Thinker cleanup");
                cmd.entity(e).insert(ThinkerExecutingActionTag);
                ScorerStep::Finished
            },
            ScorerStep::Finished => ScorerStep::Init

        }
    }



}

 */
