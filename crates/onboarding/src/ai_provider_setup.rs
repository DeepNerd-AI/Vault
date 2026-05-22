use anyhow::Result;
use gpui::{Action, App, AsyncApp, Context, Entity, IntoElement, Render, Task, Window};
use reqwest::StatusCode;
use settings::{SaturatingBool, update_settings_file};
use ui::SwitchField;
use ui::prelude::*;
use ui_input::InputField;

use crate::Finish;

const ANTHROPIC_API_URL: &str = "https://api.anthropic.com";
const OPENAI_API_URL: &str = "https://api.openai.com";
const GEMINI_API_URL: &str = "https://generativelanguage.googleapis.com";
const GROQ_API_URL: &str = "https://api.groq.com/openai/v1";
const OLLAMA_API_URL: &str = "http://localhost:11434";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ProviderId {
    Anthropic,
    OpenAi,
    Gemini,
    Groq,
    Ollama,
}

#[derive(Clone, Debug, Default)]
struct ProviderStatus {
    is_verified: bool,
    is_testing: bool,
    error_message: Option<String>,
}

pub struct AiProviderSetup {
    anthropic_key: Entity<InputField>,
    openai_key: Entity<InputField>,
    gemini_key: Entity<InputField>,
    groq_key: Entity<InputField>,
    anthropic: ProviderStatus,
    openai: ProviderStatus,
    gemini: ProviderStatus,
    groq: ProviderStatus,
    ollama: ProviderStatus,
    ai_disabled: bool,
}

impl AiProviderSetup {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            anthropic_key: cx.new(|cx| InputField::new(window, cx, "sk-ant-...")),
            openai_key: cx.new(|cx| InputField::new(window, cx, "sk-...")),
            gemini_key: cx.new(|cx| InputField::new(window, cx, "AIza...")),
            groq_key: cx.new(|cx| InputField::new(window, cx, "gsk_...")),
            anthropic: ProviderStatus::default(),
            openai: ProviderStatus::default(),
            gemini: ProviderStatus::default(),
            groq: ProviderStatus::default(),
            ollama: ProviderStatus::default(),
            ai_disabled: false,
        }
    }

    fn has_verified_provider(&self) -> bool {
        self.anthropic.is_verified
            || self.openai.is_verified
            || self.gemini.is_verified
            || self.groq.is_verified
            || self.ollama.is_verified
    }

    fn key_for_provider(&self, provider: ProviderId, cx: &App) -> String {
        match provider {
            ProviderId::Anthropic => self.anthropic_key.read(cx).text(cx),
            ProviderId::OpenAi => self.openai_key.read(cx).text(cx),
            ProviderId::Gemini => self.gemini_key.read(cx).text(cx),
            ProviderId::Groq => self.groq_key.read(cx).text(cx),
            ProviderId::Ollama => String::new(),
        }
    }

    fn status_mut(&mut self, provider: ProviderId) -> &mut ProviderStatus {
        match provider {
            ProviderId::Anthropic => &mut self.anthropic,
            ProviderId::OpenAi => &mut self.openai,
            ProviderId::Gemini => &mut self.gemini,
            ProviderId::Groq => &mut self.groq,
            ProviderId::Ollama => &mut self.ollama,
        }
    }

    fn run_test(&mut self, provider: ProviderId, window: &mut Window, cx: &mut Context<Self>) {
        let mut api_key = None;
        if provider != ProviderId::Ollama {
            let key = self.key_for_provider(provider, cx);
            if key.trim().is_empty() {
                let status = self.status_mut(provider);
                status.is_verified = false;
                status.error_message = Some("Enter an API key first.".to_string());
                cx.notify();
                return;
            }
            api_key = Some(key);
        }

        let status = self.status_mut(provider);
        status.is_testing = true;
        status.error_message = None;
        cx.notify();

        cx.spawn_in(window, async move |this, cx| {
            let result = test_provider(provider, api_key).await;
            this.update(cx, |this, cx| {
                let status = this.status_mut(provider);
                status.is_testing = false;
                match result {
                    Ok(()) => {
                        status.is_verified = true;
                        status.error_message = None;
                    }
                    Err(error_message) => {
                        status.is_verified = false;
                        status.error_message = Some(error_message);
                    }
                }
                cx.notify();
            })
        })
        .detach_and_log_err(cx);
    }

    fn continue_onboarding(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.ai_disabled {
            let fs = <dyn fs::Fs>::global(cx);
            update_settings_file(fs, cx, move |settings, _| {
                settings.project.disable_ai = Some(SaturatingBool(true));
            });
            window.dispatch_action(Finish.boxed_clone(), cx);
            return;
        }

        if !self.has_verified_provider() {
            return;
        }

        let fs = <dyn fs::Fs>::global(cx);
        update_settings_file(fs, cx, move |settings, _| {
            settings.project.disable_ai = Some(SaturatingBool(false));
        });

        let mut credentials_to_store: Vec<(&'static str, String)> = Vec::new();
        if self.anthropic.is_verified {
            credentials_to_store.push((ANTHROPIC_API_URL, self.anthropic_key.read(cx).text(cx)));
        }
        if self.openai.is_verified {
            credentials_to_store.push((OPENAI_API_URL, self.openai_key.read(cx).text(cx)));
        }
        if self.gemini.is_verified {
            credentials_to_store.push((GEMINI_API_URL, self.gemini_key.read(cx).text(cx)));
        }
        if self.groq.is_verified {
            credentials_to_store.push((GROQ_API_URL, self.groq_key.read(cx).text(cx)));
        }

        let credentials_provider = zed_credentials_provider::global(cx);
        window
            .spawn(cx, async move |cx| {
                for (url, key) in credentials_to_store {
                    credentials_provider
                        .write_credentials(url, "Bearer", key.as_bytes(), &cx)
                        .await?;
                }

                cx.update(|window, cx| {
                    window.dispatch_action(Finish.boxed_clone(), cx);
                })?;

                Ok::<(), anyhow::Error>(())
            })
            .detach_and_log_err(cx);
    }

    fn render_provider_card(
        &self,
        provider_name: &'static str,
        provider: ProviderId,
        key_field: Option<Entity<InputField>>,
        button_label: &'static str,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let status = match provider {
            ProviderId::Anthropic => &self.anthropic,
            ProviderId::OpenAi => &self.openai,
            ProviderId::Gemini => &self.gemini,
            ProviderId::Groq => &self.groq,
            ProviderId::Ollama => &self.ollama,
        };

        let card_disabled = self.ai_disabled;

        v_flex()
            .w_full()
            .min_h(rems(10.))
            .p_3()
            .gap_2()
            .border_1()
            .border_color(cx.theme().colors().border_variant)
            .rounded_md()
            .child(
                h_flex()
                    .justify_between()
                    .child(Label::new(provider_name).size(LabelSize::Default).color(
                        if card_disabled {
                            Color::Muted
                        } else {
                            Color::Default
                        },
                    ))
                    .child(
                        Label::new(if card_disabled {
                            "Disabled"
                        } else if status.is_verified {
                            "Verified"
                        } else {
                            "Not verified"
                        })
                        .size(LabelSize::Small)
                        .color(if card_disabled {
                            Color::Muted
                        } else if status.is_verified {
                            Color::Success
                        } else {
                            Color::Muted
                        }),
                    ),
            )
            .when_some(key_field, |this, key_field| {
                if card_disabled {
                    this.child(
                        h_flex().h(rems(2.)).items_center().child(
                            Label::new("AI features are disabled")
                                .color(Color::Muted)
                                .size(LabelSize::Small),
                        ),
                    )
                } else {
                    this.child(key_field)
                }
            })
            .when(!card_disabled, |this| {
                this.when_some(status.error_message.clone(), |this, error_message| {
                    this.child(
                        Label::new(error_message)
                            .size(LabelSize::Small)
                            .color(Color::Error),
                    )
                })
            })
            .child(
                Button::new(
                    format!("test-provider-{:?}", provider),
                    if status.is_testing {
                        "Testing..."
                    } else {
                        button_label
                    },
                )
                .disabled(status.is_testing || card_disabled)
                .on_click(cx.listener(move |this, _, window, cx| {
                    if !card_disabled {
                        this.run_test(provider, window, cx);
                    }
                })),
            )
            .into_any_element()
    }
}

