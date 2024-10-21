use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Theme {
	foreground: String,
	background: String,
}

#[derive(PartialEq, Properties)]
pub struct NavbarProps {
	theme: Theme,
}

#[function_component]
fn Navbar(props: &NavbarProps) -> Html {
	html! {
		<nav class={classes!("bg-neutral-100", "p-4")}>
			<Title theme={props.theme.clone()}>
				{ "App title" }
			</Title>
			<NavButton theme={props.theme.clone()}>
				{ "Somewhere" }
			</NavButton>
		</nav>
	}
}

#[derive(PartialEq, Properties)]
pub struct ThemeProps {
	theme: Theme,
	children: Children,
}

#[function_component]
fn Title(props: &ThemeProps) -> Html {
	html! {
		<p class={format!("text-{}-500", props.theme.clone().foreground)}>{props.children.clone()}</p>
	}
}

#[function_component]
fn NavButton(props: &ThemeProps) -> Html {
	html! {
		<p class={format!("bg-{}-500", props.theme.clone().background)}>{props.children.clone()}</p>
	}
}

#[function_component]
pub fn Component() -> Html {
	let theme = Theme {
		foreground: "red".to_owned(),
		background: "blue".to_owned(),
	};

	html! {
		<div class="flex flex-col gap-2 items-center justify-center w-full min-h-[100dvh] p-8 bg-gray-100">
			<Navbar {theme} />
		</div>
	}
}
