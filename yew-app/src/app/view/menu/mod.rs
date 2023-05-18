use yew::{html, html::Scope, Html};

use crate::app::{
    model::{Model, Page},
    msg::Msg,
    App,
};

pub struct Menu;

impl Menu {
    pub fn view(model: &Model, link: &Scope<App>) -> Html {
        html! {
            <div class="navbar bg-base-100">
                <div class="navbar-start">
                    <a class="btn btn-ghost normal-case text-xl">{"Yew App"}</a>
                </div>
                <div class="navbar-center">
                    <ul class="menu menu-horizontal px-1">
                        <li class="px-1">
                            <a
                                class={ if model.page == Page::Home {"btn-primary"} else {""}}
                                onclick={link.callback(|_| Msg::ShowPage(Page::Home))}>
                                    {"Home"}
                            </a>
                        </li>
                        <li class="px-1">
                            <a
                                class={ if model.page == Page::Posts {"btn-primary"} else {""}}
                                onclick={link.callback(|_| Msg::ShowPage(Page::Posts))}>
                                    {"Posts"}
                            </a>
                        </li>
                        <li class="px-1">
                            <a
                                class={ if model.page == Page::Settings {"btn-primary"} else {""}}
                                onclick={link.callback(|_| Msg::ShowPage(Page::Settings))}>
                                    {"Settings"}
                            </a>
                        </li>
                    </ul>
                </div>
                <div class="navbar-end">
                    <button class="btn btn-square btn-ghost">
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="inline-block w-5 h-5 stroke-current"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 12h.01M12 12h.01M19 12h.01M6 12a1 1 0 11-2 0 1 1 0 012 0zm7 0a1 1 0 11-2 0 1 1 0 012 0zm7 0a1 1 0 11-2 0 1 1 0 012 0z"></path></svg>
                    </button>
                </div>
          </div>
        }
    }
}
/*

           <div>
               <ul>
                   <li>
                       <a
                           class={ if model.page == Page::Home {"active"} else {""}}
                           href={"#"}
                   </li>{"   "}
                   <li>
                       <a
                           class={ if model.page == Page::Posts {"active"} else {""}}
                           href={"#"} </a>
                   </li>{"   "}
                   <li>
                       <a
                           class={ if model.page == Page::Settings {"active"} else {""}}
                           href={"#"} </a>
                   </li>
               </ul>
           </div>

*/
