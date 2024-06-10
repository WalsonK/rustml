extern crate rand;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use std::collections::HashMap;
use crate::environments::line_world::Environment; //

pub type State = i64;
pub type Action = i64;
pub type Reward = f64;

#[derive(Clone, Debug)]
pub struct EpisodeStep {
    pub state: State,
    pub action: Action,
    pub reward: Reward,
}

pub struct MonteCarloESModel {
    pub num_episodes: usize,
    pub gamma: f64,
    pub max_steps: usize,
    pub policy: HashMap<State, Action>,
    pub q_values: HashMap<(State, Action), Reward>,
    pub returns: HashMap<(State, Action), Vec<Reward>>,
}

impl MonteCarloESModel {
    pub fn new(num_episodes: usize, gamma: f64, max_steps: usize) -> Box<MonteCarloESModel> {
        Box::new(MonteCarloESModel {
            num_episodes,
            gamma,
            max_steps,
            policy: HashMap::new(),
            q_values: HashMap::new(),
            returns: HashMap::new(),
        })
    }

    pub fn monte_carlo_es<E: Environment>(&mut self, env: &mut E) {
        let mut rng = thread_rng();

        for episode in 0..self.num_episodes {
            let state = *env.all_states().choose(&mut rng).expect("No states available");
            env.set_state(state);
            let mut available_actions = env.available_actions();
            if available_actions.is_empty() {
                println!("No actions available for state {}", state);
                continue;
            }
            let mut action = *available_actions.choose(&mut rng).expect("No actions available");

            let mut episode_steps: Vec<EpisodeStep> = Vec::new();
            let mut steps = 0;
            let mut done = false;

            while !done && steps < self.max_steps {
                let (next_state, reward, is_done) = env.step(action);
                episode_steps.push(EpisodeStep {
                    state,
                    action,
                    reward,
                });

                env.set_state(next_state);
                steps += 1;
                done = is_done;

                // Ajout de débogage pour voir l'état et l'action à chaque étape
                println!("Episode {}, Step {}: State {}, Action {}, Reward {}", episode, steps, state, action, reward);

                // Affichage de l'état de l'environnement après chaque action
                env.display();

                available_actions = env.available_actions();
                if available_actions.is_empty() {
                    println!("No actions available for state {}", next_state);
                    break;
                }
                action = *available_actions.choose(&mut rng).expect("No actions available");
            }

            self.process_episode(episode_steps);
        }
    }

    fn process_episode(&mut self, episode: Vec<EpisodeStep>) {
        let mut g: Reward = 0.0;
        let mut visited_state_action_pairs: Vec<(State, Action)> = Vec::new();

        for step in episode.iter().rev() {
            g = self.gamma * g + step.reward;
            let state_action_pair = (step.state, step.action);

            if !visited_state_action_pairs.contains(&state_action_pair) {
                visited_state_action_pairs.push(state_action_pair);
                self.returns.entry(state_action_pair)
                    .or_insert_with(Vec::new)
                    .push(g);

                let return_list = self.returns.get(&state_action_pair).unwrap();
                let mean_return = return_list.iter().copied().sum::<Reward>() / return_list.len() as Reward;
                self.q_values.insert(state_action_pair, mean_return);

                let best_action = self.find_best_action(step.state);
                self.policy.insert(step.state, best_action);
            }
        }
    }

    fn find_best_action(&self, state: State) -> Action {
        let mut best_action = 0;
        let mut best_value = f64::NEG_INFINITY;

        for action in self.q_values.keys().filter_map(|&(s, a)| if s == state { Some(a) } else { None }) {
            let value = *self.q_values.get(&(state, action)).unwrap_or(&f64::NEG_INFINITY);
            if value > best_value {
                best_value = value;
                best_action = action;
            }
        }

        best_action
    }
}
