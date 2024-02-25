use leptos::*;
use leptos_meta::*;
use leptos_router::*;

// Modules
mod components;
mod pages;

// Components
use crate::components::navbar::NavBar;

// Top-Level pages
use crate::pages::health::Health;
use crate::pages::home::Home;
use crate::pages::not_found::NotFound;

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <div id="app">
            <NavBar/>
            <br/>
            <div class="container mx-auto p-4">
                <Router>
                    <Routes>
                        <Route path="/" view=Home/>
                        <Route path="/health" view=Health/>
                        <Route path="/*" view=NotFound/>
                    </Routes>
                </Router>
            </div>
        </div>
    }
}