impl Render for AiProviderSetup {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .w_full()
            .gap_4()
            .child(
                v_flex()
                    .gap_1()
                    .child(Headline::new("Set up your AI provider").size(HeadlineSize::Small))
                    .child(
                        Label::new("Verify at least one provider to continue.")
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    ),
            )
            .child(
                v_flex()
                    .gap_2()
                    .child(
                        SwitchField::new(
                            "disable-all-ai-features",
                            Some("Code-Only Mode (Disable all AI features)"),
                            Some("Turn off inline assistants, agents, and predictive completions to use Vault purely as a high-performance code editor.".into()),
                            if self.ai_disabled {
                                ui::ToggleState::Selected
                            } else {
                                ui::ToggleState::Unselected
                            },
                            cx.listener(move |this, &selection, _, cx| {
                                this.ai_disabled = match selection {
                                    ui::ToggleState::Selected => true,
                                    ui::ToggleState::Unselected => false,
                                    ui::ToggleState::Indeterminate => return,
                                };
                                cx.notify();
                            })
                        )
                    )
            )
            .child(
                div()
                    .h_px()
                    .w_full()
                    .bg(cx.theme().colors().border_variant)
            )
            .child(
                h_flex()
                    .w_full()
                    .gap_3()
                    .child(self.render_provider_card(
                        "Anthropic",
                        ProviderId::Anthropic,
                        Some(self.anthropic_key.clone()),
                        "Test",
                        window,
                        cx,
                    ))
                    .child(self.render_provider_card(
                        "OpenAI",
                        ProviderId::OpenAi,
                        Some(self.openai_key.clone()),
                        "Test",
                        window,
                        cx,
                    )),
            )
            .child(
                h_flex()
                    .w_full()
                    .gap_3()
                    .child(self.render_provider_card(
                        "Google Gemini",
                        ProviderId::Gemini,
                        Some(self.gemini_key.clone()),
                        "Test",
                        window,
                        cx,
                    ))
                    .child(self.render_provider_card(
                        "Groq",
                        ProviderId::Groq,
                        Some(self.groq_key.clone()),
                        "Test",
                        window,
                        cx,
                    )),
            )
            .child(self.render_provider_card(
                "Ollama (local, free)",
                ProviderId::Ollama,
                None,
                "Detect",
                window,
                cx,
            ))
            .child(
                Button::new("continue-ai-onboarding", "Continue")
                    .style(ButtonStyle::Filled)
                    .disabled(!self.ai_disabled && !self.has_verified_provider())
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.continue_onboarding(window, cx);
                    })),
            )
    }
}

