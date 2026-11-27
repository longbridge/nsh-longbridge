//! `Avatar`, `AvatarBadge`, `AvatarGroup` and `AvatarGroupCount` in a
//! headless engine: the initials and what screen readers get, sizes, two
//! letters fitting at each size, the
//! group's sizes, overlap and overflow, and a scope's tokens and the app's
//! classes reaching the fallback.

mod common;

use common::{bounds, light, start};
use nsh_core::prelude::*;
use nsh_longbridge::{
    ScopeTokens, Size, avatar, avatar_badge, avatar_group, avatar_group_count,
    components::avatar::AVATAR_FALLBACK, theme_scope,
};
use nsh_testing::Headless;

/// The fill of the view holding `selector` (the fallback around its
/// initials).
fn parent_fill(app: &Headless, selector: &str) -> Option<u32> {
    let screen = app.screen();
    let id = screen.id(selector).unwrap();
    let parent = screen
        .layout_nodes()
        .into_iter()
        .find(|n| n.id == id)
        .unwrap_or_else(|| panic!("no layout for {selector}"))
        .parent;
    screen
        .view_style(&format!("view:{parent}"))
        .unwrap()
        .unwrap()
        .background_color
}

fn side(app: &Headless, selector: &str) -> (f32, f32) {
    let b = bounds(app, selector);
    (b.size.width.0, b.size.height.0)
}

/// Avatars of each size, one decorative, one with a badge.
#[component]
fn People() -> impl IntoElement {
    view()
        .items_start()
        .child(
            avatar()
                .name("Ada Lovelace")
                .size(Size::Xs)
                .accessibility_label("xs"),
        )
        .child(
            avatar()
                .name("Ada Lovelace")
                .size(Size::Sm)
                .accessibility_label("sm"),
        )
        .child(avatar().name("Grace Hopper"))
        .child(
            avatar()
                .name("Alan Turing")
                .size(Size::Lg)
                .accessibility_label("lg"),
        )
        .child(avatar().name("Hidden Person").accessibility_label(""))
        .child(
            avatar()
                .name("Online Person")
                .accessibility_label("Online Person, online")
                .child(avatar_badge()),
        )
}

#[test]
fn the_initials_and_the_name_for_screen_readers() {
    let app = start::<People>(true);
    let screen = app.screen();
    assert!(screen.matches("role:image[Grace Hopper]").unwrap());
    assert!(screen.shows("GH"));
    assert!(screen.matches("role:image[Online Person, online]").unwrap());
    // Decorative: hidden from screen readers, its initials still drawn.
    assert!(!screen.matches("role:image[Hidden Person]").unwrap());
    assert!(screen.shows("HP"));
}

#[test]
fn sizes_are_the_same_on_touch_screens() {
    for desktop in [true, false] {
        let app = start::<People>(desktop);
        assert_eq!(side(&app, "role:image[xs]"), (20.0, 20.0));
        assert_eq!(side(&app, "role:image[sm]"), (24.0, 24.0));
        assert_eq!(side(&app, "role:image[Grace Hopper]"), (32.0, 32.0));
        assert_eq!(side(&app, "role:image[lg]"), (40.0, 40.0));
    }
}

/// A badge sits at the avatar's bottom end, 10 at `md`.
#[test]
fn a_badge_sits_at_the_bottom_end() {
    let app = start::<People>(true);
    let avatar = bounds(&app, "role:image[Online Person, online]");
    let screen = app.screen();
    let badge = screen
        .layout_nodes()
        .into_iter()
        .filter(|n| {
            n.bounds.size.width.0 == 10.0
                && n.bounds.origin.y.0 >= avatar.origin.y.0
                && n.bounds.origin.y.0 < avatar.origin.y.0 + 32.0
        })
        .map(|n| n.bounds)
        .next()
        .expect("a 10-point badge");
    assert_eq!(
        badge.origin.x.0 + 10.0,
        avatar.origin.x.0 + avatar.size.width.0
    );
    assert_eq!(
        badge.origin.y.0 + 10.0,
        avatar.origin.y.0 + avatar.size.height.0
    );
}

/// A small group of five, three shown, the count after them; one avatar
/// with its own size.
#[component]
fn Team() -> impl IntoElement {
    view().items_start().child(
        avatar_group()
            .size(Size::Sm)
            .accessibility_label("Team")
            .total(5)
            .overflow(avatar_group_count().child("+2"))
            .child(avatar().name("Ada Lovelace"))
            .child(avatar().name("Grace Hopper"))
            .child(avatar().name("Alan Turing").size(Size::Lg))
            .child(avatar().name("Edsger Dijkstra"))
            .child(avatar().name("Barbara Liskov")),
    )
}

