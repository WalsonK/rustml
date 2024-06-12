use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Action {
    Rock,
    Paper,
    Scissors,
}

impl Action {
    fn beats(&self, other: Action) -> i32 {
        match (self, other) {
            (Action::Rock, Action::Scissors) | (Action::Scissors, Action::Paper) | (Action::Paper, Action::Rock) => 1,
            (Action::Scissors, Action::Rock) | (Action::Paper, Action::Scissors) | (Action::Rock, Action::Paper) => -1,
            _ => 0,
        }
    }
}

pub struct Agent;

impl Agent {
    fn new() -> Self {
        Agent
    }

    pub fn choose_action(&self) -> Action {
        // Here, you can implement any strategy for the agent
        let actions = [Action::Rock, Action::Paper, Action::Scissors];
        let random_index = rand::thread_rng().gen_range(0..3);
        actions[random_index]
    }
}


pub struct Adversary {
    first_action: Action,
}

impl Adversary {
    fn new() -> Self {
        Adversary { first_action: Action::Rock }
    }

    fn choose_action(&mut self, round: usize, agent_first_action: Action) -> Action {
        if round == 0 {
            // This is the first round. Play a random action and remember the agent's first action.
            let actions = [Action::Rock, Action::Paper, Action::Scissors];
            let action = actions[rand::thread_rng().gen_range(0..3)];
            self.first_action = agent_first_action;
            action
        } else {
            // This is the second round or beyond. Always play the agent's first action.
            self.first_action
        }
    }
}


pub struct Environment {
    pub(crate) agent: Agent,
    adversary: Adversary,
    round: usize,
    pub(crate) agent_score: i32,
    adversary_score: i32,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            agent: Agent::new(),
            adversary: Adversary::new(),
            round: 0,
            agent_score: 0,
            adversary_score: 0,
        }
    }

    pub fn step(&mut self, agent_action: Action) -> (i32, bool) {
        let adversary_action = self.adversary.choose_action(self.round, agent_action);
        println!("Adversary chose {:?}", adversary_action);
        let result = agent_action.beats(adversary_action);
        self.agent_score += result;

        self.round += 1;
        let done = self.round >= 2;
        (result, done)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_beats() {
        assert_eq!(Action::Rock.beats(Action::Scissors), 1);
        assert_eq!(Action::Scissors.beats(Action::Paper), 1);
        assert_eq!(Action::Paper.beats(Action::Rock), 1);
        assert_eq!(Action::Scissors.beats(Action::Rock), -1);
        assert_eq!(Action::Paper.beats(Action::Scissors), -1);
        assert_eq!(Action::Rock.beats(Action::Paper), -1);
        assert_eq!(Action::Rock.beats(Action::Rock), 0);
        assert_eq!(Action::Paper.beats(Action::Paper), 0);
        assert_eq!(Action::Scissors.beats(Action::Scissors), 0);
    }

    #[test]
    fn test_agent_choose_action() {
        let agent = Agent::new();
        let action = agent.choose_action();
        assert!(matches!(action, Action::Rock | Action::Paper | Action::Scissors));
    }

    #[test]
    fn test_adversary_choose_action() {
        let mut adversary = Adversary::new();
        let action = adversary.choose_action(0, Action::Rock);
        assert!(matches!(action, Action::Rock | Action::Paper | Action::Scissors));
        assert_eq!(adversary.choose_action(1, Action::Paper), Action::Rock);
    }

    #[test]
    fn test_environment_step() {
        let mut env = Environment::new();
        let agent_action_round_1 = env.agent.choose_action();
        println!("Agent chose {:?}", agent_action_round_1);
        let (result_round_1, done) = env.step(agent_action_round_1);
        assert_eq!(done, false);
        env.adversary.first_action = agent_action_round_1;
        let agent_action_round_2 = env.agent.choose_action();
        let (result_round_2, done) = env.step(agent_action_round_2);
        println!("Result: {}, Done: {}", result_round_2, done);
        assert_eq!(done, true);
    }
}



/*fn main() {
    let mut env = Environment::new();
    let agent_action_round_1 = env.agent.choose_action();
    let (result_round_1, _) = env.step(agent_action_round_1);
    println!("Round 1: Agent chose {:?}, result: {}", agent_action_round_1, result_round_1);

    let agent_action_round_2 = env.agent.choose_action(); // Agent choisit une action aléatoire pour le deuxième round
    let (result_round_2, done) = env.step(agent_action_round_2); // Passer l'action du deuxième round
    println!("Round 2: Agent chose {:?}, result: {}", agent_action_round_2, result_round_2); // Agent joue la même action que le deuxième round

    if done {
        println!("Game over. Total score: {}", env.agent_score);
    }
}*/


