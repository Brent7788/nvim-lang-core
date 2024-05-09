#[cfg(test)]
pub mod common_tests {
    use std::env;

    use log::info;
    use nvim_lang_core::common::{
        logger::Logger,
        string::{DelimiterType, StringDelimiterSlice, StringSlice},
    };

    #[test]
    fn simple_string_slices_by_should_be() {
        env::set_var("RUST_BACKTRACE", "1");

        let n = String::from("var x = 'This should be simple.';");

        let n_slices: [Option<&str>; 2] =
            n.slices_by(&DelimiterType::DelimiterChar('\''), &[DelimiterType::None]);

        assert_ne!(None, n_slices[0]);

        if let Some(s) = n_slices[0] {
            assert_eq!("This should be simple.", s);
        }

        let n_slices: [Option<&str>; 2] =
            n.slices_by(&DelimiterType::DelimiterStr("'"), &[DelimiterType::None]);

        assert_ne!(None, n_slices[0]);

        if let Some(s) = n_slices[0] {
            assert_eq!("This should be simple.", s);
        }
    }

    #[test]
    fn simple_string_slices_by_str_dlm_should_be() {
        let n = String::from("var x = *--*This should be simple.*--*;");

        let n_slices: [Option<&str>; 2] =
            n.slices_by(&DelimiterType::DelimiterStr("*--*"), &[DelimiterType::None]);

        assert_ne!(None, n_slices[0]);

        if let Some(s) = n_slices[0] {
            assert_eq!("This should be simple.", s);
        }
    }

    #[test]
    fn empty_string_slice_by_should_be() {
        let n = String::from("var x = 8;");

        let n_slices: [Option<&str>; 2] =
            n.slices_by(&DelimiterType::DelimiterChar('\''), &[DelimiterType::None]);

        assert_eq!(None, n_slices[0]);
        assert_eq!(None, n_slices[1]);
    }

    #[test]
    fn multiple_string_slice_by_should_be() {
        let n = String::from("var x = 'This should be simple.'; print('This is print text');");

        let n_slices: [Option<&str>; 2] =
            n.slices_by(&DelimiterType::DelimiterChar('\''), &[DelimiterType::None]);

        assert_ne!(None, n_slices[0]);
        assert_ne!(None, n_slices[1]);

        if let Some(s) = n_slices[0] {
            assert_eq!("This should be simple.", s);
        }

        if let Some(s) = n_slices[1] {
            assert_eq!("This is print text", s);
        }

        let n_slices: [Option<&str>; 2] =
            n.slices_by(&DelimiterType::DelimiterStr("'"), &[DelimiterType::None]);

        assert_ne!(None, n_slices[0]);
        assert_ne!(None, n_slices[1]);

        if let Some(s) = n_slices[0] {
            assert_eq!("This should be simple.", s);
        }

        if let Some(s) = n_slices[1] {
            assert_eq!("This is print text", s);
        }
    }

    #[test]
    fn half_string_slices_by_should_be() {
        let n = String::from("var x = '\"This should be simple.\"; print(\"This is print text\");");

        let n_slices: [Option<&str>; 1] =
            n.slices_by(&DelimiterType::DelimiterChar('\''), &[DelimiterType::None]);

        if let Some(s) = n_slices[0] {
            assert_eq!(
                "\"This should be simple.\"; print(\"This is print text\");",
                s
            );
        }
    }

    #[test]
    fn string_slices_by_with_ignore_dlm_should_be() {
        let n = String::from(
            "var x = -This is term command(rm -f --no), that will remove everything.-;",
        );

        let n_slices: [Option<&str>; 2] = n.slices_by(
            &DelimiterType::DelimiterChar('-'),
            &[
                DelimiterType::DelimiterStr("-f"),
                DelimiterType::DelimiterStr("--"),
            ],
        );

        assert_ne!(None, n_slices[0]);

        if let Some(s) = n_slices[0] {
            assert_eq!(
                "This is term command(rm -f --no), that will remove everything.",
                s
            );
        }

        let n = String::from("var x = -This is term command(rm -f --no)---;");

        let n_slices: [Option<&str>; 2] = n.slices_by(
            &DelimiterType::DelimiterChar('-'),
            &[
                DelimiterType::DelimiterStr("-f"),
                DelimiterType::DelimiterStr("--"),
            ],
        );

        assert_ne!(None, n_slices[0]);

        if let Some(s) = n_slices[0] {
            assert_eq!("This is term command(rm -f --no)--", s);
        }

        let n = String::from("var x = --This is --%term command(rm -f -no)---;");

        let n_slices: [Option<&str>; 2] = n.slices_by(
            &DelimiterType::DelimiterStr("--"),
            &[
                DelimiterType::DelimiterStr(")-"),
                DelimiterType::DelimiterStr("--%"),
            ],
        );

        assert_ne!(None, n_slices[0]);

        if let Some(s) = n_slices[0] {
            assert_eq!("This is --%term command(rm -f -no)-", s);
        }
    }

    #[test]
    fn simple_string_slice_between_should_be() {
        let simple_string = String::from("Hello, */This is a slice in between./*, end");
        let simple_string = simple_string.slice_between("*/", "/*");
        assert_eq!("This is a slice in between.", simple_string);
        let simple_string = String::from("Hello, /This is a slice in between./, end");
        let simple_string = simple_string.slice_between("/", "/");
        assert_eq!("This is a slice in between.", simple_string);
        let simple_string = String::from("Hello, /This is a slice/in between./, end");
        let simple_string = simple_string.slice_between("/", "/");
        assert_eq!("This is a slice", simple_string);
        let simple_string = String::from("Hello, --[[This is a slice in between. end");
        let simple_string = simple_string.slice_between("--[[", "--]]");
        assert_eq!("This is a slice in between. end", simple_string);
        let simple_string = String::from("Hello, This is a slice in between.--]] end");
        let simple_string = simple_string.slice_between("--[[", "--]]");
        assert_eq!("Hello, This is a slice in between.", simple_string);
        let simple_string = String::from("Hello, This is a slice in between. end");
        let simple_string = simple_string.slice_between("*/", "/*");
        assert_eq!("Hello, This is a slice in between. end", simple_string);
    }

    #[test]
    fn simple_weird_string_slices_between_should_be() {
        env::set_var("RUST_BACKTRACE", "1");

        let simple_string = String::from("--[[ Run Neo Tree on Start 🌲]]");

        let simple_string = simple_string.slice_between("--[[", "--]]");
        assert_eq!(" Run Neo Tree on Start 🌲]]", simple_string);

        let simple_string = String::from("--[[ Run Neo Tree on Start 🌲]]");

        let simple_string = simple_string.slice_between("--[[", "]]");
        assert_eq!(" Run Neo Tree on Start 🌲", simple_string);

        let simple_string = String::from("🌲-🌲Run Neo Tree on Start🌲-🌲");

        let simple_string = simple_string.slice_between("-", "-");
        assert_eq!("🌲Run Neo Tree on Start🌲", simple_string);

        let simple_string = String::from("🌲- Run Neo Tree on Start -🌲");

        let simple_string = simple_string.slice_between("🌲", "🌲");
        assert_eq!("- Run Neo Tree on Start -", simple_string);
    }
}
