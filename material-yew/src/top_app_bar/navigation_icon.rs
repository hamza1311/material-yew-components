use yew::prelude::*;

const SLOT: &str = "navigationIcon";

/// Props for [`MatTopAppBarNavigationIcon`]
#[derive(Properties, Clone)]
pub struct TopAppBarNavigationIconProps {
    pub children: Children,
}

/// Defines header for [`MatTopAppBar`][crate::MatTopAppBar] or
/// [`MatTopAppBarFixed`][crate::MatTopAppBarFixed].
///
/// If the child passed is an element (a `VTag`), then it is modified to include
/// the appropriate attributes. Otherwise, the child is wrapped in a `span`
/// containing said attributes.
pub struct MatTopAppBarNavigationIcon {
    props: TopAppBarNavigationIconProps,
}

impl Component for MatTopAppBarNavigationIcon {
    type Message = ();
    type Properties = TopAppBarNavigationIconProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        false
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let children = self
            .props
            .children
            .iter()
            .map(|child| {
                match child {
                    Html::VTag(mut vtag) => {
                        vtag.add_attribute("slot", SLOT);
                        Html::VTag(vtag)
                    }
                    _ => {
                        html! {
                            <span slot=SLOT>
                                { child }
                            </span>
                        }
                    }
                }
            })
            .collect::<Html>();

        html! {
            { children }
        }
    }
}
