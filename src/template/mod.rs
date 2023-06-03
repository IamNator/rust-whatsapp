use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error;
use std::str::FromStr;
use std::string::ToString;

// https://developers.facebook.com/docs/whatsapp/cloud-api/reference/messages/#template-messages

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParameterType {
    Text,
    Image,
    ButtonPayload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Parameter {
    #[serde(rename = "type")]
    pub parameter_type: ParameterType,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageLink {
    pub link: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageParameter {
    #[serde(rename = "type")]
    pub parameter_type: ParameterType,
    pub image: ImageLink,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ButtonPayloadParameter {
    #[serde(rename = "type")]
    pub parameter_type: ParameterType,
    pub payload: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComponentType {
    Header,
    Body,
    Button,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubType {
    QuickReply,
    Url,
    None,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Component {
    #[serde(rename = "type")]
    pub component_type: ComponentType,
    pub parameters: Vec<Box<dyn ParameterInterface>>,
    pub sub_type: Option<SubType>,
    pub index: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Language {
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Template {
    pub name: String,
    pub language: Option<Language>,
    pub components: Vec<Component>,
}

impl Template {
    pub fn new(template_name: String, lang_code: LanguageCode) -> Self {
        Self {
            name: template_name,
            language: Some(Language {
                code: lang_code.to_string(),
            }),
            components: vec![],
        }
    }

    pub fn from_byte(b: &[u8]) -> Result<Self, Box<dyn Error>> {
        let tmpl: Template = serde_json::from_slice(b)?;
        Ok(tmpl)
    }

    pub fn clean_text(s: &str) -> String {
        let space = regex::Regex::new(r"\s+").unwrap();
        let s = space.replace_all(s, " ");
        let s = s.trim().replace("\n", "").replace("\r", "");
        s
    }

    pub fn add_header(&mut self, text: &str) -> &mut Self {
        self.add_component(
            ComponentType::Header,
            None,
            ParameterType::Text,
            Self::clean_text(text),
        );
        self
    }

    pub fn add_header_image(&mut self, image_link: &str) -> &mut Self {
        self.add_component(
            ComponentType::Header,
            None,
            ParameterType::Image,
            Self::clean_text(image_link),
        );
        self
    }

    pub fn add_body(&mut self, text: &str) -> &mut Self {
        self.add_component(
            ComponentType::Body,
            None,
            ParameterType::Text,
            Self::clean_text(text),
        );
        self
    }

    pub fn add_button(&mut self, text: &str) -> &mut Self {
        self.add_component(
            ComponentType::Button,
            None,
            ParameterType::Text,
            Self::clean_text(text),
        );
        self
    }

    pub fn add_button_payload(&mut self, payload: &str) -> &mut Self {
        self.add_component(
            ComponentType::Button,
            None,
            ParameterType::ButtonPayload,
            payload.to_string(),
        );
        self
    }

    pub fn add_quick_reply(&mut self, text: &str) -> &mut Self {
        self.add_component(
            ComponentType::Button,
            Some(SubType::QuickReply),
            ParameterType::Text,
            Self::clean_text(text),
        );
        self
    }
    
    pub fn add_url(&mut self, url: &str) -> &mut Self {
        self.add_component(
            ComponentType::Button,
            Some(SubType::Url),
            ParameterType::Text,
            Self::clean_text(url),
        );
        self
    }
    
    fn add_component(
        &mut self,
        component_type: ComponentType,
        sub_type: Option<SubType>,
        parameter_type: ParameterType,
        text: String,
    ) {
        let parameter: Box<dyn ParameterInterface> = match parameter_type {
            ParameterType::Text => Box::new(Parameter {
                parameter_type,
                text,
            }),
            ParameterType::Image => Box::new(ImageParameter {
                parameter_type,
                image: ImageLink { link: text },
            }),
            ParameterType::ButtonPayload => Box::new(ButtonPayloadParameter {
                parameter_type,
                payload: text,
            }),
        };
    
        let component = Component {
            component_type,
            parameters: vec![parameter],
            sub_type,
            index: None,
        };
    
        self.components.push(component);
    }
    
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

trait ParameterInterface: Serialize {
fn as_parameter(&self) -> &dyn erased_serde::Serialize;
}

impl ParameterInterface for Parameter {
fn as_parameter(&self) -> &dyn erased_serde::Serialize {
self
}
}

impl ParameterInterface for ImageParameter {
fn as_parameter(&self) -> &dyn erased_serde::Serialize {
self
}
}

impl ParameterInterface for ButtonPayloadParameter {
fn as_parameter(&self) -> &dyn erased_serde::Serialize {
self
}
}

    