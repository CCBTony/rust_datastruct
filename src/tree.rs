// 抽象数据结构
use std::rc::Rc;
use std::cell::RefCell;
use std::rc::Weak;
use std::fmt::Display;

pub mod binary {
    use super::*;

    // 遍历状态
    enum NodeCheckStatus {
        Done,
        UNDO,
    }

    #[derive(Getters, MutGetters, Setters, Clone, Debug)]
    pub struct BinaryNode<V: Clone + Display>
    {
        #[get = "pub"]
        #[set = "pub"]
        #[get_mut = "pub"]
        key: String,

        #[get = "pub"]
        #[set = "pub"]
        #[get_mut = "pub"]
        value: Rc<RefCell<V>>,

        #[get = "pub"]
        #[set = "pub"]
        #[get_mut = "pub"]
        top: Option<Weak<RefCell<BinaryNode<V>>>>,

        #[get = "pub"]
        #[set = "pub"]
        #[get_mut = "pub"]
        left: Option<Rc<RefCell<BinaryNode<V>>>>,

        #[get = "pub"]
        #[set = "pub"]
        #[get_mut = "pub"]
        right: Option<Rc<RefCell<BinaryNode<V>>>>,
    }

    impl<V: Clone + Display> BinaryNode<V> {
        pub fn new(key: String, value: V) -> Self {
            Self {
                key,
                value: Rc::new(RefCell::new(value)),
                top: None,
                left: None,
                right: None,
            }
        }

        pub fn depth(&self) -> i32 {
            let mut d = 0;
            let mut cur: Rc<RefCell<BinaryNode<V>>>;

            match self.top {
                None => return d,
                Some(ref r) => {
                    match r.upgrade() {
                        None => return d,
                        Some(ref r) => {
                            cur = Rc::clone(r);
                            d += 1;
                        }
                    }
                }
            }

            loop {
                let cur_t = Rc::clone(&cur);
                let b_cur_t = cur_t.as_ref().borrow();
                let top = b_cur_t.top.as_ref();
                match top {
                    None => return d,
                    Some(r) => {
                        match r.upgrade() {
                            None => return d,
                            Some(ref r) => {
                                cur = Rc::clone(r);
                                d += 1;
                            }
                        }
                    }
                }
            }
        }

        pub fn height(&self) -> i32 {
            type N<V> = Rc<RefCell<BinaryNode<V>>>;
            let mut h = 0;
            let mut max_h = 0;
            let mut stack: Vec<(N<V>, NodeCheckStatus)> = Vec::new();

            if self.left.is_some() {
                let t = self.left.as_ref();
                stack.push((Rc::clone(t.unwrap()), NodeCheckStatus::UNDO));
            }
            if self.right.is_some() {
                let t = self.right.as_ref();
                stack.push((Rc::clone(t.unwrap()), NodeCheckStatus::UNDO));
            }
            if stack.len() == 0 {
                return h;
            }

            loop {
                match stack.pop() {
                    None => break max_h,
                    Some((node, status)) => {
                        match status {
                            NodeCheckStatus::Done => {
                                if max_h < h {
                                    max_h = h;
                                }
                                h -= 1;
                            }
                            NodeCheckStatus::UNDO => {
                                let node_borrow = node.as_ref().borrow();
                                let left = node_borrow.left.as_ref();
                                let right = node_borrow.right.as_ref();

                                stack.push((Rc::clone(&node), NodeCheckStatus::Done));
                                h += 1;

                                if left.is_none() && right.is_none() {
                                    continue;
                                }
                                if left.is_some() {
                                    stack.push((Rc::clone(left.unwrap()), NodeCheckStatus::UNDO));
                                }
                                if right.is_some() {
                                    stack.push((Rc::clone(right.unwrap()), NodeCheckStatus::UNDO));
                                }
                            }
                        }
                    }
                }
            }
        }

        pub fn left_height(&self) -> i32 {
            match self.left() {
                None => -1,
                Some(ref rc) => rc.as_ref().borrow().height()
            }
        }

