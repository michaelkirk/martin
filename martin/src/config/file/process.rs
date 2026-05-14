#[cfg(all(feature = "mlt", feature = "_tiles"))]
use mlt_core::encoder::EncoderConfig;
#[cfg(all(feature = "mlt", feature = "_tiles"))]
use serde::{Deserialize, Serialize};

#[cfg(all(feature = "mlt", feature = "_tiles"))]
use crate::config::primitives::AutoOption;

/// Internal carrier for resolved per-source processing settings.
///
/// Not serialized directly - config files use `convert_to_mlt` / `convert_to_mvt`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ProcessConfig {
    #[cfg(all(feature = "mlt", feature = "_tiles"))]
    pub convert_to_mlt: Option<MltProcessConfig>,
    #[cfg(all(feature = "mlt", feature = "_tiles"))]
    pub convert_to_mvt: Option<MvtProcessConfig>,
}

#[cfg(all(feature = "mlt", feature = "_tiles"))]
impl ProcessConfig {
    /// Whether MVT→MLT conversion is configured to run on every request,
    /// regardless of the client's `Accept` header.
    #[must_use]
    pub fn always_convert_to_mlt(&self) -> bool {
        self.convert_to_mlt
            .as_ref()
            .and_then(MltProcessConfig::as_explicit)
            .is_some_and(|c| c.when_to_convert.is_always())
    }

    /// Whether MLT→MVT conversion is configured to run on every request,
    /// regardless of the client's `Accept` header.
    #[must_use]
    pub fn always_convert_to_mvt(&self) -> bool {
        self.convert_to_mvt
            .as_ref()
            .and_then(MvtProcessConfig::as_explicit)
            .is_some_and(|c| c.when_to_convert.is_always())
    }
}

/// Configuration for MVT-to-MLT format conversion.
///
/// Three-state value parsed from YAML:
/// - `"auto"` / `"default"` / `true` — use `mlt-core`'s default `EncoderConfig`
/// - `"disabled"` / `"off"` / `"no"` / `false` — explicitly skip conversion
/// - An object with explicit fields — override specific encoder settings
#[cfg(all(feature = "mlt", feature = "_tiles"))]
pub type MltProcessConfig = AutoOption<MltEncoderConfig>;

/// Configuration for MLT-to-MVT format conversion.
#[cfg(all(feature = "mlt", feature = "_tiles"))]
pub type MvtProcessConfig = AutoOption<MvtEncoderConfig>;

/// When to apply a format conversion.
///
/// Conversion features default to [`MatchAccept`](Self::MatchAccept): the
/// pipeline only runs when the client's `Accept` header selects the target
/// format. Set to [`Always`](Self::Always) to convert every tile and have the
/// source advertised as the post-conversion format regardless of `Accept`.
#[cfg(all(feature = "mlt", feature = "_tiles"))]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "unstable-schemas", derive(schemars::JsonSchema))]
#[serde(rename_all = "kebab-case")]
pub enum WhenToConvert {
    /// Convert only when the client's `Accept` header selects the target format.
    #[default]
    MatchAccept,
    /// Convert every tile, regardless of the client's `Accept` header. The
    /// source is advertised as the post-conversion format.
    Always,
}

#[cfg(all(feature = "mlt", feature = "_tiles"))]
impl WhenToConvert {
    /// Returns `true` if conversion should happen regardless of `Accept`.
    #[must_use]
    pub fn is_always(self) -> bool {
        matches!(self, Self::Always)
    }
}

// serde's `skip_serializing_if` requires `fn(&T) -> bool`.
#[cfg(all(feature = "mlt", feature = "_tiles"))]
#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_default_when_to_convert(w: &WhenToConvert) -> bool {
    *w == WhenToConvert::default()
}

/// Explicit encoder configuration for MVT conversion
#[cfg(all(feature = "mlt", feature = "_tiles"))]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "unstable-schemas", derive(schemars::JsonSchema))]
pub struct MvtEncoderConfig {
    /// Whether MLT→MVT conversion runs only when the client's `Accept` header
    /// asks for MVT (default) or on every request.
    #[serde(default, skip_serializing_if = "is_default_when_to_convert")]
    pub when_to_convert: WhenToConvert,
}

