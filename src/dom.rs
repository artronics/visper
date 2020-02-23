pub mod element;
pub mod layout;

use crate::dom::element::Element;
use indextree::{Arena, NodeId};
use std::ops::Deref;
use crate::dom::layout::Layout;

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug, Hash)]
struct ElementId {
    node_id: NodeId
}

impl ElementId {
    fn append(self, child: ElementId, dom: &mut Dom) {
        self.node_id.append(child.node_id, &mut dom.arena)
    }
}

struct Dom {
    arena: Arena<Element>,
    layout: Layout,
}

impl Dom {
    pub fn new() -> Self {
        Dom {
            arena: Arena::new(),
            layout: Layout::new(),
        }
    }

    pub fn add_element(&mut self, elm: Element) -> ElementId {
        let node_id = self.arena.new_node(elm);
        ElementId { node_id }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn foo() {
        let element = Element {};
        let element2 = Element {};
        let dom = &mut Dom::new();

        let e = dom.add_element(element);
        let d = dom.add_element(element2);
        e.append(d, dom)


    }
}
