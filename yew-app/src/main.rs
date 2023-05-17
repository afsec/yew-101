mod app;

use app::App;
// <button onclick={ onclick_dec }>{ "-1" }</button>

// #[function_component]
// fn App() -> Html {
//     let counter = use_state(|| 0);
//     let onclick_inc = {
//         let counter = counter.clone();
//         move |ev| {
//             let value = *counter + 1;
//             counter.set(value);
//         }
//     };
//     let onclick_dec = {
//         let counter = counter.clone();
//         move |ev| {
//             let value = *counter - 1;
//             counter.set(value);
//         }
//     };

fn main() {
    yew::Renderer::<App>::new().render();
}