/// Explicit encoder configuration for MLT conversion.
/// All fields are optional; unset fields use `mlt-core`'s defaults.
#[cfg(all(feature = "mlt", feature = "_tiles"))]
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "unstable-schemas", derive(schemars::JsonSchema))]
pub struct MltEncoderConfig {
    /// Generate tessellation data for polygons and multi-polygons.
    pub tessellate: Option<bool>,
    /// Try sorting features by Z-order (Morton) curve index of their first vertex.
    pub try_spatial_morton_sort: Option<bool>,
    /// Try sorting features by Hilbert curve index of their first vertex.
    pub try_spatial_hilbert_sort: Option<bool>,
    /// Try sorting features by their feature ID in ascending order.
    pub try_id_sort: Option<bool>,
    /// Allow FSST string compression.
    pub allow_fsst: Option<bool>,
    /// Allow `FastPFOR` integer compression.
    pub allow_fpf: Option<bool>,
    /// Allow string grouping into shared dictionaries.
    pub allow_shared_dict: Option<bool>,
    /// Whether MVT→MLT conversion runs only when the client's `Accept` header
    /// asks for MLT (default) or on every request.
    #[serde(default, skip_serializing_if = "is_default_when_to_convert")]
    pub when_to_convert: WhenToConvert,
}

/// Applying `MltEncoderConfig` overrides on top of `EncoderConfig` defaults.
///
/// Uses exhaustive destructuring of both structs so that adding a field
/// to either `MltEncoderConfig` or `EncoderConfig` causes a compile error
/// until this conversion is updated.
#[cfg(all(feature = "mlt", feature = "_tiles"))]
impl From<MltEncoderConfig> for EncoderConfig {
    fn from(src: MltEncoderConfig) -> Self {
        // Destructure both so new fields cause a compile error.
        let MltEncoderConfig {
            tessellate,
            try_spatial_morton_sort,
            try_spatial_hilbert_sort,
            try_id_sort,
            allow_fsst,
            allow_fpf,
            allow_shared_dict,
            // Processor-pipeline metadata, not part of `mlt-core`'s encoder.
            when_to_convert: _,
        } = src;

        Self {
            tessellate: tessellate.unwrap_or(Self::default().tessellate),
            try_spatial_morton_sort: try_spatial_morton_sort
                .unwrap_or(Self::default().try_spatial_morton_sort),
            try_spatial_hilbert_sort: try_spatial_hilbert_sort
                .unwrap_or(Self::default().try_spatial_hilbert_sort),
            try_id_sort: try_id_sort.unwrap_or(Self::default().try_id_sort),
            allow_fsst: allow_fsst.unwrap_or(Self::default().allow_fsst),
            allow_fpf: allow_fpf.unwrap_or(Self::default().allow_fpf),
            allow_shared_dict: allow_shared_dict.unwrap_or(Self::default().allow_shared_dict),
        }
    }
}

/// Resolve effective process config using full-override semantics:
/// per-source > source-type > global > default.
#[must_use]
pub fn resolve_process_config(
    global: &ProcessConfig,
    source_type: &ProcessConfig,
    per_source: &ProcessConfig,
) -> ProcessConfig {
    let default = ProcessConfig::default();
    if *per_source != default {
        per_source.clone()
    } else if *source_type != default {
        source_type.clone()
    } else {
        global.clone()
    }
}

#[cfg(test)]
mod tests {
    #[cfg(all(feature = "mlt", feature = "_tiles"))]
    use indoc::indoc;

    use super::*;

    #[cfg(all(feature = "mlt", feature = "_tiles"))]
    #[test]
    fn parse_mlt_auto_string() {
        let cfg: MltProcessConfig = serde_yaml::from_str("auto").unwrap();
        assert_eq!(cfg, MltProcessConfig::Auto);
    }

    #[cfg(all(feature = "mlt", feature = "_tiles"))]
    #[test]
    fn parse_mlt_explicit_empty() {
        let cfg: MltProcessConfig = serde_yaml::from_str("{}").unwrap();
        assert_eq!(cfg, MltProcessConfig::Explicit(MltEncoderConfig::default()));
    }

