use crate::dot_path::{positioned_dot_path::RootType, *};
use serde_json;

#[test]
fn test_positioned_dotpath_basic() {
    let path_str = "$.foo.bar";
    let pdp = PositionedDotPath::new(path_str).unwrap();
    assert_eq!(pdp.root, RootType::Local);
    assert_eq!(pdp.branches, vec!["foo".to_string(), "bar".to_string()]);
    assert_eq!(pdp.ending, Ending::Data);
    assert_eq!(pdp.to_string(), path_str);
}

#[test]
fn test_positioned_dotpath_branch_ending() {
    let path_str = "$.foo.bar.";
    let pdp = PositionedDotPath::new(path_str).unwrap();
    assert_eq!(pdp.ending, Ending::Branch);
    assert_eq!(pdp.to_string(), path_str);
}

#[test]
fn test_positioned_dotpath_defined_root() {
    let path_str = "[[root1]].branch1.branch2";
    let pdp = PositionedDotPath::new(path_str).unwrap();
    match pdp.clone().root {
        RootType::Root(r) => assert_eq!(r, "root1".to_string()),
        _ => panic!("Expected defined root"),
    }
    assert_eq!(
        pdp.clone().branches,
        vec!["branch1".to_string(), "branch2".to_string()]
    );
    assert_eq!(pdp.to_string(), path_str);
}

#[test]
fn test_true_dotpath_basic() {
    let path_str = "#.foo.bar";
    let tdp = TrueDotPath::new(path_str).unwrap();
    assert_eq!(tdp.branches, vec!["foo".to_string(), "bar".to_string()]);
    assert_eq!(tdp.ending, Ending::Data);
    assert_eq!(tdp.to_string(), path_str);
}

#[test]
fn test_true_dotpath_branch_ending() {
    let path_str = "#.foo.bar.";
    let tdp = TrueDotPath::new(path_str).unwrap();
    assert_eq!(tdp.ending, Ending::Branch);
    assert_eq!(tdp.to_string(), path_str);
}

#[test]
fn test_dotpath_new_automatic_selection() {
    let pdp_str = "$.a.b";
    let tdp_str = "#.a.b";

    let dp1 = DotPath::new(pdp_str).unwrap();
    match dp1 {
        DotPath::Positioned(_) => (),
        _ => panic!("Expected Positioned variant"),
    }

    let dp2 = DotPath::new(tdp_str).unwrap();
    match dp2 {
        DotPath::True(_) => (),
        _ => panic!("Expected True variant"),
    }
}

#[test]
fn test_positioned_dotpath_errors() {
    // Empty string
    assert!(matches!(
        PositionedDotPath::new(""),
        Err(DotPathCreationError::IsEmpty)
    ));

    // Empty branch
    assert!(matches!(
        PositionedDotPath::new("$..bar"),
        Err(DotPathCreationError::EmptyBranchError(_, 1))
    ));

    // Undefined root
    assert!(matches!(
        PositionedDotPath::new("unknown.foo"),
        Err(DotPathCreationError::UndefinedRootError(_))
    ));

    // Pointing at root data with single local root `$`
    assert!(matches!(
        PositionedDotPath::new("$"),
        Err(DotPathCreationError::PointingAtRootDataError(_))
    ));
}
#[test]
fn test_true_dotpath_errors() {
    // Empty string
    assert!(matches!(
        TrueDotPath::new(""),
        Err(DotPathCreationError::IsEmpty)
    ));

    // Wrong root symbol
    assert!(matches!(
        TrueDotPath::new("$foo.bar"),
        Err(DotPathCreationError::UndefinedRootError(_))
    ));

    // Empty branch
    assert!(matches!(
        TrueDotPath::new("#.foo..bar"),
        Err(DotPathCreationError::EmptyBranchError(_, 2))
    ));

    // Single true root `#` is invalid as data
    assert!(matches!(
        TrueDotPath::new("#"),
        Err(DotPathCreationError::PointingAtRootDataError(_))
    ));
}

#[test]
fn test_true_root_constant() {
    let true_root = TrueDotPath::TRUE_ROOT.clone();
    assert_eq!(true_root.branches.len(), 0);
    assert_eq!(true_root.ending, Ending::Branch);
    assert_eq!(true_root.to_string(), "#.");
}

#[test]
fn test_dotpath_display_trait() {
    let pdp = PositionedDotPath::new("$.[[r]].a.b.").unwrap();
    let dp: DotPath = pdp.clone().into();
    assert_eq!(dp.to_string(), pdp.to_string());

    let tdp = TrueDotPath::new("#.x.y.").unwrap();
    let dp2: DotPath = tdp.clone().into();
    assert_eq!(dp2.to_string(), tdp.to_string());
}

#[test]
fn positioned_dotpath_roundtrip() {
    let path = "$.foo.bar.";
    let pdp = PositionedDotPath::new(path).unwrap();
    let formatted = pdp.to_string();
    let pdp2 = PositionedDotPath::new(&formatted).unwrap();
    assert_eq!(pdp, pdp2);
}

#[test]
fn true_dotpath_roundtrip() {
    let path = "#.x.y.z.";
    let tdp = TrueDotPath::new(path).unwrap();
    let formatted = tdp.to_string();
    let tdp2 = TrueDotPath::new(&formatted).unwrap();
    assert_eq!(tdp, tdp2);
}

#[test]
fn serde_positioned_dotpath() {
    let pdp = PositionedDotPath::new("$.[[r]].a.b").unwrap();
    let json = serde_json::to_string(&pdp).unwrap();
    let de: PositionedDotPath = serde_json::from_str(&json).unwrap();
    assert_eq!(pdp, de);
}

#[test]
fn serde_true_dotpath() {
    let tdp = TrueDotPath::new("#.foo.bar").unwrap();
    let json = serde_json::to_string(&tdp).unwrap();
    let de: TrueDotPath = serde_json::from_str(&json).unwrap();
    assert_eq!(tdp, de);
}

#[test]
fn long_branches_and_unusual_chars() {
    let path = "$.a_1.b-2.c$3.d!e.f@";
    let pdp = PositionedDotPath::new(path).unwrap();
    assert_eq!(pdp.branches, vec!["a_1", "b-2", "c$3", "d!e", "f@"]);
    assert_eq!(pdp.to_string(), path);
}

#[test]
fn multiple_true_dotpaths() {
    let paths = vec!["#.a.b", "#.x.y.z.", "#.branch"];
    for p in paths {
        let tdp = TrueDotPath::new(p).unwrap();
        assert_eq!(tdp.to_string(), p);
    }
}

#[test]
fn conversion_into_dotpath() {
    let pdp = PositionedDotPath::new("$.[[r]].a").unwrap();
    let dp: DotPath = pdp.clone().into();
    match dp {
        DotPath::Positioned(inner) => assert_eq!(inner, pdp),
        _ => panic!("Expected Positioned variant"),
    }

    let tdp = TrueDotPath::new("#.foo").unwrap();
    let dp2: DotPath = tdp.clone().into();
    match dp2 {
        DotPath::True(inner) => assert_eq!(inner, tdp),
        _ => panic!("Expected True variant"),
    }
}

#[test]
fn edge_case_branch_suffix() {
    let path = "$.onlybranch.";
    let pdp = PositionedDotPath::new(path).unwrap();
    assert_eq!(pdp.ending, Ending::Branch);
    assert_eq!(pdp.to_string(), path);

    let tpath = "#.branch.";
    let tdp = TrueDotPath::new(tpath).unwrap();
    assert_eq!(tdp.ending, Ending::Branch);
    assert_eq!(tdp.to_string(), tpath);
}
