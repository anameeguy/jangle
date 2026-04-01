use crate::{
    Data::{Branch, Value},
    Ending, TrueDotPath, TrueRoot,
};

impl TrueRoot {
    pub fn is_true_dot_path_valid(&self, true_dot_path: &TrueDotPath) -> bool {
        if true_dot_path.parts.is_empty() {
            return true_dot_path.ending == Ending::Branch;
        }

        let mut current = &self.root;

        for part in &true_dot_path.parts[..true_dot_path.parts.len().saturating_sub(1)] {
            match current.stuff.get(part) {
                Some(Branch(branch)) => current = branch,
                _ => return false,
            }
        }

        match (true_dot_path.parts.last(), true_dot_path.ending) {
            (Some(last), Ending::Data) => matches!(current.stuff.get(last), Some(Value(_))),
            (Some(last), Ending::Branch) => matches!(current.stuff.get(last), Some(Branch(_))),
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Branch, Data, TrueDotPath, TrueRoot, data::Value, dot_path::Ending};

    fn make_tree() -> TrueRoot {
        let mut root = Branch::new();

        // #.a.b.c = 42
        let mut a = Branch::new();
        let mut b = Branch::new();
        b.stuff.insert("c".into(), Data::Value(Value::Int(42)));
        a.stuff.insert("b".into(), Data::Branch(b));
        root.stuff.insert("a".into(), Data::Branch(a));

        // #.x = "hello"
        root.stuff
            .insert("x".into(), Data::Value(Value::String("hello".into())));

        // #.branch_only (empty branch)
        root.stuff
            .insert("branch_only".into(), Data::Branch(Branch::new()));

        TrueRoot { root }
    }

    // --- ✅ VALID CASES ---

    #[test]
    fn valid_deep_value_path() {
        let tree = make_tree();

        let path = TrueDotPath {
            parts: vec!["a".into(), "b".into(), "c".into()],
            ending: Ending::Data,
        };

        assert!(tree.is_true_dot_path_valid(&path));
    }

    #[test]
    fn valid_shallow_value_path() {
        let tree = make_tree();

        let path = TrueDotPath {
            parts: vec!["x".into()],
            ending: Ending::Data,
        };

        assert!(tree.is_true_dot_path_valid(&path));
    }

    #[test]
    fn valid_branch_path() {
        let tree = make_tree();

        let path = TrueDotPath {
            parts: vec!["a".into(), "b".into()],
            ending: Ending::Branch,
        };

        assert!(tree.is_true_dot_path_valid(&path));
    }

    #[test]
    fn valid_root_branch_path() {
        let tree = make_tree();

        let path = TrueDotPath {
            parts: vec!["branch_only".into()],
            ending: Ending::Branch,
        };

        assert!(tree.is_true_dot_path_valid(&path));
    }

    // --- ❌ INVALID CASES ---

    #[test]
    fn invalid_missing_path() {
        let tree = make_tree();

        let path = TrueDotPath {
            parts: vec!["a".into(), "b".into(), "does_not_exist".into()],
            ending: Ending::Data,
        };

        assert!(!tree.is_true_dot_path_valid(&path));
    }

    #[test]
    fn invalid_intermediate_is_value() {
        let tree = make_tree();

        // #.x.anything -> x is a value, not a branch
        let path = TrueDotPath {
            parts: vec!["x".into(), "oops".into()],
            ending: Ending::Data,
        };

        assert!(!tree.is_true_dot_path_valid(&path));
    }

    #[test]
    fn invalid_last_is_branch_but_expect_data() {
        let tree = make_tree();

        // #.a.b is a branch, but expecting data
        let path = TrueDotPath {
            parts: vec!["a".into(), "b".into()],
            ending: Ending::Data,
        };

        assert!(!tree.is_true_dot_path_valid(&path));
    }

    #[test]
    fn invalid_last_is_value_but_expect_branch() {
        let tree = make_tree();

        // #.x is a value, but expecting branch
        let path = TrueDotPath {
            parts: vec!["x".into()],
            ending: Ending::Branch,
        };

        assert!(!tree.is_true_dot_path_valid(&path));
    }

    #[test]
    fn invalid_empty_path_data() {
        let tree = make_tree();

        let path = TrueDotPath {
            parts: vec![],
            ending: Ending::Data,
        };

        assert!(!tree.is_true_dot_path_valid(&path));
    }

    // --- 🔍 EDGE CASES ---

    #[test]
    fn branch_exists_but_child_missing() {
        let tree = make_tree();

        let path = TrueDotPath {
            parts: vec!["a".into(), "b".into(), "missing".into()],
            ending: Ending::Branch,
        };

        assert!(!tree.is_true_dot_path_valid(&path));
    }

    #[test]
    fn deep_branch_then_value_mismatch() {
        let tree = make_tree();

        // #.a.b.c exists but is a value, not a branch
        let path = TrueDotPath {
            parts: vec!["a".into(), "b".into(), "c".into()],
            ending: Ending::Branch,
        };

        assert!(!tree.is_true_dot_path_valid(&path));
    }

    #[test]
    fn single_level_branch_vs_value_confusion() {
        let tree = make_tree();

        // branch_only is a branch, not a value
        let path = TrueDotPath {
            parts: vec!["branch_only".into()],
            ending: Ending::Data,
        };

        assert!(!tree.is_true_dot_path_valid(&path));
    }
}