    #[cfg(all(feature = "mlt", feature = "_tiles"))]
    #[test]
    fn parse_mlt_when_to_convert_always() {
        let cfg: MltProcessConfig = serde_yaml::from_str(indoc! {"
            when_to_convert: always
        "})
        .unwrap();
        assert_eq!(
            cfg,
            MltProcessConfig::Explicit(MltEncoderConfig {
                when_to_convert: WhenToConvert::Always,
                ..Default::default()
            })
        );
    }

    /// Default `when_to_convert` is `match-accept` and must round-trip without
    /// being emitted into YAML (so the existing `auto`/explicit shapes stay
    /// concise on disk).
    #[cfg(all(feature = "mlt", feature = "_tiles"))]
    #[test]
    fn when_to_convert_default_skipped_in_serialization() {
        let cfg = MltProcessConfig::Explicit(MltEncoderConfig {
            tessellate: Some(true),
            ..Default::default()
        });
        let yaml = serde_yaml::to_string(&cfg).unwrap();
        assert!(
            !yaml.contains("when_to_convert"),
            "default when_to_convert leaked into YAML: {yaml}"
        );
    }

    #[cfg(all(feature = "mlt", feature = "_tiles"))]
    #[test]
    fn always_convert_to_mlt_helper() {
        let cfg = ProcessConfig {
            convert_to_mlt: Some(MltProcessConfig::Explicit(MltEncoderConfig {
                when_to_convert: WhenToConvert::Always,
                ..Default::default()
            })),
            convert_to_mvt: None,
        };
        assert!(cfg.always_convert_to_mlt());
        assert!(!cfg.always_convert_to_mvt());
    }

    #[cfg(all(feature = "mlt", feature = "_tiles"))]
    #[test]
    fn always_convert_to_mvt_helper() {
        let cfg = ProcessConfig {
            convert_to_mlt: None,
            convert_to_mvt: Some(MvtProcessConfig::Explicit(MvtEncoderConfig {
                when_to_convert: WhenToConvert::Always,
            })),
        };
        assert!(!cfg.always_convert_to_mlt());
        assert!(cfg.always_convert_to_mvt());
    }

    /// `auto` (no explicit settings) does not trigger Always.
    #[cfg(all(feature = "mlt", feature = "_tiles"))]
    #[test]
    fn always_helpers_are_false_for_auto() {
        let cfg = ProcessConfig {
            convert_to_mlt: Some(MltProcessConfig::Auto),
            convert_to_mvt: Some(MvtProcessConfig::Auto),
        };
        assert!(!cfg.always_convert_to_mlt());
        assert!(!cfg.always_convert_to_mvt());
    }

    #[cfg(all(feature = "mlt", feature = "_tiles"))]
    #[test]
    fn parse_mlt_explicit_with_overrides() {
        let cfg: MltProcessConfig = serde_yaml::from_str(indoc! {"
            tessellate: true
            allow_fsst: false
        "})
        .unwrap();
        assert_eq!(
            cfg,
            MltProcessConfig::Explicit(MltEncoderConfig {
                tessellate: Some(true),
                allow_fsst: Some(false),
                ..Default::default()
            })
        );
    }

    #[cfg(all(feature = "mlt", feature = "_tiles"))]
    #[test]
    fn serde_round_trip_auto() {
        let cfg = MltProcessConfig::Auto;
        let yaml = serde_yaml::to_string(&cfg).unwrap();
        insta::assert_snapshot!(yaml, @"auto");
        let parsed: MltProcessConfig = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(cfg, parsed);
    }

    #[cfg(all(feature = "mlt", feature = "_tiles"))]
    #[test]
    fn serde_round_trip_disabled() {
        let cfg = MltProcessConfig::Disabled;
        let yaml = serde_yaml::to_string(&cfg).unwrap();
        insta::assert_snapshot!(yaml, @"disabled");
        let parsed: MltProcessConfig = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(cfg, parsed);
    }

    #[cfg(all(feature = "mlt", feature = "_tiles"))]
    #[test]
    fn serde_round_trip_explicit() {
        let cfg = MltProcessConfig::Explicit(MltEncoderConfig {
            tessellate: Some(true),
            ..Default::default()
        });
        let yaml = serde_yaml::to_string(&cfg).unwrap();
        let parsed: MltProcessConfig = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(cfg, parsed);
    }

    #[cfg(all(feature = "mlt", feature = "_tiles"))]
    #[test]
    fn parse_mlt_invalid_string() {
        let result = serde_yaml::from_str::<MltProcessConfig>("invalid");
        assert!(result.is_err());
    }

    #[cfg(all(feature = "mlt", feature = "_tiles"))]
    #[test]
    fn parse_mlt_invalid_type() {
        let result = serde_yaml::from_str::<MltProcessConfig>("123");
        assert!(result.is_err());
    }

    #[cfg(all(feature = "mlt", feature = "_tiles"))]
    #[test]
    fn render_failure_mlt_unknown_string() {
        use crate::config::test_helpers::render_failure;
        insta::assert_snapshot!(render_failure(indoc! {"
                convert_to_mlt: atuo
            "}), @r#"
          × invalid value: string "atuo", expected a string ("auto", "enabled",
          │ "disabled"), a boolean, or a map of settings
           ╭─[config.yaml:1:1]
         1 │ convert_to_mlt: atuo
           · ───────┬──────
           ·        ╰── invalid value: string "atuo", expected a string ("auto", "enabled", "disabled"), a boolean, or a map of settings
           ╰────
        "#);
    }

    #[cfg(all(feature = "mlt", feature = "_tiles"))]
    #[test]
    fn render_failure_mlt_integer() {
        use crate::config::test_helpers::render_failure;
        insta::assert_snapshot!(render_failure(indoc! {"
                convert_to_mlt: 42
            "}), @r#"
          × invalid type: integer `42`, expected a string ("auto", "enabled",
          │ "disabled"), a boolean, or a map of settings
           ╭─[config.yaml:1:1]
         1 │ convert_to_mlt: 42
           · ───────┬──────
           ·        ╰── invalid type: integer `42`, expected a string ("auto", "enabled", "disabled"), a boolean, or a map of settings
           ╰────
        "#);
    }

    /// Inner-field errors must point at the *value*, not the outer `convert_to_mlt:` line —
    /// proves the explicit branch hands the saphyr deserializer to `MltEncoderConfig`
    /// instead of routing through a `serde_yaml::Value`.
    #[cfg(all(feature = "mlt", feature = "_tiles"))]
    #[test]
    fn render_failure_mlt_nested_field_bad_type() {
        use crate::config::test_helpers::render_failure;
        insta::assert_snapshot!(render_failure(indoc! {"
                convert_to_mlt:
                  tessellate: yes-please
            "}), @r"
          × invalid boolean
           ╭─[config.yaml:2:15]
         1 │ convert_to_mlt:
         2 │   tessellate: yes-please
           ·               ─────┬────
           ·                    ╰── invalid boolean
           ╰────
        ");
    }

    #[cfg(all(feature = "mlt", feature = "_tiles"))]
    #[test]
    fn resolve_per_source_disabled_overrides_global_auto() {
        let global = ProcessConfig {
            convert_to_mlt: Some(MltProcessConfig::Auto),
            convert_to_mvt: None,
        };
        let per_source = ProcessConfig {
            convert_to_mlt: Some(MltProcessConfig::Disabled),
            convert_to_mvt: None,
        };
        let resolved = resolve_process_config(&global, &ProcessConfig::default(), &per_source);
        assert_eq!(resolved.convert_to_mlt, Some(MltProcessConfig::Disabled));
    }

    #[cfg(all(feature = "mlt", feature = "_tiles"))]
    #[test]
    fn resolve_per_source_overrides_all() {
        let global = ProcessConfig {
            convert_to_mlt: Some(MltProcessConfig::Auto),
            convert_to_mvt: None,
        };
        let source_type = ProcessConfig {
            convert_to_mlt: None,
            convert_to_mvt: Some(MvtProcessConfig::Auto),
        };
        let per_source = ProcessConfig {
            convert_to_mlt: Some(MltProcessConfig::Explicit(MltEncoderConfig {
                tessellate: Some(true),
                ..Default::default()
            })),
            convert_to_mvt: None,
        };

        let resolved = resolve_process_config(&global, &source_type, &per_source);
        assert_eq!(resolved, per_source);
    }

    #[cfg(all(feature = "mlt", feature = "_tiles"))]
    #[test]
    fn resolve_source_type_overrides_global() {
        let global = ProcessConfig {
            convert_to_mlt: Some(MltProcessConfig::Auto),
            convert_to_mvt: None,
        };
        let source_type = ProcessConfig {
            convert_to_mlt: None,
            convert_to_mvt: Some(MvtProcessConfig::Auto),
        };

        let resolved = resolve_process_config(&global, &source_type, &ProcessConfig::default());
        assert_eq!(resolved, source_type);
    }

    #[cfg(all(feature = "mlt", feature = "_tiles"))]
    #[test]
    fn resolve_global_used_as_fallback() {
        let global = ProcessConfig {
            convert_to_mlt: Some(MltProcessConfig::Auto),
            convert_to_mvt: None,
        };

        let resolved = resolve_process_config(
            &global,
            &ProcessConfig::default(),
            &ProcessConfig::default(),
        );
        assert_eq!(resolved, global);
    }

    #[test]
    fn resolve_default_when_all_none() {
        let resolved = resolve_process_config(
            &ProcessConfig::default(),
            &ProcessConfig::default(),
            &ProcessConfig::default(),
        );
        assert_eq!(resolved, ProcessConfig::default());
    }

    /// Pins the schema shape to the wire format. With the `AutoOption` migration the
    /// schema includes string aliases for `auto`/`default`/`true`,
    /// `disabled`/`off`/`no`/`false`, a boolean shorthand, and the explicit
    /// `MltEncoderConfig` branch — four `oneOf` entries in total.
    #[cfg(all(feature = "mlt", feature = "unstable-schemas"))]
    #[test]
    fn json_schema_matches_serde_format() {
        let schema = serde_json::to_value(schemars::schema_for!(MltProcessConfig)).unwrap();
        let one_of = schema
            .get("oneOf")
            .and_then(|v| v.as_array())
            .expect("MltProcessConfig schema should be a `oneOf`");
        assert_eq!(one_of.len(), 4, "schema: {schema}");

        // The explicit branch should still reference MltEncoderConfig.
        let mut saw_encoder_ref = false;
        for entry in one_of {
            if let Some(reference) = entry.get("$ref").and_then(|v| v.as_str())
                && reference.ends_with("/MltEncoderConfig")
            {
                saw_encoder_ref = true;
            }
        }
        assert!(
            saw_encoder_ref,
            "expected $ref to MltEncoderConfig: {schema}"
        );
    }
}
