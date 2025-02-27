use crate::dialogue::{states::receive_age::ReceiveAgeState, Dialogue};
use teloxide::prelude::*;
use teloxide_macros::teloxide;

#[derive(Generic)]
pub struct ReceiveFullNameState;

#[teloxide(subtransition)]
async fn receive_full_name(
    state: ReceiveFullNameState,
    cx: TransitionIn,
    ans: String,
) -> TransitionOut<Dialogue> {
    cx.answer_str("How old are you?").await?;
    next(ReceiveAgeState::up(state, ans))
}
