// tests/lib.rs

// Include each component test directly
#[path = "components/button/Button.test.rs"]
mod button_test;

#[path = "components/panel/Panel.test.rs"]
mod panel_test;

#[path = "components/card/Card.test.rs"]
mod card_test;

// Integration tests
mod integration {
    mod full_analysis;
}