/// ★ NSH gaps 2: the avatars of a group take its size and overlap, the
/// first has no overlap, and one with its own `size` keeps it.
#[test]
fn a_group_sizes_and_overlaps_its_avatars() {
    let app = start::<Team>(true);
    let screen = app.screen();
    assert!(screen.matches("role:group[Team]").unwrap());
    assert!(!screen.matches("role:image[Edsger Dijkstra]").unwrap());
    assert!(screen.shows("+2"));
    let ada = bounds(&app, "role:image[Ada Lovelace]");
    let grace = bounds(&app, "role:image[Grace Hopper]");
    let alan = bounds(&app, "role:image[Alan Turing]");
    assert_eq!(side(&app, "role:image[Ada Lovelace]"), (24.0, 24.0));
    assert_eq!(side(&app, "role:image[Alan Turing]"), (40.0, 40.0));
    let group = bounds(&app, "role:group[Team]");
    assert_eq!(
        ada.origin.x.0, group.origin.x.0,
        "the first does not overlap"
    );
    assert_eq!(grace.origin.x.0, ada.origin.x.0 + 24.0 - 4.0);
    assert_eq!(
        alan.origin.x.0,
        grace.origin.x.0 + 24.0 - 4.0,
        "the group's overlap, not the avatar's size's"
    );
    // The count's circle: the nearest view around its text as wide as the
    // group's avatars.
    let nodes = app.screen().layout_nodes();
    let mut id = app.screen().id("text:+2").unwrap();
    let circle = loop {
        let node = nodes.iter().find(|n| n.id == id).unwrap();
        if node.bounds.size.width.0 == 24.0 {
            break node.bounds;
        }
        id = node.parent;
    };
    assert_eq!(
        circle.origin.x.0,
        alan.origin.x.0 + 40.0 - 4.0,
        "the count overlaps too"
    );
}

/// Fallbacks outside and inside a scope changing `avatar-fallback`, and one
/// whose `fallbackClass` sets its fill.
#[component]
fn Fills() -> impl IntoElement {
    view()
        .child(avatar().name("Out Side"))
        .child(
            theme_scope()
                .tokens(ScopeTokens::new().set("avatar-fallback", "#2563eb"))
                .child(avatar().name("In Side")),
        )
        .child(
            avatar()
                .name("Class Wins")
                .fallback_class(Some("bg-[#16a34a]".into())),
        )
}

/// ★ NSH gaps 1: the fallback's look is generated classes a scope reaches
/// and the app's `fallbackClass` overrides.
#[test]
fn a_scope_and_the_apps_class_reach_the_fallback() {
    let app = start::<Fills>(true);
    assert_eq!(parent_fill(&app, "text:OS"), light(AVATAR_FALLBACK));
    assert_eq!(parent_fill(&app, "text:IS"), common::css("#2563eb"));
    assert_eq!(parent_fill(&app, "text:CW"), common::css("#16a34a"));
}

/// Two wide letters at each size, and a group of each size with a count.
#[component]
fn Wide() -> impl IntoElement {
    let sizes = [
        (Size::Xs, "Mark Wong"),
        (Size::Sm, "Will Moss"),
        (Size::Md, "Mary Wade"),
        (Size::Lg, "Walt Mann"),
    ];
    view()
        .items_start()
        .children(
            sizes
                .into_iter()
                .map(|(size, name)| avatar().name(name).size(size)),
        )
        .children(sizes.into_iter().map(|(size, _)| {
            avatar_group()
                .size(size)
                .limit(1)
                .total(9)
                .accessibility_label(format!("{size:?} group"))
                .overflow(avatar_group_count().child(format!("+{size:?}")))
                .child(avatar().name("Ada Lovelace"))
                .child(avatar().name("Grace Hopper"))
        }))
}

/// The circle around a group's count: the nearest view around its text as
/// wide as `side`.
fn count_circle(app: &Headless, count: &str, side: f32) -> nsh_core::geometry::Bounds<Points> {
    let nodes = app.screen().layout_nodes();
    let mut id = app.screen().id(&format!("text:{count}")).unwrap();
    loop {
        let node = nodes.iter().find(|n| n.id == id).unwrap();
        if node.bounds.size.width.0 == side {
            return node.bounds;
        }
        id = node.parent;
    }
}

/// The initials and a group's count are set in a size that keeps two wide
/// letters inside the circle at every size, and in a group the next circle
/// leaves the initials before it clear.
#[test]
fn two_letters_fit_at_every_size() {
    let app = start::<Wide>(true);
    let screen = app.screen();
    for (name, initials, side, font) in [
        ("Mark Wong", "MW", 20.0, 8.0),
        ("Will Moss", "WM", 24.0, 10.0),
        ("Mary Wade", "MW", 32.0, 14.0),
        ("Walt Mann", "WM", 40.0, 16.0),
    ] {
        let label = format!("role:image[{name}] >> text:{initials}");
        let style = screen.span_style(&label).unwrap();
        assert_eq!(style.font_size.0, font, "{name}");
        let width = bounds(&app, &label).size.width.0;
        assert!(width <= side * 0.8, "{name}: {width} wide in {side}");
    }
    for (size, side, font) in [
        ("Xs", 20.0, 8.0),
        ("Sm", 24.0, 10.0),
        ("Md", 32.0, 14.0),
        ("Lg", 40.0, 16.0),
    ] {
        let count = format!("+{size}");
        assert_eq!(
            screen
                .span_style(&format!("text:{count}"))
                .unwrap()
                .font_size
                .0,
            font,
            "{size}"
        );
        let initials = bounds(
            &app,
            &format!("role:group[{size} group] >> role:image[Ada Lovelace] >> text:AL"),
        );
        let next = count_circle(&app, &count, side);
        assert!(
            initials.origin.x.0 + initials.size.width.0 <= next.origin.x.0,
            "{size}: the initials end at {}, the next circle starts at {}",
            initials.origin.x.0 + initials.size.width.0,
            next.origin.x.0
        );
    }
}