        pub fn right_height(&self) -> i32 {
            match self.right() {
                None => -1,
                Some(ref rc) => rc.as_ref().borrow().height()
            }
        }
    }

    // 添加左节点, 返回原左节点
    pub fn link_left<V: Clone + Display>(
        parent_node: Rc<RefCell<BinaryNode<V>>>,
        child_node: Option<Rc<RefCell<BinaryNode<V>>>>,
    ) -> Option<Rc<RefCell<BinaryNode<V>>>> {
        let r_left = if parent_node.as_ref().borrow_mut().left().is_none() {
            None
        } else {
            let t = Rc::clone(parent_node.as_ref().borrow_mut().left().as_ref().unwrap());
            t.as_ref().borrow_mut().set_top(None);
            Some(t)
        };
        if child_node.is_some() {
            child_node.as_ref()
                .unwrap()
                .as_ref()
                .borrow_mut()
                .set_top(Some(Rc::downgrade(&parent_node)));
        }
        parent_node.as_ref().borrow_mut().set_left(child_node);

        r_left
    }

    // 添加右边节点, 返回原右节点
    pub fn link_right<V: Clone + Display>(
        parent_node: Rc<RefCell<BinaryNode<V>>>,
        child_node: Option<Rc<RefCell<BinaryNode<V>>>>,
    ) -> Option<Rc<RefCell<BinaryNode<V>>>> {
        let r_right = if parent_node.as_ref().borrow_mut().right().is_none() {
            None
        } else {
            let t = Rc::clone(parent_node.as_ref().borrow_mut().right().as_ref().unwrap());
            t.as_ref().borrow_mut().set_top(None);
            Some(t)
        };

        if child_node.is_some() {
            let weak_rc = Rc::downgrade(&parent_node);
            child_node
                .as_ref()
                .unwrap()
                .as_ref()
                .borrow_mut()
                .set_top(Some(weak_rc));
        }
        parent_node.as_ref().borrow_mut().set_right(child_node);

        r_right
    }

    pub fn is_left_child<V: Clone + Display>(
        parent_node: Rc<RefCell<BinaryNode<V>>>,
        child_node: Rc<RefCell<BinaryNode<V>>>,
    ) -> bool {
        let left_ptr = match parent_node.as_ref().borrow().left() {
            None => None,
            Some(ref rc) => Some(rc.as_ptr())
        };
        left_ptr.is_some() && child_node.as_ptr() == left_ptr.unwrap()
    }

    pub fn take_from_top<V: Clone + Display>(node: &Rc<RefCell<BinaryNode<V>>>) -> Option<Rc<RefCell<BinaryNode<V>>>> {
        if node.as_ref().borrow().top().is_some() {
            let rc = node.as_ref().borrow().top().as_ref().unwrap().upgrade().unwrap();
            if is_left_child(Rc::clone(&rc), Rc::clone(node)) {
                link_left(Rc::clone(&rc), None);
            } else {
                link_right(Rc::clone(&rc), None);
            }
            Some(rc)
        } else {
            None
        }
    }
}

pub mod search {
    use super::*;
    use super::binary::{BinaryNode, is_left_child};

    pub trait SearchTree<V: Clone + Display> {
        fn root(&self) -> &Option<Rc<RefCell<BinaryNode<V>>>>;
        fn add_node(&mut self, node_rc: Rc<RefCell<BinaryNode<V>>>);

        // 以下为默认实现
        fn find_node(&self, key: &String) -> Option<Rc<RefCell<BinaryNode<V>>>> {
            let r = self.root();
            match *r {
                None => None,
                Some(ref r) => _find_node(key, Rc::clone(r))
            }
        }

        fn find(&self, key: &String) -> Option<Rc<RefCell<V>>> {
            match self.find_node(key) {
                None => None,
                Some(r) => Some(Rc::clone(r.as_ref().borrow().value()))
            }
        }

