//! Macroscopic metrics on world state.




impl State for Realizing {
    fn type(&self):
        return type(current_state(init(game_spec)))

    fn shape(&self) -> Vec<u32>:
        state = current_state(init(game_spec))
        return size(vectorize_state(game_spec, state))

    fn complexity(&self) -> u64:
        state = current_state(init(game_spec))
        return Base.summarysize(state)
}
