extern crate osascript;
#[macro_use] extern crate serde_derive;

use osascript::JavaScript;

#[derive(Serialize)]
struct AlertParams {
    title: String,
    message: String,
    alert_type: String,
    buttons: Vec<String>,
}

#[derive(Deserialize)]
struct AlertResult {
    #[serde(rename="buttonReturned")]
    button: String,
}

fn main() {
    let script = JavaScript::new("
        var App = Application('Finder');
        App.includeStandardAdditions = true;
        return App.displayAlert($params.title, {
            message: $params.message,
            'as': $params.alert_type,
            buttons: $params.buttons,
        });
    ");

    let rv: AlertResult = script.execute_with_params(AlertParams {
        title: "Shit is on fire!".into(),
        message: "What is happening".into(),
        alert_type: "critical".into(),
        buttons: vec![
            "Show details".into(),
            "Ignore".into(),
        ]
    }).unwrap();

    println!("You clicked '{}'", rv.button);
}
