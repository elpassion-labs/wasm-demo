#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate yew;

use failure::Error;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

#[derive(Default)]
pub struct GithubService {
    web: FetchService,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Issue {
    title: String,
    state: String
}

type Issues = Vec<Issue>;
type IssuesResult = Result<Issues, Error>;

impl GithubService {
    pub fn new() -> Self {
        Self {
            web: FetchService::new(),
        }
    }

    pub fn get_issues(&mut self, repo: String, callback: Callback<IssuesResult>) -> FetchTask {
        let url = format!("https://api.github.com/repos/{}/issues?state=all", repo);

        let handler = move |response: Response<Json<IssuesResult>>| {
            let (meta, Json(data)) = response.into_parts();
            if meta.status.is_success() {
                callback.emit(data)
            } else {
                callback.emit(Err(
                    format_err!("error: {}", "foo")
                ))
            }
        };

        let request = Request::get(url.as_str()).body(Nothing).unwrap();
        self.web.fetch(request, handler.into())
    }
}

struct Model {
    github_service: GithubService,
    issues: Vec<Issue>,
    issues_callback: Callback<IssuesResult>,
    task: Option<FetchTask>,
    repo: String,
}

enum Msg {
    GetIssues,
    IssuesReady(IssuesResult),
    GotInput(String),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        Model {
            github_service: GithubService::new(),
            issues: Vec::<Issue>::new(),
            issues_callback: link.send_back(Msg::IssuesReady),
            task: None,
            repo: "DenisKolodin/yew".into(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GetIssues => {
                let task = self.github_service.get_issues(self.repo.clone(), self.issues_callback.clone());
                self.task = Some(task);

            }

            Msg::IssuesReady(Ok(issues)) => {
                self.issues = issues;
            }

            Msg::IssuesReady(Err(_)) => {
                let mut gh_issues = Vec::<Issue>::new();
                gh_issues.push(Issue { title: "Error".to_string(), state: "closed".to_string()});
                self.issues = gh_issues;
            }

            Msg::GotInput(value) => {
                self.repo = value
            }
        }

        // Update your model on events
        true
    }
}

fn view_sidebar_tab(text: &str) -> Html<Model> {
    html! {
        <li class="navigation__item js-show-all",>
            <img src="./icon-github.svg", class="navigation__icon", />
            <div class="navigation__text",>{ text }</div>
        </li>
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let view_issue = |issue: &Issue| html! {
            <li class="list__item",>
              <p class="list__item__title",>{ &issue.title }</p>
            </li>
        };

        html! {
            // <div class="background",></div>
            <div class="app",>
                <aside class="app__sider",>
                    <div class="window-buttons",>
                        <button class="window-buttons__button window-buttons__button--close",></button>
                        <button class="window-buttons__button window-buttons__button--minimize",></button>
                        <button class="window-buttons__button window-buttons__button--maximize",></button>
                    </div>

                    <nav class="navigation js-nav",>
                        <ul class="navigation__wrapper",>
                            { view_sidebar_tab("All") }
                            { view_sidebar_tab("Open") }
                            { view_sidebar_tab("Close") }
                            <li class="navigation__item js-show-all",>
                                <button onclick=|_| Msg::GetIssues,>{ "fetch issues" }</button>
                            </li>
                        </ul>
                    </nav>
                </aside>

                <div class="app__content",>
                    <ul class="list",>
                        { for self.issues.iter().map(view_issue) }
                    </ul>
                </div>
            </div>
            // <input type="text", oninput=|e| Msg::GotInput(e.value),></input>
        }
    }
}

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
