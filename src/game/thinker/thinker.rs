use bevy::prelude::*;

use super::thinkerboard::ThinkerBoard;


#[derive(Component, Reflect, Default, Debug)]
pub struct ThinkerTag;


#[derive(Component, Reflect, Default, Debug)]
pub struct ThinkerExecutingActionTag;





#[derive(Debug, PartialEq, Eq, Reflect)]
pub enum ScoreStep {
    Init, Evaluating, AssigningAction, CleaningUp, Finished
}

impl Default for ScoreStep {
    fn default() -> Self {
        Self::Evaluating
    }
}

#[derive(Component, Debug, Default, Reflect)]
pub struct Score {
    pub value: f32,
    pub step: ScoreStep,
    pub scorer: usize,
    pub previous_winner: usize,
    pub previous_winner2: usize
}





#[derive(Default, Reflect, Clone, Debug)]
#[reflect(Default)]
pub enum ScorerComparisons {
    #[default]
    Static,
    Contains,
    Bigger,
    Smaller,
    Equals
}

#[derive(Default, Reflect, Clone)]
pub struct Scorer {
    pub scorer_type: usize,
    pub comparison: ScorerComparisons,
    pub value: f32,
    pub score: f32,
    pub state: usize,
}


impl Scorer {
    pub fn new( scorer_type: usize, 
                comparison: ScorerComparisons, 
                value: f32, 
                score: f32, 
                state: usize) -> Self {
        Scorer {
            scorer_type,
            comparison,
            value,
            score,
            state,
        }
    }

    pub fn calculate(&self, tb:&ThinkerBoard) -> f32 {
        match self.comparison {

            ScorerComparisons::Contains => {
                self.score
            },
            ScorerComparisons::Bigger => {
                self.score
            },
            ScorerComparisons::Smaller => {
                self.score
            },
            ScorerComparisons::Equals => {

                self.score
            },
            _ => {
                self.score
            }
        }
    }
}


#[derive(Default, Reflect)]
pub struct State {
    actions: Vec<usize>,
}





#[derive(Component, Clone)]
pub struct Scorers {
    pub scorers: Vec<Scorer>,
   
}

impl Default for Scorers {
    fn default() -> Self {
        Scorers {
            scorers: Vec::new(), // Initializes an empty vector of Scorers
        }
    }
}

#[derive(Component, Clone)]
pub struct States {
    pub scorers: Vec<States>,
   
}







#[derive(Debug, PartialEq, Eq, Reflect)]
pub enum ScorerStep {
    Init, Evaluating, AssigningAction, CleaningUp, Finished
}

impl Default for ScorerStep {
    fn default() -> Self {
        Self::Evaluating
    }
}




pub fn score_management_system(
    mut cmd: Commands,
    //map: Res<ActionScorerMap>,
    mut query: Query<(Entity, 
                        &mut Score, 
                        &mut Scorers, 
                        &ThinkerBoard,
                        Option<&ThinkerExecutingActionTag>, 
                        )>,
){

    for(e, mut score, mut scorers, board, executing) in query.iter_mut() {
        // Are we executing or picking
        if let Some(_executing) = executing {
            println!("Thinker is executing an action");
         

        } else {
            println!("Thinker is not executing an action");
            
            match score.step {
                ScoreStep::Init => {
                    println!("Thinker INIT");
                    score.previous_winner2 = score.previous_winner;
                    score.previous_winner = score.scorer;
                    score.value = 0.;
                    score.scorer = usize::default();
                    score.step = ScoreStep::Evaluating;
                },
                ScoreStep::Evaluating => {
                    // Evaluate the scorers
                    let mut max_value = f32::MIN;
                    let mut max_index = -1;

                    for scorer in &mut scorers.scorers {
                        let s = scorer.calculate(&board);
                    }
                    score.step = ScoreStep::AssigningAction;
                },
                ScoreStep::AssigningAction => {
                    
                },
                ScoreStep::CleaningUp => {
                    
                }
                ScoreStep::Finished => {
                    
                },
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
}









































