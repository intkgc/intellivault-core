use std::mem;

use chatgpt::{
    client::ChatGPT,
    err::Error,
    types::{ChatMessage, CompletionResponse, Role},
};

use crate::intoresult::IntoResult;

#[derive(PartialEq)]
enum ActionType {
    Analyze,
    StandardReply,
    SpecialAction,
    Unknown(String),
}

#[derive(Debug)]
pub enum ConverationError {
    ChatGPTError(Error),
    UnkownAction(String),
}
pub struct Conversation {
    messages: Vec<ChatMessage>,
    client: ChatGPT,
}

impl Conversation {
    pub fn new(client: ChatGPT) -> Self {
        Self {
            messages: Default::default(),
            client,
        }
    }

    async fn determine_actions_types(
        &mut self,
        message: &String,
    ) -> Result<Vec<ActionType>, Error> {
        let prompt = "You are a decision-making assistant in a knowledge base. Based on the user's query, decide which of the following actions is most appropriate. Reply with the action's key only (e.g., 'retrieve_notes_and_analyze', or 'standard_reply', 'do_something'). You can write few variants by writing separated by commas. If assistant don't know user information stored in database and needed for reply choose 'retrieve_notes_and_analyze'";

        let history = vec![
            ChatMessage {
                role: Role::System,
                content: prompt.to_string(),
            },
            ChatMessage {
                role: Role::User,
                content: message.to_string(),
            },
        ];

        let answer = mem::take(
            &mut self
                .client
                .send_history(&history)
                .await?
                .message_choices
                .first_mut()
                .unwrap()
                .message
                .content,
        );

        let mut decisions: Vec<_> = answer
            .split(",")
            .map(|it| match it.trim() {
                "retrieve_notes_and_analyze" => ActionType::Analyze,
                "standard_reply" => ActionType::StandardReply,
                "do_something" => ActionType::SpecialAction,
                _ => ActionType::Unknown(it.trim().into()),
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

    async fn analize(&mut self, message: &String) {}
    
    async fn send_user_message(&mut self, message: &String) -> Result<String, ConverationError> {
        self.messages.push(ChatMessage {
            role: Role::User,
            content: message.clone(),
        });    
        
        let reply = self
            .client
            .send_history(&self.messages)
            .await
            .map_err(|err| ConverationError::ChatGPTError(err))?
            .message()
            .content
            .clone();
        
        self.messages.push(ChatMessage {
            role: Role::Assistant,
            content: reply.clone()
        });
        
        reply.into_ok()
    }

    pub async fn send_message<S: Into<String>>(
        &mut self,
        message: S,
    ) -> Result<String, ConverationError> {
        let message = message.into();
        let actions = self
            .determine_actions_types(&message)
            .await
            .map_err(|err| ConverationError::ChatGPTError(err))?;

        for i in actions.into_iter() {
            match i {
                ActionType::Analyze => println!("analize"),
                ActionType::StandardReply => {
                    return self.send_user_message(&message).await
                }
                ActionType::SpecialAction => println!("action"),
                ActionType::Unknown(it) => Err(ConverationError::UnkownAction(it))?,
            }
        }

        Ok(String::default())
    }
}