        fn find_and_clone(&self, key: &String) -> Option<V> {
            match self.find(key) {
                None => None,
                Some(rc) => Some(rc.as_ref().borrow().clone())
            }
        }

        fn add(&mut self, key: String, value: V) {
            let node_rc = Rc::new(RefCell::new(BinaryNode::new(key, value)));
            self.add_node(node_rc);
        }

        fn update(&mut self, key: &String, value: V) -> Result<(), String> {
            let dest = self.find_node(key);
            match dest {
                None => Err(String::from(format!("node={} not exists", key))),
                Some(rc) => {
                    rc.as_ref().borrow_mut().set_value(Rc::new(RefCell::new(value)));
                    Ok(())
                }
            }
        }

        fn height(&self) -> i32 {
            let root = self.root();
            match root {
                None => -1,
                Some(ref rc) => rc.as_ref().borrow().height()
            }
        }

        fn depth(&self) -> i32 { self.height() }
    }

    fn _find_node<V: Clone + Display>(key: &String, mut cur: Rc<RefCell<BinaryNode<V>>>) -> Option<Rc<RefCell<BinaryNode<V>>>> {
        loop {
            let cur_t = Rc::clone(&cur);
            let cur_borrow = cur_t.as_ref().borrow();
            let src_key = cur_borrow.key();
            if *src_key == *key {
                break Some(cur);
            } else if *src_key < *key {
                match cur_borrow.right() {
                    None => break None,
                    Some(ref r) => { cur = Rc::clone(r); }
                }
            } else {
                match cur_borrow.left() {
                    None => break None,
                    Some(ref r) => { cur = Rc::clone(r); }
                }
            }
        }
    }

    pub fn dumps<V: Clone + Display>(node: Rc<RefCell<BinaryNode<V>>>, level: i32) {
        let mut idx = 0;
        while idx < level {
            print!(" ");
            idx += 1;
        }
        let left;
        let right;
        {
            let borrow = node.as_ref().borrow();
            println!(
                "[{}] key={}, value={}, strong={}, weak={}, height={}, depth={}",
                if borrow.top().is_none() {
                    String::from("root")
                } else {
                    match borrow.top().as_ref().unwrap().upgrade() {
                        None => String::from("root"),
                        Some(ref top_rc) => {
                            if is_left_child(Rc::clone(top_rc), Rc::clone(&node)) {
                                format!("{}-L", top_rc.as_ref().borrow().key())
                            } else {
                                format!("{}-R", top_rc.as_ref().borrow().key())
                            }
                        }
                    }
                },
                borrow.key(),
                borrow.value().as_ref().borrow(),
                Rc::strong_count(&node) - 1, Rc::weak_count(&node),
                borrow.height(),
                borrow.depth()
            );

            left = match borrow.left() {
                None => None,
                Some(ref rc) => Some(Rc::clone(rc))
            };

            right = match borrow.right() {
                None => None,
                Some(ref rc) => Some(Rc::clone(rc))
            };
        }

        if let Some(left_rc) = left {
            dumps(left_rc, level + 1);
        }
        if let Some(right_rc) = right {
            dumps(right_rc, level + 1);
        }
    }
}

pub mod avl {
    use super::*;
    use super::binary::*;
    use super::search::*;

    enum TranType {
        SingleLeft,
        // 左单旋
        SingleRight,
        // 右单旋
        DualLeft,
        // 左双旋
        DualRight,
        // 右双旋
        None, // 不旋转
    }

    // 判断旋转类型
    fn _test_tran_type<V: Clone + Display>(root: Rc<RefCell<BinaryNode<V>>>) -> TranType {
        let root_borrow = root.as_ref().borrow();
        let left_tree_height = root_borrow.left_height();
        let right_tree_height = root_borrow.right_height();

        if left_tree_height - right_tree_height > 1 {
            let left = root_borrow.left();
            let left_borrow = left.as_ref().unwrap().as_ref().borrow();
            if left_borrow.left_height() > left_borrow.right_height() {
                return TranType::SingleLeft;
            } else {
                return TranType::DualLeft;
            }
        } else if 1 < right_tree_height - left_tree_height {
            let right = root_borrow.right();
            let right_borrow = right.as_ref().unwrap().as_ref().borrow();
            if right_borrow.left_height() < right_borrow.right_height() {
                return TranType::SingleRight;
            } else {
                return TranType::DualRight;
            }
        } else {
            return TranType::None;
        }
    }

