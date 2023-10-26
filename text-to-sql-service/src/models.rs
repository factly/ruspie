use serde::{Deserialize, Serialize};

// #[derive(Debug, Serialize, Deserialize)]
// pub struct OpenAIRequest {
//     model: String,
//     pub prompt: String,
//     temperature: u64,
//     max_tokens: u64,
//     stop: Vec<String>,
// }

// impl From<String> for OpenAIRequest {
//     fn from(prompt: String) -> Self {
//         let stop: Vec<String> = vec!["#".to_owned(), ";".to_owned()];
//         Self {
//             model: String::from("text-davinci-003"),
//             prompt,
//             temperature: 0,
//             max_tokens: 150,
//             stop,
//         }
//     }
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIRequest {
    model: String,
    messages: Vec<Message>,
}

impl From<String> for OpenAIRequest {
    fn from(content: String) -> Self {
        let messages = vec![Message {
            role: "user".to_string(),
            content,
        }];
        OpenAIRequest {
            model: "gpt-3.5-turbo".to_string(),
            messages,
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIResponse {
    pub choices: Vec<Choice>,
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Choice {
//     pub text: String,
//     pub index: u64,
//     pub finish_reason: String,
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct Choice {
    pub index: u64,
    pub message: Message,
}
