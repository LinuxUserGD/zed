use std::rc::Rc;

use gpui::ClickEvent;
use ui::{prelude::*, IconButtonShape, Tooltip};

use crate::context::{ContextKind, ContextSnapshot};

#[derive(IntoElement)]
pub enum ContextPill {
    Added {
        context: ContextSnapshot,
        dupe_name: bool,
        focused: bool,
        on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
        on_remove: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
    },
    Suggested {
        name: SharedString,
        icon_path: Option<SharedString>,
        kind: ContextKind,
        focused: bool,
        on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
    },
}

impl ContextPill {
    pub fn added(
        context: ContextSnapshot,
        dupe_name: bool,
        focused: bool,
        on_remove: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
    ) -> Self {
        Self::Added {
            context,
            dupe_name,
            on_remove,
            focused,
            on_click: None,
        }
    }

    pub fn suggested(
        name: SharedString,
        icon_path: Option<SharedString>,
        kind: ContextKind,
        focused: bool,
    ) -> Self {
        Self::Suggested {
            name,
            icon_path,
            kind,
            focused,
            on_click: None,
        }
    }

    pub fn on_click(mut self, listener: Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>) -> Self {
        match &mut self {
            ContextPill::Added { on_click, .. } => {
                *on_click = Some(listener);
            }
            ContextPill::Suggested { on_click, .. } => {
                *on_click = Some(listener);
            }
        }
        self
    }

    pub fn id(&self) -> ElementId {
        match self {
            Self::Added { context, .. } => {
                ElementId::NamedInteger("context-pill".into(), context.id.0)
            }
            Self::Suggested { .. } => "suggested-context-pill".into(),
        }
    }

    pub fn icon(&self) -> Icon {
        match self {
            Self::Added { context, .. } => match &context.icon_path {
                Some(icon_path) => Icon::from_path(icon_path),
                None => Icon::new(context.kind.icon()),
            },
            Self::Suggested {
                icon_path: Some(icon_path),
                ..
            } => Icon::from_path(icon_path),
            Self::Suggested {
                kind,
                icon_path: None,
                ..
            } => Icon::new(kind.icon()),
        }
    }
}

impl RenderOnce for ContextPill {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let color = cx.theme().colors();

        let base_pill = h_flex()
            .id(self.id())
            .pl_1()
            .pb(px(1.))
            .border_1()
            .rounded_sm()
            .gap_1()
            .child(self.icon().size(IconSize::XSmall).color(Color::Muted));

        match &self {
            ContextPill::Added {
                context,
                dupe_name,
                on_remove,
                focused,
                on_click,
            } => base_pill
                .bg(color.element_background)
                .border_color(if *focused {
                    color.border_focused
                } else {
                    color.border.opacity(0.5)
                })
                .pr(if on_remove.is_some() { px(2.) } else { px(4.) })
                .child(
                    h_flex()
                        .id("context-data")
                        .gap_1()
                        .child(
                            div().max_w_64().child(
                                Label::new(context.name.clone())
                                    .size(LabelSize::Small)
                                    .truncate(),
                            ),
                        )
                        .when_some(context.parent.as_ref(), |element, parent_name| {
                            if *dupe_name {
                                element.child(
                                    Label::new(parent_name.clone())
                                        .size(LabelSize::XSmall)
                                        .color(Color::Muted),
                                )
                            } else {
                                element
                            }
                        })
                        .when_some(context.tooltip.clone(), |element, tooltip| {
                            element.tooltip(Tooltip::text(tooltip.clone()))
                        }),
                )
                .when_some(on_remove.as_ref(), |element, on_remove| {
                    element.child(
                        IconButton::new(("remove", context.id.0), IconName::Close)
                            .shape(IconButtonShape::Square)
                            .icon_size(IconSize::XSmall)
                            .tooltip(Tooltip::text("Remove Context"))
                            .on_click({
                                let on_remove = on_remove.clone();
                                move |event, window, cx| on_remove(event, window, cx)
                            }),
                    )
                })
                .when_some(on_click.as_ref(), |element, on_click| {
                    let on_click = on_click.clone();
                    element.on_click(move |event, window, cx| on_click(event, window, cx))
                }),
            ContextPill::Suggested {
                name,
                icon_path: _,
                kind,
                focused,
                on_click,
            } => base_pill
                .cursor_pointer()
                .pr_1()
                .border_color(if *focused {
                    color.border_focused
                } else {
                    color.border_variant.opacity(0.5)
                })
                .hover(|style| style.bg(color.element_hover.opacity(0.5)))
                .child(
                    div().px_0p5().max_w_64().child(
                        Label::new(name.clone())
                            .size(LabelSize::Small)
                            .color(Color::Muted)
                            .truncate(),
                    ),
                )
                .child(
                    Label::new(match kind {
                        ContextKind::File => "Active Tab",
                        ContextKind::Thread
                        | ContextKind::Directory
                        | ContextKind::FetchedUrl
                        | ContextKind::Symbol => "Active",
                    })
                    .size(LabelSize::XSmall)
                    .color(Color::Muted),
                )
                .child(
                    Icon::new(IconName::Plus)
                        .size(IconSize::XSmall)
                        .into_any_element(),
                )
                .tooltip(|window, cx| {
                    Tooltip::with_meta("Suggested Context", None, "Click to add it", window, cx)
                })
                .when_some(on_click.as_ref(), |element, on_click| {
                    let on_click = on_click.clone();
                    element.on_click(move |event, window, cx| on_click(event, window, cx))
                }),
        }
    }
}