#[allow(dead_code)]
pub fn has_any_configured_ai_key(cx: &App) -> Task<bool> {
    let provider = zed_credentials_provider::global(cx);
    cx.spawn(async move |cx| {
        let urls = [
            ANTHROPIC_API_URL,
            OPENAI_API_URL,
            GEMINI_API_URL,
            GROQ_API_URL,
            OLLAMA_API_URL,
        ];

        for url in urls {
            match provider.read_credentials(url, &cx).await {
                Ok(Some(_)) => return true,
                Ok(None) => {}
                Err(error) => {
                    zlog::warn!("Failed to read credentials for {url}: {error}");
                }
            }
        }

        false
    })
}

pub async fn has_any_configured_ai_key_async(cx: &mut AsyncApp) -> bool {
    let provider = cx.update(|cx| zed_credentials_provider::global(cx));
    let urls = [
        ANTHROPIC_API_URL,
        OPENAI_API_URL,
        GEMINI_API_URL,
        GROQ_API_URL,
        OLLAMA_API_URL,
    ];

    for url in urls {
        match provider.read_credentials(url, cx).await {
            Ok(Some(_)) => return true,
            Ok(None) => {}
            Err(error) => {
                zlog::warn!("Failed to read credentials for {url}: {error}");
            }
        }
    }

    false
}

async fn test_provider(provider: ProviderId, api_key: Option<String>) -> Result<(), String> {
    reqwest_client::runtime()
        .spawn(async move {
            let client = reqwest::Client::new();
            match provider {
                ProviderId::Anthropic => {
                    let key = api_key.ok_or_else(|| "Missing Anthropic API key".to_string())?;
                    let response = client
                        .post("https://api.anthropic.com/v1/messages")
                        .header("x-api-key", key)
                        .header("anthropic-version", "2023-06-01")
                        .header("content-type", "application/json")
                        .body(
                            serde_json::to_vec(&serde_json::json!({
                                "model": "claude-3-5-haiku-latest",
                                "max_tokens": 1,
                                "messages": [{"role": "user", "content": "ping"}]
                            }))
                            .map_err(|err| err.to_string())?,
                        )
                        .send()
                        .await
                        .map_err(|err| err.to_string())?;
                    if response.status() == StatusCode::OK {
                        Ok(())
                    } else {
                        Err(format!("Anthropic test failed: HTTP {}", response.status()))
                    }
                }
                ProviderId::OpenAi => {
                    let key = api_key.ok_or_else(|| "Missing OpenAI API key".to_string())?;
                    let response = client
                        .get("https://api.openai.com/v1/models")
                        .bearer_auth(key)
                        .send()
                        .await
                        .map_err(|err| err.to_string())?;
                    if response.status() == StatusCode::OK {
                        Ok(())
                    } else {
                        Err(format!("OpenAI test failed: HTTP {}", response.status()))
                    }
                }
                ProviderId::Gemini => {
                    let key = api_key.ok_or_else(|| "Missing Gemini API key".to_string())?;
                    let response = client
                        .get(format!(
                            "https://generativelanguage.googleapis.com/v1beta/models?key={key}"
                        ))
                        .send()
                        .await
                        .map_err(|err| err.to_string())?;
                    if response.status() == StatusCode::OK {
                        Ok(())
                    } else {
                        Err(format!("Gemini test failed: HTTP {}", response.status()))
                    }
                }
                ProviderId::Groq => {
                    let key = api_key.ok_or_else(|| "Missing Groq API key".to_string())?;
                    let response = client
                        .get("https://api.groq.com/openai/v1/models")
                        .bearer_auth(key)
                        .send()
                        .await
                        .map_err(|err| err.to_string())?;
                    if response.status() == StatusCode::OK {
                        Ok(())
                    } else {
                        Err(format!("Groq test failed: HTTP {}", response.status()))
                    }
                }
                ProviderId::Ollama => {
                    let response = client
                        .get("http://localhost:11434/api/tags")
                        .send()
                        .await
                        .map_err(|err| err.to_string())?;
                    if response.status() == StatusCode::OK {
                        Ok(())
                    } else {
                        Err(format!("Ollama is offline: HTTP {}", response.status()))
                    }
                }
            }
        })
        .await
        .map_err(|err| format!("Task panicked: {}", err))?
}
