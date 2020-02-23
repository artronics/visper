use stretch::Stretch;
use stretch::style::Style;
use stretch::node::Node;

#[derive(Copy, Clone, Debug)]
pub struct LayoutStyle {
    style: Style,
}

impl Default for LayoutStyle {
    fn default() -> Self {
        LayoutStyle {
            style: Style::default(),
        }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LayoutNode {
    node: Node
}

pub struct Layout {
    stretch: Stretch,
}

impl Layout {
    pub fn new() -> Self {
        Layout {
            stretch: Stretch::new(),
        }
    }

    pub fn new_node(&mut self, layout_style: LayoutStyle, children: Vec<LayoutNode>) -> LayoutNode {
        let layout_children = children.iter().map(|child| child.node).collect();

        LayoutNode {
            node: self.stretch.new_node(layout_style.style, layout_children).expect("Invalid child added as layout")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stretch() {
        let mut layout = Layout::new();
        let style = LayoutStyle::default();
        let n1 = layout.new_node(style, vec![]);
        let n2 = layout.new_node(style, vec![n1]);
    }
}