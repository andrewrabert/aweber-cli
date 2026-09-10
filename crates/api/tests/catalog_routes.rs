use std::collections::BTreeSet;

use aweber::catalog::{Operation, groups, resolve};

/// Every non-hidden operation is routed exactly once and resolves back to itself.
#[test]
fn routes_and_operations_agree_in_both_directions() {
    let mut routed: Vec<Operation> = Vec::new();
    for group in groups() {
        for operation in group.operations {
            routed.push(*operation);
        }
    }
    let unique: BTreeSet<Operation> = routed.iter().copied().collect();
    assert_eq!(routed.len(), unique.len(), "an operation is routed twice");
    assert_eq!(
        unique.len(),
        Operation::ALL.len(),
        "an operation is not routed"
    );

    for operation in Operation::ALL {
        assert_eq!(
            resolve(operation.group(), operation.action()),
            Some(operation),
            "{operation:?} does not resolve back to itself"
        );
    }
}

/// The four `oauth` operations are hidden by name and sit in the hidden group alone.
#[test]
fn the_oauth_operations_are_hidden_by_name() {
    let hidden: Vec<Operation> = Operation::ALL
        .into_iter()
        .filter(|operation| operation.is_hidden())
        .collect();
    assert_eq!(
        hidden,
        vec![
            Operation::OauthGetAccessToken,
            Operation::OauthGetRequestToken,
            Operation::OauthRevoke,
            Operation::OauthToken,
        ]
    );
    for operation in hidden {
        assert_eq!(operation.group(), "oauth");
    }
    let visible: Vec<&str> = groups()
        .iter()
        .filter(|group| !group.hidden)
        .map(|group| group.name)
        .collect();
    assert!(!visible.contains(&"oauth"));
}

/// Every `ArgSpec` names an argument of the operation's clap command, or a reserved one.
#[test]
fn every_spec_names_a_real_argument() {
    for operation in Operation::ALL {
        let command = operation.command();
        let arguments: BTreeSet<String> = command
            .get_arguments()
            .map(|arg| arg.get_id().to_string())
            .collect();
        for spec in operation.specs() {
            let known =
                arguments.contains(&spec.name) || spec.name == aweber::catalog::PRECONDITION_ARG;
            assert!(
                known,
                "{operation:?} has a spec for {}, which its command does not declare",
                spec.name
            );
        }
    }
}

/// `Operation::ALL` holds each operation once, and `about` answers with the
/// operation's own clap `about` however the two are ordered.
#[test]
fn about_belongs_to_its_operation() {
    let unique: BTreeSet<Operation> = Operation::ALL.into_iter().collect();
    assert_eq!(
        unique.len(),
        Operation::ALL.len(),
        "an operation appears in `ALL` twice"
    );

    let mut shuffled: Vec<Operation> = Operation::ALL.into_iter().collect();
    shuffled.reverse();
    for operation in shuffled {
        let declared = operation
            .command()
            .get_about()
            .map(ToString::to_string)
            .unwrap_or_default();
        assert_eq!(
            operation.about(),
            declared,
            "{operation:?} answers with another operation's help text"
        );
    }
}
