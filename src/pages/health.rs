use leptos::*;

pub const GIT_COMMIT_SHA: &str = env!("VERGEN_GIT_SHA");

/// Health show version
#[component]
pub fn Health() -> impl IntoView {
    view! {
        <div class="flex  justify-center">
            <div class="block max-w-[38rem] rounded-lg border border-neutral-200 bg-white dark:border-neutral-300 dark:bg-neutral-600">
                <div class="border-b-2 border-[#0000002d] px-6 py-3 text-neutral-600 dark:text-neutral-50 font-semibold">
                    Build Version
                </div>
                <div class="p-6">
                    <p class="text-base text-black dark:text-neutral-50">
                        <pre class="text-center">{format!("{GIT_COMMIT_SHA}")}</pre>
                    </p>
                </div>
            </div>
        </div>
    }
}
