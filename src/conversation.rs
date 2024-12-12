use std::mem;

use chatgpt::{
    client::ChatGPT,
    err::Error,
    types::{ChatMessage, CompletionResponse},
};

#[derive(PartialEq)]
enum ActionType {
    Analyze,
    StandardReply,
    SpecialAction,
    Unknown,
}

enum ConverationError {
    ChatGPTError(Error),
    UnkownAction,
}
struct Conversation {
    messages: Vec<ChatMessage>,
    client: ChatGPT,
}

impl Conversation {
    async fn determine_actions_types(
        &mut self,
        message: &String,
    ) -> Result<Vec<ActionType>, Error> {
        let prompt = "You are a decision-making assistant in a knowledge base. Based on the user's query, decide which of the following actions is most appropriate. Reply with the action's key only (e.g., 'retrieve_notes_and_analyze', or 'standard_reply', 'do_special_action'). You can write few variants by writing separated by commas";

        let answer = mem::take(
            &mut self
                .client
                .send_message(prompt)
                .await?
                .message_choices
                .first_mut()
                .unwrap()
                .message
                .content,
        );

        let mut decisions: Vec<_> = answer
            .split(",")
            .map(|it| match it {
                "retrieve_notes_and_analyze" => ActionType::Analyze,
                "standard_reply" => ActionType::StandardReply,
                "do_special_action" => ActionType::SpecialAction,
                _ => ActionType::Unknown,
            })
            .collect();

        if decisions.contains(&ActionType::StandardReply)
            && decisions.contains(&ActionType::Analyze)
        {
            let index = decisions
                .iter()
                .enumerate()
                .find(|it| *it.1 == ActionType::StandardReply)
                .unwrap()
                .0;
            decisions.remove(index);
        }

        Ok(decisions)
    }
    
    pub async fn analize(&mut self, message: &String) {
        
    }
    
    pub async fn send_message<S: Into<String>>(
        &mut self,
        message: S,
    ) -> Result<(), ConverationError> {
        let message = message.into();
        let actions = self
            .determine_actions_types(&message)
            .await
            .map_err(|err| ConverationError::ChatGPTError(err))?;

        for i in actions.iter() {
            match i {
                ActionType::Analyze => todo!(),
                ActionType::StandardReply => todo!(),
                ActionType::SpecialAction => todo!(),
                ActionType::Unknown => Err(ConverationError::UnkownAction)?,
            }
        }

        Ok(())
    }
}
