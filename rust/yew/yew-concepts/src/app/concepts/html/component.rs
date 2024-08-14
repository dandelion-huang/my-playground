use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ListProps {
    pub children: ChildrenWithProps<ListItem>,
}

#[derive(Properties, PartialEq)]
pub struct ListItemProps {
    pub value: AttrValue,
}

#[function_component]
pub fn List(props: &ListProps) -> Html {
    html! {
        <ul>
            {for props.children.iter()}
        </ul>
    }
}

#[function_component]
pub fn ListItem(props: &ListItemProps) -> Html {
    html! {
        <li>
            {props.value.clone()}
        </li>
    }
}

#[function_component]
pub fn ParentComponent() -> Html {
    html! {
        <List>
            <ListItem value="a" />
            <ListItem value="b" />
            <ListItem value="c" />
        </List>
    }
}

#[function_component]
pub fn Component() -> Html {
    html! {
        <div class="flex flex-col items-center justify-center w-full h-[100dvh] bg-gray-100">
            <ParentComponent />
        </div>
    }
}
