fn main() {
    let mut EpicAuthapp = EpicAuth::v1_2::EpicAuthApi::new(
        "example",
        "JjPMBVlIOd",
        "db40d586f4b189e04e5c18c3c94b7e72221be3f6551995adc05236948d1762bc",
        "1.0",
        "https://EpicAuth.cc/api/1.2/",
    );

    // None or Some(apphash)
    // we call unwrap() because if the response success = false, it returns an error with the error message
    EpicAuthapp.init(None).unwrap();

    // None will auto generate hwid, if you want to use your own hwid system then use Some(hwid)
    //auth.register("some_username".to_string(), "some_password_from_user".to_string(), "7F64WM-TP3I4H-6NY0QI-164KGY-WP5CHF-EBFG30".to_string(), None).unwrap();
    // again None will autogenerate hwid
    EpicAuthapp.login("some_username".to_string(), "some_password_from_user".to_string(), None).unwrap();

    // for more functions see the docs https://docs.rs/EpicAuth/latest/EpicAuth/
}