    #[derive(Getters, MutGetters, Setters, Clone, Debug)]
    pub struct AVLTree<V: Clone + Display> {
        #[set = "pub"]
        #[get_mut = "pub"]
        root: Option<Rc<RefCell<BinaryNode<V>>>>,
    }

    impl<V: Clone + Display> SearchTree<V> for AVLTree<V> {
        fn root(&self) -> &Option<Rc<RefCell<BinaryNode<V>>>> {
            return &self.root;
        }

        fn add_node(&mut self, node_rc: Rc<RefCell<BinaryNode<V>>>) {
            if self.root.is_none() {
                self.root = Some(node_rc);
                return;
            }

            let mut cur = Rc::clone(self.root.as_ref().unwrap());

            loop {
                let cur_p = cur.as_ptr();
                let cur_t = Rc::clone(&cur);
                let is_less = cur_t.as_ref().borrow().key() >= node_rc.as_ref().borrow().key();

                if !is_less {
                    if cur_t.as_ref().borrow().right().is_some() {
                        cur = Rc::clone(cur_t.as_ref().borrow().right().as_ref().unwrap());
                    } else {
                        link_right(Rc::clone(&cur), Some(node_rc));
                        break;
                    }
                } else {
                    if cur_t.as_ref().borrow().left().is_some() {
                        cur = Rc::clone(cur_t.as_ref().borrow().left().as_ref().unwrap());
                    } else {
                        link_left(Rc::clone(&cur), Some(node_rc));
                        break;
                    }
                }
            }

            loop {
                let t = Rc::clone(&cur);
                let top = match t.as_ref().borrow().top() {
                    None => None,
                    Some(ref weak) => weak.upgrade()
                };
                let adjust_type = _test_tran_type(Rc::clone(&cur));
                self._adjust(Rc::clone(&cur), adjust_type);
                if let Some(rc) = top {
                    cur = rc;
                } else {
                    break;
                }
            }
        }
    }

    impl<V: Clone + Display> AVLTree<V> {
        pub fn new() -> Self { Self { root: None } }

        pub fn min_val(&self) -> Option<Rc<RefCell<V>>> {
            let mut cur: Rc<RefCell<BinaryNode<V>>>;
            match self.root {
                None => None,
                Some(ref r) => {
                    cur = Rc::clone(r);
                    loop {
                        let cur_t = Rc::clone(&cur);
                        let borrow_cur = cur_t.as_ref().borrow();
                        match borrow_cur.left() {
                            None => break Some(Rc::clone(borrow_cur.value())),
                            Some(ref r) => cur = Rc::clone(r)
                        }
                    }
                }
            }
        }

        pub fn min_val_clone(&self) -> Option<V> {
            match self.min_val() {
                None => None,
                Some(rc) => Some(rc.as_ref().borrow().clone())
            }
        }

        pub fn max_val(&self) -> Option<Rc<RefCell<V>>> {
            let mut cur: Rc<RefCell<BinaryNode<V>>>;
            match self.root {
                None => None,
                Some(ref r) => {
                    cur = Rc::clone(r);
                    loop {
                        let cur_t = Rc::clone(&cur);
                        let borrow_cur = cur_t.as_ref().borrow();
                        match borrow_cur.right() {
                            None => break Some(Rc::clone(borrow_cur.value())),
                            Some(ref r) => cur = Rc::clone(r)
                        }
                    }
                }
            }
        }

        pub fn max_val_clone(&self) -> Option<V> {
            match self.max_val() {
                None => None,
                Some(rc) => Some(rc.as_ref().borrow().clone())
            }
        }

