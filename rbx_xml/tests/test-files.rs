use std::{fs, path::Path};

use rbx_dom_weak::DomViewer;

macro_rules! test_models {
    ( $( $test_name: ident : $file_name: expr,)* ) => {
        $(
            #[test]
            fn $test_name() {
                let _ = env_logger::try_init();

                let mut path = Path::new("../test-files").join($file_name);
                path.push("xml.rbxmx");

                let contents = fs::read_to_string(path).unwrap();
                let dom = rbx_xml::from_str_default(&contents).unwrap();

                let mut viewer = DomViewer::new();

                insta::assert_yaml_snapshot!(viewer.view_children(&dom));
            }
        )*
    };
}

test_models! {
    attributes: "models/attributes",
    ball_socket_constraint: "models/ball-socket-constraint",
    default_inserted_folder: "models/default-inserted-folder",
    default_inserted_part: "models/default-inserted-part",
    ref_adjacent: "models/ref-adjacent",
    ref_child: "models/ref-child",
    ref_parent: "models/ref-parent",
    tags: "models/tags",
    body_movers: "models/body-movers",
    union: "models/unions",
    sharedstring: "models/sharedstring",
    font: "models/font",

    unknown_type: "edge-cases/xml-unknown-type",
    empty_font: "edge-cases/empty-font",
}
