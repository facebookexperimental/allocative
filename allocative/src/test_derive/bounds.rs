use crate as allocative;
use crate::Allocative;

#[derive(Allocative)]
#[allocative(bound = "K: Allocative, V:Allocative, S")]
struct HashMap<K, V, S = std::collections::hash_map::RandomState> {
    map: std::collections::HashMap<K, V, S>,
}

#[derive(Allocative)]
#[allocative(bound = "S: Sized")]
struct CanBeUnsized<S: ?Sized> {
    #[allocative(visit = via_sized)]
    s: Box<S>,
}

fn via_sized<S>(s: &Box<S>, visitor: &mut allocative::Visitor) {
    visitor
        .enter(
            allocative::Key::new("s"),
            std::mem::size_of_val(Box::as_ref(s)),
        )
        .exit()
}