        pub fn height(&self) -> i32 {
            match self.root {
                None => -1,
                Some(ref rc) => rc.as_ref().borrow().height()
            }
        }

        fn _replace_parent(
            &mut self,
            top: Option<Rc<RefCell<BinaryNode<V>>>>,
            old_parent: Rc<RefCell<BinaryNode<V>>>,
            new_parent: Rc<RefCell<BinaryNode<V>>>,
        ) {
            if top.is_some() {
                let top_rc = top.as_ref().unwrap();
                if is_left_child(Rc::clone(top_rc), old_parent) {
                    link_left(Rc::clone(top_rc), Some(new_parent));
                } else {
                    link_right(Rc::clone(top_rc), Some(new_parent));
                }
            } else {
                self.root = Some(new_parent);
            }
        }

        // 旋转平衡，算法参见《数据结构与算法分析：C语言描述》第二版 4.4
        fn _adjust(&mut self, root: Rc<RefCell<BinaryNode<V>>>, t: TranType) {
            match t {
                TranType::SingleRight => {
                    println!("SingleRight!");
                    let mut k1 = Rc::clone(&root);
                    let mut k2 = link_right(Rc::clone(&k1), None).unwrap();
                    let mut y = link_left(Rc::clone(&k2), None);
                    let top = take_from_top(&root);

                    link_right(Rc::clone(&k1), y);
                    link_left(Rc::clone(&k2), Some(k1));

                    self._replace_parent(top, root, k2);
                }
                TranType::DualRight => {
                    println!("DualRight!");
                    let mut k1 = Rc::clone(&root);
                    let mut k3 = link_left(Rc::clone(&k1), None).unwrap();
                    let mut k2 = link_right(Rc::clone(&k3), None).unwrap();
                    let mut b = link_left(Rc::clone(&k2), None);
                    let mut c = link_right(Rc::clone(&k2), None);
                    let top = take_from_top(&root);

                    link_right(Rc::clone(&k1), b);
                    link_left(Rc::clone(&k3), c);
                    link_right(Rc::clone(&k2), Some(k3));
                    link_left(Rc::clone(&k2), Some(k1));

                    self._replace_parent(top, root, k2);
                }
                TranType::SingleLeft => {
                    println!("SingleLeft!");
                    let mut k2 = Rc::clone(&root);
                    let mut k1 = link_left(Rc::clone(&k2), None).unwrap();
                    let mut y = link_left(Rc::clone(&k1), None);
                    let top = take_from_top(&root);

                    link_left(Rc::clone(&k2), y);
                    link_right(Rc::clone(&k1), Some(k2));

                    self._replace_parent(top, root, k1);
                }
                TranType::DualLeft => {
                    println!("DualLeft!");
                    let mut k3 = Rc::clone(&root);
                    let mut k1 = link_left(Rc::clone(&k3), None).unwrap();
                    let mut k2 = link_right(Rc::clone(&k1), None).unwrap();
                    let mut b = link_left(Rc::clone(&k2), None);
                    let mut c = link_right(Rc::clone(&k2), None);
                    let top = take_from_top(&root);

                    link_right(Rc::clone(&k1), b);
                    link_left(Rc::clone(&k3), c);
                    link_right(Rc::clone(&k2), Some(k3));
                    link_left(Rc::clone(&k2), Some(k1));

                    self._replace_parent(top, root, k2);
                }
                _ => ()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tree::search::SearchTree;

    #[test]
    fn binary_node() {
        use super::binary::*;
        use super::search::*;

        let a = Rc::new(RefCell::new(BinaryNode::new(String::from("28"), "Tony")));
        let b = Rc::new(RefCell::new(BinaryNode::new(String::from("20"), "Guo")));
        let c = Rc::new(RefCell::new(BinaryNode::new(String::from("19"), "Guo Tony")));

        let ref_a = a.as_ref();
        let ref_b = b.as_ref();
        let ref_c = c.as_ref();

        link_left(Rc::clone(&a), Some(Rc::clone(&b)));
        link_right(Rc::clone(&a), Some(Rc::clone(&c)));

        assert_eq!(0, ref_a.borrow().depth());
        assert_eq!(1, ref_a.borrow().height());
        assert_eq!(1, ref_b.borrow().depth());
        assert_eq!(0, ref_b.borrow().height());
        assert_eq!(1, ref_c.borrow().depth());
        assert_eq!(0, ref_c.borrow().height());

        assert_eq!(0, ref_a.borrow().left_height());
        assert_eq!(0, ref_a.borrow().right_height());
        assert_eq!(-1, ref_b.borrow().left_height());
        assert_eq!(-1, ref_b.borrow().right_height());
        assert_eq!(-1, ref_c.borrow().left_height());
        assert_eq!(-1, ref_c.borrow().right_height());
    }

    #[test]
    fn link_test() {
        use super::*;
        use super::binary::*;
        use super::search::*;

        let a = Rc::new(RefCell::new(BinaryNode::new(String::from("28"), "Tony")));
        let b = Rc::new(RefCell::new(BinaryNode::new(String::from("20"), "Guo")));
        let c = Rc::new(RefCell::new(BinaryNode::new(String::from("19"), "Guo Tony")));

        link_right(Rc::clone(&a), Some(Rc::clone(&b)));
        assert_eq!(1, Rc::strong_count(&a));
        assert_eq!(1, Rc::weak_count(&a));
        assert_eq!(2, Rc::strong_count(&b));
        assert_eq!(0, Rc::weak_count(&b));
        assert_eq!(1, Rc::strong_count(&c));
        assert_eq!(0, Rc::weak_count(&c));
        link_left(Rc::clone(&a), Some(Rc::clone(&c)));
        assert_eq!(1, Rc::strong_count(&a));
        assert_eq!(2, Rc::weak_count(&a));
        assert_eq!(2, Rc::strong_count(&b));
        assert_eq!(0, Rc::weak_count(&b));
        assert_eq!(2, Rc::strong_count(&c));
        assert_eq!(0, Rc::weak_count(&c));
        link_right(Rc::clone(&a), None);
        assert_eq!(1, Rc::strong_count(&a));
        assert_eq!(1, Rc::weak_count(&a));
        assert_eq!(1, Rc::strong_count(&b));
        assert_eq!(0, Rc::weak_count(&b));
        assert_eq!(2, Rc::strong_count(&c));
        assert_eq!(0, Rc::weak_count(&c));
        link_right(Rc::clone(&a), None);
        assert_eq!(1, Rc::strong_count(&a));
        assert_eq!(1, Rc::weak_count(&a));
        assert_eq!(1, Rc::strong_count(&b));
        assert_eq!(0, Rc::weak_count(&b));
        assert_eq!(2, Rc::strong_count(&c));
        assert_eq!(0, Rc::weak_count(&c));
        link_left(Rc::clone(&a), None);
        assert_eq!(1, Rc::strong_count(&a));
        assert_eq!(0, Rc::weak_count(&a));
        assert_eq!(1, Rc::strong_count(&b));
        assert_eq!(0, Rc::weak_count(&b));
        assert_eq!(1, Rc::strong_count(&c));
        assert_eq!(0, Rc::weak_count(&c));
    }

    #[test]
    fn create_avl_tree() {
        use super::avl::AVLTree;
        use super::search::dumps;

        let mut names = vec!["2234", "1234", "9953", "3012", "7777", "6161", "4532", "6418", "9090", "8011", "5234", "4444"];
        let mut idx = 0;
        let mut tree = AVLTree::<String>::new();

        while names.len() > 0 {
            tree.add(idx.to_string(), String::from(names.pop().unwrap()));
            idx += 1;
        }

        dumps(Rc::clone(tree.root().as_ref().unwrap()), 0);
        assert_eq!(3, tree.height());
        assert_eq!("4444", tree.min_val_clone().unwrap());
        assert_eq!("9953", tree.max_val_clone().unwrap());
    }
}