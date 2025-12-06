macro_rules! define_pg_tags {
    (
        $($variant:ident => $tag:literal),+ $(,)?
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum PgImageTag {
            Custom(&'static str),
            $($variant,)+
        }

        impl From<PgImageTag> for &'static str {
            fn from(tag: PgImageTag) -> Self {
                match tag {
                    PgImageTag::Custom(tag) => tag,
                    $(
                        PgImageTag::$variant => $tag,
                    )+
                }
            }
        }
    }
}

define_pg_tags! {
    V181 => "18.1",
    V18 => "18",
    V177 => "17.7",
    V175 => "17.5",
    V1611 => "16.11",
    V16 => "16",
}
