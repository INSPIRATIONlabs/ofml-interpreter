//! Unit tests for OAP Configurator TUI

mod unit {
    mod oap_actions_test;
    mod oap_export_test;
    mod oap_price_test;
    mod oap_variant_test;
    mod ocd_features_test;
    mod ocd_snapshot_test;
    mod propvalue2varcond_test;
    mod table_relation_test;
    mod test_descriptions;
    mod vitra_snapshot_test;

    #[cfg(feature = "tui")]
    mod tui_data_validation_test;
    #[cfg(feature = "tui")]
    mod tui_form_test;
    #[cfg(feature = "tui")]
    mod tui_resize_test;
    #[cfg(feature = "tui")]
    mod tui_search_test;
    #[cfg(feature = "tui")]
    mod tui_snapshot_test;
    #[cfg(feature = "tui")]
    mod tui_state_test;
}
